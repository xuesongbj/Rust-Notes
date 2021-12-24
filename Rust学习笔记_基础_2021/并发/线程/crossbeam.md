# Crossbeam
### 使用缓冲行填充提升并发性能

并发编程中，有一个**伪共享(False Sharing)** 概念。为了提升性能，现代CPU都有自己的多级缓存。而在缓存系统中，都是以缓存行(Cache Line)为基本单位进行存储的，长度为64字节(L1)。 当程序中的数据存储在彼此相邻的连续内存中时，可以被L1级缓存一次加载完成，享受缓存带来的性能极致。当数据结构中的数据存储在非连续内存中时，则会出现缓存未命中的情况。
Crossbeam是第三方库，实际开发中通常用来代替标准库。它是对标准库的扩展和包装，一共包含4大模块:

* **用于增强 `std::sync` 的原子类型**：类似C++ 11 风格的 Comsume内存顺序原子类型 `AtomicConsume` 和用于存储和检索 `Arc`的`ArcCell`。
* **对标准库 `thread` 和各种同步事件的扩展**：`Scoped`线程、支持缓存行填充的 `CachePadded`等。
* **MPMC的`channel`以及各种无锁并发数据结构**：并发工作窃取双端队列、并发无锁队列(`MS-Queue`)和无锁栈(`Treiber Stack`)。
* **并发数据结构需要的内存管理组件crossbeam-epoch**：多线程并发情况下，如果线程从并发数据结构中删除某个节点，但是该节点还有可能被其他线程使用，则无法立即销毁该节点。`Epoch GC` 允许推迟销毁，直到它变得安全。

&nbsp;

## 扩展的原子类型

Crossbeam 的 `crossbeam-utils` 提供了 `AtomicConsume trait`, 是对标准库中**原子类型内存顺序**的增强。

该 `trait` 允许原子类型以「Consume」内存顺序进行读取。「Consume」内存顺序是C++中支持的一种内存顺序，称为**消耗-释放顺序**。相对于**获取-释放顺序**，**消耗-释放顺序**的性能更好。因为获取-释放顺序会同步所有写操作之前的读操作，而消耗-释放顺序则只会同步数据之间有相互依赖的操作，粒度更细，性能更好。目前仅ARM和AArch64架构支持，在其他架构上还要回归到**获取-释放顺序**。

通过 `crossbeam-utils` 包，已经为标准库 `std::sync::atomic` 中的 `AtomicBool`、`AtomicUsize`等原子类型实现了该`trait`，只需要调用 `load_consume`方法就可以使用该内存顺序。在最新的 `crossbeam-utils` 包中，还增加了一个原子类型 `AtomicCell`，等价于一个具有原子操作的 `Cell<T>` 类型。

&nbsp;

## Scoped 线程

### 标准库spawn

目前Rust版本，在如下案例中，子线程已经可以安全使用父线程中的引用(默认引用方式进行遍历)。

```rust
fn main() {
    let array = [1, 2, 3];
    let mut guards = vec![];

    //    0x000055555555e153 <+195>:	lea    rdi,[rsp+0x70]
    // => 0x000055555555e158 <+200>:	call   0x555555560f50 <<core::array::iter::IntoIter<T,_> as core::iter::traits::iterator::Iterator>::next> 
    for i in array {
        let guard = std::thread::spawn(move || {
            println!("element: {}", i);
        });
        guards.push(guard);
    }

    for guard in guards {
        guard.join().unwrap();
    }
}
```

