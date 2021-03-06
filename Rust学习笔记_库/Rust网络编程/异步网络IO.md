# 异步网络IO

同步I/O模型可能是在给定时间内处理多个客户端的主要瓶颈，必须使用线程处理更多的客户端。为更好的方法来扩展，我们可以让套接字是非阻塞的，而不是应对套接字的阻塞性质。

对于非阻塞套接字，其上的任何读取、写入或者操作都会立刻返回，无论操作成功与否。没有客户端需要等待请求完成，而是稍后同志请求成功与否。与线程相比，异步I/O模型非常高效，但它增加了代码的复杂性。

在基于UNIX的平台上，套接字上的轮询机制是通过poll和select系统调用完成的。这些调用在所有UNIX操作系统上都是兼容的，除此之外，Linux还支持epoll API。在poll和select对每个请求的套接字运行for循环的情况下，epoll通过运行时 `O(1)`来同志用户的套接字事件。

&nbsp;

## Rust中的异步抽象

Rust提供了第三方软件包形式的便捷抽象，用于处理异步I/O。当处理非阻塞套接字和底层套接字轮询机制时，它为开发人员简化了大多数复杂状态机的处理。可供用户选择的两个底层抽象软件包是 `futures` 和 `mio`。

&nbsp;

### mio

mio提供了底层机制的高度抽象，他可以为大多数IO复用API提供跨平台、高效的接口。mio是一款底层软件包，它提供了一种为 socket 事件设置反应器的便捷方法。它和标准库类型相似，例如 `TcpStream`类型，不过默认情况下它是非阻塞的。

&nbsp;

### futures

mio 杂耍式的套接字轮询状态机用起来并不是很方便。为了提供可供应用程序开发人员使用的高级AP，提供了 future 软件包。

futures软件包提供了一个 `Future` 核心trait，这是该软件包的核心组成部分。

```rust
pub trait Future {
    // Future将解析的值
    type Item;

    // 操作失败时的错误类型
    type Error;

    // poll 指定了应该如何完成future过程
    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
}
```

&nbsp;

`Future` 值自身不能构建异步应用程序，你需要将某种反应器和事件循环来推进future完成。`poll` 函数指定了应该如何完成future过程。future也可以由几件事情组合而成，从而一个接一个地链接起来。为了推进future，我们需要一个反应器和事件循环实现，这是由tokio软件包提供。

&nbsp;

### tokio

tokio整合了上述两种抽象，以及工作窃取调度程序、事件循环和计时器实现，它提供了一个运行时来驱动future完成。通过tokio框架，你可以生成多个future并让它们同时运行。

tokio软件包在技术上是一种运行时，由一个线程池、事件循环，基于mio的I/O事件的反应器组成。

当future没有任何数据要解析，或者在 `TcpStream` 客户端读取正在等待到达套接字的数据时，它将返回NoReady状态。但是在执行此操作时，还需要向反应器注册感兴趣的内容，以便能够再次获知服务器的任何新数据。

当创建future时，无须执行任何其他操作。对于future定义的工作任务，必须提交给执行程序完成。在tokio中，任务是可以执行future的用户级线程。在poll方法的实现中，任务必须安排自己稍后执行轮询，以防相关工作停滞。为此，它必须将其任务处理程序传递给反应器线程。在Linux中，反应器是mio软件包。

&nbsp;

### 构建异步Redis服务器

略
