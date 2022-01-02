# 并发

Rust 的并发依赖于本机的操作系统线程，它在标准库中的 `std::thread` 模块中提供了线程API。

&nbsp;

## 访问线程中的数据

不与父线程交互的子线程是非常少见的。

```rust

use std::thread;

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    for n in 0..5 {
        thrad::spawn(|| {
            println!("{}", nums[n]);
        });
    }
}
```

&nbsp;

在上述代码中，`vec`中包含5个数字，然后生成5个线程，其中每个线程都会访问`vec`中的数据。

```bash
   Checking ddd v0.1.0 (/root/rs/ddd)
error[E0373]: closure may outlive the current function, but it borrows `nums`, which is owned by the current function
  --> src/main.rs:9:23
   |
9  |         thread::spawn(|| {
   |                       ^^ may outlive borrowed value `nums`
10 |             println!("{}", nums[n]);
   |                            ---- `nums` is borrowed here
   |
help: to force the closure to take ownership of `nums` (and any other referenced variables), use the `move` keyword
```

`nums`来自主线程，当我们生成一个线程时，它不能保证在父线程已经失效，它指向的`Vec`也会被释放。如果Rust允许前面的代码通过编译，那么子线程可能已经访问了主线程返回后包含一些垃圾值的`nums`，并且可能导致分段错误。

可以使用同步和互斥锁方解决该问题。

```rust
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let data = Arc::new(Mutex::new(nums));                          // 保证数据原子性
    let mut handlers = vec![];                                      // 存储子线程

    for n in 0..5 {
        let d = Arc::clone(&data);                                  // 克隆原子引用

        // 启动线程
        let h = thread::spawn(move || {                             // d 所有权转移
            let v = d.lock().unwrap();

            println!("{}", v[n]);
        });

        handlers.push(h);                                           // 收集子线程
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());           // 等待所有子线程结束
}
```

### 所有权转移

使用关键字 `move` 从父线程中将值移动到其子线程中。

```rust
use std::thread;

fn main() {
    let my_str = String::from("Damn you borrow checker!");
    let _ = thread::spawn(move || {
        println!("In thread: {}", my_str);
    });

    println!("In main: {}", my_str);
}
```

在 **父线程** 中再次访问 `my_str`，此操作将失败，并显示以下错误提示:


```bash
$> cargo c
    Checking ddd v0.1.0 (/root/rs/ddd)
error[E0382]: borrow of moved value: `my_str`
11 |     println!("In main: {}", my_str);
   |                             ^^^^^^ value borrowed here after move
```

使用 `move` 关键字，发生所有权转移。即使我们只是从子线程中读取`my_str`，该数据也不再有效。
