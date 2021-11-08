# Future trait

`Future` 特征是Rust异步编程的核心。`Future`可以产生异步计算的值，一个简化的 `Future` 特征:

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

&nbsp;

## poll

`poll` 等待文件描述符的事件，当有新的事件，则 `wake()` 被调用，则 `executor` 触发 `Future`将调用 `poll`。如果 `Future` 完成，它将返回 `Poll::Ready(result)`；如果 `Future` 未完成，它将返回 `Poll::Pending`，等待下轮 `wake()`触发时候，被调用。

&nbsp;

### 简单 `SocketRead` 场景

```rust
struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Outpue = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // 套接字拥有数据就读取数据到缓冲区并返回数据.
            Poll::Ready(self.socket.read_buf())
        } else {
            // 套接字没有数据：
            // 1. 安排 `wake` 在有数据后再次调用.
            // 2. 当有数据的时候， `wake`会被调用.
            // 3. 并且这个 `Future` 的用户将知道再一次调用 `poll` 接收数据.
            self.socket.set_reaable_callback(wake);
            Poll::Pending
        }
    }
}
```

&nbsp;

### 多个异步操作

`Futures` 特征允许将多个异步操作组合在一起，而无需中间分配。一次运行多个 `Future` 或将 `Future` 链接在一起，可以通过无分配状态机实现。

```rust
pub struct Join<FutureA, FutureB> {
    // 每个字段都可能包含运行已完成的 futures.
    // 如果 `future` 已经完成，则该字段设置为 `None`，
    // 这样可以防止持续轮询(poll) future.
    // 如果这样做，就违反了 `Future` trait的契约.

    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut slef, wake: fn()) -> Poll<Self::Output> {
        // 尝试完成 future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Read(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // 尝试完成 future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Read(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 所有的futures都完成了, 返回成功
            Poll::Ready(())
        } else {
            // 一个或者全部 futures 返回 `Poll::Pending`，说明仍然有工作要去做
            // 将再次调用 `wake`
            Poll::Pending
        }
    }
}
```

&nbsp;

如何在不需要单独分配的情况下同时运行多个 `Future`，从而允许更高效的异步程序.

```rust
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 我们完成了第一个future，之后轮询它并完成第二个
                Poll::Ready(()) => self.first.take(),

                // 我们还不能完成第一个 future.
                Poll::Pending => return Poll::Pending,
            };
        }
        // 现在第一个future已经完成，尝试完成第二个
        self.second.poll(wake)
    }
}
```
