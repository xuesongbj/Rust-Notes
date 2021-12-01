# 互斥锁(Mutex)

`Mutex<T>` 是Rust实现的互斥锁,用于保护共享数据。如果类型 `T` 实现了 `Send`，那么 `Mutex<T>` 会自动实现 `Send` 和 `Sync`。在互斥锁的保护下，每次只能有一个线程有权访问数据，但在访问数据之前，必须通过调用 `lock` 方法阻塞当前线程，直到得到互斥锁，才能得到访问权限。

`Mutex<T>` 类型实现的 `lock` 方法会返回一个 `LockResult<MutexGuard<T>>` 类型，`LockResult<T>` 是 `std::sync` 模块中定义的错误类型， `MutexGuard<T>` 基于 RAII机制实现，只要超出作用域范围就会自动释放锁。另外，`Mutex<T>` 也实现了 `try_lock` 方法，该方法在获取锁的时候不会阻塞当前线程，如果得到锁，就返回 `MutexGuard<T>`；如果得不到锁，就返回 `Err`。

> RAII: Resource Acquisition Is Initialization的缩写，意为“资源获取即初始化”。  
> 
> 它是C++之父Bjarne Stroustrup提出的设计理念，其核心是把资源和对象的生命周期绑定，对象创建获取资源，对象销毁释放资源。

&nbsp;

### Mutex 使用案例

针对 `sync_send` 章节多线程的错误，可以使用支持跨线程安全共享可变变量的容器即可，可以使用Rust提供的`Mutex<T>` 类型。

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let s = Arc::new(Mutex::new("Hello".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" world!");
        });
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }
}
```

&nbsp;

## 跨线程panic和错误处理

当子线程发生错误时，因为Rust基于返回值的错误处理机制，也让跨线程错误处理变得非常方便。`std::thread::JoinHandle` 实现的 `Join` 方法会返回 `Result<T>`，当子线程内部发生panic时，该方法会返回`Err`，但通常不会对此类 `Err` 进行处理，而是直接使用`unwarp` 方法，如果获取到合法的结果，则正常使用；如果是 `Err`，则故意让父线程发生panic，这就可以把子线程的panic传播给父线程，及早发现问题。

但是如果线程在获得锁之后发生panic，则称这种情况为「中毒(Poison)」。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(1));
    let c_mutex = mutex.clone();
    let _ = thread::spawn(move || {
        let mut data = c_mutex.lock().unwrap();
        *data = 2;
        panic!("oh no");
    }).join();
    assert_eq!(mutex.is_poisoned(), true);

    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(p_err) => {
            let data = p_err.get_ref();
            println!("recovered: {}", data);
        }
    };
}
```

使用 `is_poisoned` 方法来检查获得的互斥锁的子线程是否发生panic，因为子线程发生panic，代码调用 `lock` 方法机会返回 `Err`, 这里直接处理了 `Err` 的情况。该 `Err` 是 `PoisonError<T>` 类型，提供了 `get_ref` 或 `get_mut` 方法，可以得到内部包装的 `T`类型。
