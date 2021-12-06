# 条件变量

**条件变量** 跟屏障有点儿相似，但它不是阻塞全部线程，而是在满足指定条件之前阻塞某一个得到互斥锁的线程。

```rust
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

fn main() {
    // 互斥锁和条件变量，声明 Arc<Mutex<bool>, Condvar> 类型的变量
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    thread::spawn(move || {
        // 得到互斥锁lock，通过lock方法获得互斥锁
        let &(ref lock, ref cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();

        println!("children thread started. {}", started);
        let three_seconds = time::Duration::from_secs(3);
        thread::sleep(three_seconds);

        // 修改其中包含的 bool 类型数据为true
        // 修改之后，通过notify_one 方法通知主线程: 通知主线程，第33行 
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    while !*started {
        println!("{}", started);

        // wait方法，阻塞当前main主线程，直到子线程中 started 互斥体中的条件为真
        started = cvar.wait(started).unwrap();
        println!("{}", started);
    }
}
```

**在运行中每个条件变量只能和一个互斥体一起使用。** 在有些线程需要获取某个状态成立的情况下，如果单独使用互斥锁会比较浪费系统资源，因为只有多次出入临界区才能获取到某个状态的信息。此时就可以配合使用条件变量，当状态成立时通知互斥体就可以，因此减少了系统资源的浪费。
