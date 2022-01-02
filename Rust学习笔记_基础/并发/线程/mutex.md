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

### 死锁案例

Rust虽然可以避免数据竞争，但不能避免其他问题，比如死锁，在日常Coding中还需要多家留意。

```rust
// 多线程模拟掷硬币场景，硬币有正面和反面，规定连续抛出正面10次为一轮。采用8个线程，每个线程模拟一轮掷硬币，然后分别统计每一轮掷硬币的总次数和8个线程的平均掷硬币次数。

use std::thread;
use std::sync::{Arc, Mutex};
use rand::random;

// total_flips: 掷硬币总数
// target_flips: 正面朝上的目标数
// continue_positive: 连续掷出正面的次数
// iter_counts: 掷硬币总次数
fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut continue_positive = 0;
    let mut iter_counts = 0;

    // 模拟掷硬币，直到continue_positive次数达到目标次数target_flips为止
    while continue_positive <= target_flips {
        iter_counts += 1;
        let pro_or_con = random();
        if pro_or_con {
            continue_positive += 1;
        } else {
            continue_positive = 0;
        }
    }

    println!("iter_counts: {}", iter_counts);
    let mut total_flips = total_flips.lock().unwrap();  // 掷完之后，对掷硬币总数进行统计
    *total_flips += iter_counts;
}
```

```rust
// completed： 记录掷硬币实验完成的线程总数

fn main() {
    // 使用 Mutex<T> 保护的是在多线程之间共享数据
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    let completed = completed.lock().unwrap();
    while *completed < runs {}
    let total_flips = total_flips.lock().unwrap();
    println!("Final average: {}", *total_flips / *completed);
}
```

以上代码会引起死锁，因为 `main` 主线程一直有对 `completed` 互斥锁，将会导致所有模拟掷硬币的子线程阻塞。子线程阻塞以后，就无法更新 `completed` 的值了。

&nbsp;

对以上产生死锁代码进行修改：

```rust
fn main() {
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    loop {
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / *completed);
            break;
        }
    }
}
```
