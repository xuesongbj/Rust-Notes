# Waker

`futures` 在第一次被 `poll` 时无法完成任务是很常见的。遇到这种情况，`futures` 需要将任务执行的现场保护起来，等待下次唤醒(通过 `Waker` 进行唤醒)，继续执行任务。

`Waker` 提供了一种 `wake()` 方法，用于唤醒休眠中的任务。

`Waker` 还实现了 `clone()`， 以便复制和存储任务。

&nbsp;

## 构建定时器

```rust
use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

pub struct TimerFuture {
    // `Arc<Mutex<..>>` 共享值在线程和 `Future` 之间进行通信
    shared_state: Arc<Mutex<SharedState>>,
}

// `future`与等待线程之间的共享状态
struct SharedState {
    // 判断任务是否就绪
    completed: bool,
    
    // 任务唤醒者
    waker: Option<Waker>,
}
```

&nbsp;

### `Future`具体实现

```rust
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>， cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Pin：保证mut Self不会发生move!!!

        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // 任务执行完成
            Poll::Ready(())
        } else {
            // 设置唤醒器, 以便线程在计时器(timer)过期的时候可以唤醒当前任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```

如果线程设置 `shared_state.completed = true`， 任务执行完成；否则为当前任务克隆 `LocalWaker`，将其转换为 `Waker`，然后传递给 `shared_state.waker`，等待当前任务被唤醒。

在每次轮训(`Poll`)时更新 `Waker`，因为 `Future` 可能已经转移到另一个不同的任务与 `Waker`。`Future` 被轮询之后在任务间传递时会发生这种情况。

&nbsp;

### 构造计时器并启动线程

```rust
impl TimerFuture {
    // 创建一个新的 `TimerFuture`，在timeout之后，任务完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));

        // 创建一个新的线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            // 唤醒一个future任务
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
```

&nbsp;

### async/await 实例

```rust
#![feature(arbitrary_self_types, futures_api)]
#![feature(async_await, pin)]

use futures::executor::ThreadPool;

use std::future::Future;
use std::pin::Pin;
use std::task::*;

pub struct AlmostReady {
    ready: bool,
    value: i32,
}

pub fn almost_ready(value: i32) -> AlmostReady {
    AlmostReady{ ready: false, value }
}

impl Future for AlmostReady {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value + 1)
        } else {
            unsafe { Pin::get_unchecked_mut(self).ready = true; }
            lw.wake();
            Poll::Pending
        }
    }
}

fn main() {
    let mut executor = ThreadPool::new().unwrap();
    let future = async {
        println!("howdy!");
        let x = almost_ready(5).await;
        println!("done: {:?}", x);
    };

    executor.spawn_ok(future);
}
```
