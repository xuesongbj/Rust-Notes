/*
    cargo.toml:
        [dependencies]
        num_cpus = "1.8"
    
    // num_cpus: 可以识别当前运行的计算机中CPU的个数
*/

se std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// 

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Thunk<'a> = Box<dyn FnBox + Send + 'a>;

// 定义 ThreadPoolSharedData 结构体
struct ThreadPoolSharedData {
    // 标记线程的名称 
    name: Option<String>,
    
    // 接收端(rx)
    job_receiver: Mutex<Receiver<Thunk<'static>>>,          

    // 空锁 & 空的条件变量
    // 实现线程池的join方法，条件变量需要配合互斥锁才能使用
    empty_trigger: Mutex<()>,                               
    empty_condvar: Condvar,

    // 总队列数
    queued_count: AtomicUsize,

    // 正在执行的线程数
    active_count: AtomicUsize,
    
    // 线程迟允许的最大线程数
    max_thread_count: AtomicUsize,

    // 线程池发生panic数量
    panic_count: AtomicUsize,

    // 线程栈大小，若不设置，默认为8MB
    stack_size: Option<usize>,
}

impl ThreadPoolSharedData {
    // 条件满足，表示线程池处于工作状态
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0  || self.active_count.load(Ordering::SeqCst) > 0
    }

    fn no_work_notify_all(&self) {
        // 工作线程处于闲置状态，所有任务完成
        if !self.has_work() {
            // 拿到线程池锁 & 通知所有阻塞线程解除阻塞状态
            *self.empty_trigger.lock().expect("Unable to notify all joining threads");
            self.empty_condvar.notify_all();
        }
    }
}

pub struct ThreadPool {
    // 存储 Channel发送端(tx)
    jobs: Sender<Thunk<'static>>,

    // 记录工作线程共享的数据
    shared_data: Arc<ThreadPoolSharedData>,
}

impl ThreadPool {
    // 初始化线程
    pub fn new(num_threads: usize) -> ThreadPool {
        Builder::new().num_threads(num_threads).build()
    }

    // 将任务添加到Channel队列，
    // 使用AtomicUsize的fetch_add方法将queued_count累加一次
    pub fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static
    {
        self.shared_data.queued_count.fetch_add(1, Ordering::SeqCst);
        self.jobs.send(Box::new(job)).expect("unable to send job into queue.");
    }

    // 阻塞主线程，等待线程池中所有任务执行完成
    pub fn join(&self) {
        // 线程池若处于闲置状态，则提前返回
        if self.shared_data.has_work() == false {
            return ();
        }

        // 获得互斥锁
        let mut lock = self.shared_data.empty_trigger.lock().unwrap();

        // 如果线程池中的工作线程一直处于正常工作状态，则调用empty_condvar的wait方法阻塞当前线程
        // 直到获得解除阻塞的通知(notify_all)
        while self.shared_data.has_work() {
            lock = self.shared_data.empty_condvar.wait(lock).unwrap();
        }
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    // 工作线程
    num_threads: Option<usize>,

    // 线程名称
    thread_name: Option<String>,

    // 线程栈大小
    thread_stack_size: Option<usize>,
}

impl Builder {
    // 生成一个字段初始化均为None的Builder实例
    pub fn new() -> Builder {
        Builder {
            num_threads: None,
            thread_name: None,
            thread_stack_size: None,
        }
    }

    // 设置工作线程数
    pub fn num_threads(mut self, num_threads: usize) -> Builder {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }

    // 初始化线程池
    pub fn build(self) -> ThreadPool {
        // 创建一个无界队列
        let (tx, rx) = channel::<Thunk<'static>>();

        // 获取线程数量，若没设置，则通过num_cpus获取当前cpu核心数，作为工作线程数
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);

        // 初始化ThreadPoolSharedData实例
        let shared_data = Arc::new(ThreadPoolSharedData{
            name: self.thread_name,
            job_receiver: Mutex::new(rx),
            empty_condvar: Condvar::new(),
            empty_trigger: Mutex::new(()),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            panic_count: AtomicUsize::new(0),
            stack_size: self.thread_stack_size,
        });

        for _ in 0..num_threads {
            // 生成工作线程
            spawn_in_pool(shared_data.clone());
        }