```x86asm
; 设置断点 & 运行
(gdb) b 6
(gdb) c

; 运行索引为1的闭包
(gdb) info threads
  Id   Target Id                              Frame
  1    Thread 0x7ffff7c3b240 (LWP 7273) "eee" clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:78
* 2    Thread 0x7ffff7c39700 (LWP 7277) "eee" eee::main::{{closure}} () at src/main.rs:6
  3    Thread 0x7fffeffff700 (LWP 7278) "eee" clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:78

; i ==> array[0]
(gdb) info locals
arg0 = 0x7ffff7c38bf4
i = 2

(gdb) x/xw 0x7ffff7c38bf4
0x7ffff7c38bf4: 0x00000001

; --------------------------------------------

; 设置断点 & 运行
(gdb) b eee::main:6
(gdb) c

; 运行索引为2的闭包
(gdb) info threads
  Id   Target Id                              Frame
  1    Thread 0x7ffff7c3b240 (LWP 7273) "eee" __pthread_clockjoin_ex (threadid=140737350178560, thread_return=0x0, clockid=<optimized out>, abstime=<optimized out>,
    block=<optimized out>) at pthread_join_common.c:145
  2    Thread 0x7ffff7c39700 (LWP 7277) "eee" 0x0000555555562f8c in eee::main::{{closure}} () at src/main.rs:6
* 3    Thread 0x7fffeffff700 (LWP 7278) "eee" 0x0000555555562f8c in eee::main::{{closure}} () at src/main.rs:6
  4    Thread 0x7ffff7a38700 (LWP 7279) "eee" 0x0000555555562f7d in eee::main::{{closure}} () at src/main.rs:6

; i ==> array[1]
(gdb) info locals
arg0 = 0x7fffefffebf4
i = 2

(gdb) x/xw 0x7fffefffebf4
0x7fffefffebf4: 0x00000002

; --------------------------------------------

; 设置断点 & 运行
(gdb) b eee::main:6
(gdb) c

; 运行索引为3的闭包
(gdb) info threads
  Id   Target Id                              Frame
  1    Thread 0x7ffff7c3b240 (LWP 7273) "eee" __pthread_clockjoin_ex (threadid=140737348077312, thread_return=0x0, clockid=<optimized out>, abstime=<optimized out>,
    block=<optimized out>) at pthread_join_common.c:145
* 4    Thread 0x7ffff7a38700 (LWP 7279) "eee" 0x0000555555562f8c in eee::main::{{closure}} () at src/main.rs:6

; i ==> array[2]
(gdb) info locals
arg0 = 0x7ffff7a37bf4
i = 3

(gdb) x/xw 0x7ffff7a37bf4
0x7ffff7a37bf4: 0x00000003
```

&nbsp;

### 使用Scoped线程

Crossbeam 提供了一种 Scoped线程，允许子线程可以安全地使用父线程中的引用。

```rust
use crossbeam::thread::scope;

fn main() {
    let array = [1, 2, 3];

    // 传入一个以scope为参数的闭包，在该闭包中由scope参数生成子线程，
    // 可以保证安全地使用main主线程中array数组元素的引用
    let _ = scope(|scope| {
        for i in array {
            scope.spawn(move |_| { println!("element: {}", i)});
        }
    });
}
```

闭包中的`scope`参数是一个内部使用的`Scope`结构体，该结构体会负责子线程的创建、`join` 父线程和析构等工作，以确保引用的安全。

&nbsp;

### 使用缓冲行填充提升并发性能

为了提升性能，现代CPU都有自己的多级缓存。而在缓存系统中，都是以缓存行(Cache Line)为基本单位进行存储的，长度为64字节(L1)。 当程序中的数据存储在彼此相邻的连续内存中时，可以被L1级缓存一次加载完成，享受缓存带来的性能极致。当数据结构中的数据存储在非连续内存中时，则会出现缓存未命中的情况。

&nbsp;

#### 伪共享(False Sharing)

并发编程中，有一个**伪共享(False Sharing)** 概念。将数据存储在连续紧凑的内存中虽然可以带来高性能，但是将置于多线程下就会发生问题。多线程操作同一个缓存行的不同字节，将会产生竞争，导致线程彼此牵连，互相影响，最终变成串行的程序，降低了并发性，这就是所谓的伪共享(False Sharing)。

为了避免伪共享，就需要将多线程之间的数据进行隔离，使得它们不在同一行缓存行，从而提升多线程的并发性能。

&nbsp;

#### 伪共享解决方案

避免伪共享的方案有很多，其中一种方案就是刻意增大元素间的间隔，使得不同线程的存取单位位于不同的缓存行。Crossbeam提供了`CachePadded<T>`类型，可以进行**缓存行填充(Padding)，**从而避免伪共享。

在Crossbeam提供的并发数据结构中就用到了缓存行填充。比如并发的工作窃取双端队列(crossbeam-deque)，就用到了缓存行填充来避免伪共享，提升并发性能。

&nbsp;

### MPMC Channel

Crossbeam提供了一个 `std::sync::mpsc` 的替代品 `MPMC Channel`，也就是多生产者多消费者通道。标准库 `mpsc` 中的 `Sender` 和 `Receiver` 都没有实现 `Sync`，但是 Crossbeam 提供的 `MPMC Channel` 的 `Sender` 和 `Receiver` 都实现了 `Sync`。

Crossbeam提供的MPMC Channel和标准库的Channel类似，也提供了无界通道和有界通道两种类型。

```rust
use crossbeam::channel as channel;

fn main() {
    // 使用unbounded函数创建无界通道。
    let (s, r) = channel::unbounded();
    let _ = crossbeam::scope(|scope| {
        scope.spawn(|_| {
            let _ = s.send(1);
            r.recv().unwrap();
        });

        scope.spawn(|_| {
            let _ = s.send(2);
            r.recv().unwrap();
        });
    });
}
```

Crossbeam中还提供了 `select!` 宏，用于方便地处理一组通道中的消息。
