# yield(生成器)

如果要支持`async/await` 异步开发，最好是能有协程支持。所以，Rust的第一步是需要引进协程(Coroutine)。

## 协程

协程的实现分为两类：有栈协程(stackfull)、无栈协程(stackless)。

&nbsp;

### 有栈协程(stackfull)

有栈协程的实现，一般每个协程都自带独立的栈，功能强大，但是比较耗内存，性能不如无栈协程。

&nbsp;

### 无栈协程(stackless)

无栈协程的实现，一般是基于状态机(State Machine)，不实用独立的栈，具体的应用形式叫**生成器(Generator)**。这种形式的协程性能更好，但功能要弱于有栈协程，但也够用了。在Rust标准库支持的协程功能，就属于无栈协程。

&nbsp;

### 生成器

```rust
// cargo +nightly run

#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

fn main() {
    // 创建生成器
    // yield 专门为生成器引入的关键字
    let mut gen = || {
        yield 1;
        yield 2;
        yield 3;
        return 4
    };

    for _ in 0..4 {
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
    }
}
```