        // 初始化完成ThreadPool实例
        ThreadPool {
            jobs: tx,
            shared_data: shared_data,
        }
    }
}

fn spawn_in_pool(shared_data: Arc<ThreadPoolSharedData>) {
    // 设置thread.name & thead.stack_size
    let mut builder = thread::Builder::new();
    if let Some(ref name) = shared_data.name {
        builder = builder.name(name.clone());
    }

    if let Some(ref stack_size) = shared_data.stack_size {
        builder = builder.stack_size(stack_size.to_owned());
    }

    // 创建工作线程
    builder.spawn(move || {
        let sentinel = Sentinel::new(&shared_data);
        // loop阻塞当前工作线程从任务队列中取具体的任务来执行
        loop {
            // 获取当前队列中的active_count
            let thread_counter_val = shared_data.active_count.load(Ordering::Acquire);

            // 获取max_thread_count数目
            let max_thread_count_val = shared_data.max_thread_count.load(Ordering::Relaxed);

            // 如果工作线程数大于最大线程数，退出
            if thread_counter_val >= max_thread_count_val {
                break;
            }

            // 获取具体的工作任务
            let message = {
                // 先得到job_receiver锁，然后调用recv方法从队列中获取任务
                // 此时并未执行
                let lock = shared_data.job_receiver.lock().expect("unable to lock job_receiver");
                lock.recv()
            };

            // 从message获取具体的闭包任务
            let job = match message {
                Ok(job) => job,
                Err(..) => break,
            };

            shared_data.queued_count.fetch_sub(1, Ordering::SeqCst);    // 任务数减1
            shared_data.active_count.fetch_add(1, Ordering::SeqCst);    // 现在需要执行任务，所以活跃线程加1
            
            // 执行具体任务
            job.call_box();

            // 活跃线程数减1
            shared_data.active_count.fetch_sub(1, Ordering::SeqCst);

            // 通知条件变量wait方法，解除线程阻塞
            shared_data.no_work_notify_all();
        }

        // 使用cancel方法设置sentinel实例的状态
        // 表示该线程正常执行完所有任务
        sentinel.cancel();
    }).unwrap();
}

// 该结构体用于监控当前工作线程的状态
struct Sentinel<'a> {
    // 包装线程池共享数据
    shared_data: &'a Arc<ThreadPoolSharedData>,

    // 当前线程状态
    // true: 工作线程正在工作
    // false: 当前工作线程正常执行完毕
    active: bool,
}

impl<'a> Sentinel<'a> {

    // 设置工作线程正在执行
    fn new(shared_data: &'a Arc<ThreadPoolSharedData>) -> Sentinel<'a> {
        Sentinel {
            shared_data: shared_data,
            active: true,
        }
    }

    // 线程正常执行完毕
    fn cancel(mut self) {
        self.active = false;
    }
}

// 当工作线程中的Sentinel<'a>实例离开作用域时会调用析构函数drop
impl<'a> Drop for Sentinel<'a> {
    fn drop(&mut self) {
        // 如果条件成立，线程未执行完成，线程并未正常退出
        if self.active {
            // 工作线程归还到线程池
            self.shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
            
            // 判断当前线程是否出现Panic，而退出
            // 如果是，则panic_count 加1
            if thread::panicking() {
                self.shared_data.panic_count.fetch_add(1, Ordering::SeqCst);
            }

            // 通知线程，进行解除阻塞
            self.shared_data.no_work_notify_all();

            // 生成工作线程
            spawn_in_pool(self.shared_data.clone())
        }
    }
}

fn main() {
    // 创建8个工作线程
    let pool = ThreadPool::new(8);

    // 创建一个原子类变量test_count，用于计数测试
    let test_count = Arc::new(AtomicUsize::new(0));
    for _ in 0..42 {
        let test_count = test_count.clone();

        // 使用pool.execute将test_count加1任务放到线程池中进行计算
        pool.execute(move || {
            test_count.fetch_add(1, Ordering::Relaxed);
        });
    }

    // 阻塞main线程，等待线程池中的任务执行完成
    pool.join();

    // 判断执行结果是否OK!!!
    assert_eq!(42, test_count.load(Ordering::Relaxed));
}