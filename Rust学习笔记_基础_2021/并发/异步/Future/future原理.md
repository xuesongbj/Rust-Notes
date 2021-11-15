# future运行原理

Rust中一个`Future`被执行常见的流程:

* 通过手动展开或组合的形式，创建一个顶层`Future`实例，其内部直接或间接调用了很多`Future`(此时，Future还没有被执行)。
* 用户将该`Future` 丢进一个标准库外实现的`Executor`。
* `Executor`创建一些环境信息并与该 `Future`实例绑定(我们将环境信息和Future实例整体称为Task)。
* 立即调用一次该 `Future`的`poll`方法，很快该方法执行完成，返回 `Poll::Pending`。
* `Executor`将该task放入等待队列。
* 某个时间点，该task被唤醒，将该task放入就绪队列。
* 按顺序取出就绪队列中的task，依次调用每个task对应`Future`的`Poll`方法，若仍然返回 `Poll::Pending`,返回步骤5,若返回`Poll::Ready(XXX)`，则将该结果返回给最终用户。

`Future`本身并没有直接以符合异步语义的方式执行，它要求第三方实现的`Executor`能够在合适的时间调用 `poll`方法。而`Executor`获知这个「合适的时间」的关键便在于 `poll` 方法的`&mut Context` 参数。

&nbsp;

## Future实例 

```rust
use futures::channel::mpsc;
use futures::executor; //standard executors to provide a context for futures and streams
use futures::executor::ThreadPool;
use futures::StreamExt;

fn main() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();
    
    // 通过 async 创建Future, 其中 async 负责Future实现。暂时还没有为Future提供执行器, 所以它不会运行。
    let fut_values = async {
        // 创建另一个 async, 同时是由async异步生成Future实现的地方。
        // 由于是在父 async内部，因此在执行时将提供父块的executor。 
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };

       
        // 通过线程池生成Future，对Future进行发送，生产者
        pool.spawn_ok(fut_tx_result);

        // 消费者，
        let fut_values = rx.map(|v| v * 2).collect();

        // 等待Future执行完成
        fut_values.await
    };

    // 实际上述Future，调用 `Future::poll`并随后链接到适当的 `Future::poll`和需要执行者驱动所有 `Futures`的方法。
    // 最终 `fut_values` 将被驱动完成。
    let values: Vec<i32> = executor::block_on(fut_values);

    println!("Values={:?}", values);
}
```
