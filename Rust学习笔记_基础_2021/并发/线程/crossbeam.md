# Crossbeam

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

闭包中的`scope`参数是一个内部使用的`Scope`结构体，该结构体会负责子线程的创建、`join`父线程和析构等工作，以确保引用的安全。
