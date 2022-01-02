# 屏蔽(barrier)

大多数现代计算机为了提高性能而采取乱序执行，这使得内存屏蔽成为必然。内存屏障(Memory barrier) 是一类同步屏蔽指令，它使得CPU或编译器在对内进行操作的时候，严格按照一定的顺序来执行，也就是说在内存屏蔽之前的指令和之后的指令不会由于系统优化等原因而导致乱序。

内存屏蔽之前的所有写操作都要写入内存；内存屏蔽之后的读操作都可以获得同步屏障之前的写操作结果。因此，对于敏感的程序块，写操作之后、读操作之前可以插入内存屏障。

&nbsp;

## rust 内存屏障

Rust中屏蔽的用法和互斥锁类似，它可以通过`wait`方法在某个点阻塞全部进入临界区的线程。

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");

            // 阻塞所有线程，当都执行到临界区，继续执行
            c.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
