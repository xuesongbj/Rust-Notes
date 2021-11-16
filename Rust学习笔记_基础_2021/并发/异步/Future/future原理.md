# Future运行原理

* Future 具体特征:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

&nbsp;

## Future 执行流程

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

### Future实例 

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

&nbsp;

## Future 参数

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

很长一段时间内，`Future`的唯一参数是 `&Waker`类型。在稳定版本(`stablize Future`)前夕，考虑到将来可能的扩展需求，withoutboats 提出将 `Waker`包装在 `Context` 类型内，这样将来需要扩展 `Future` 时可以避免引入breakchange(中断变更)。目前阶段内，`std::task::Context` 类型基本等价于`std::task::Waker` 类型。

```rust
// std::task::Context

// 目前，`Context` 仅用于提供对 `&Waker`的访问， 该 `&Waker`可用于唤醒当前任务.
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct Context<'a> {
    waker: &'a Waker,
    
    // 保证生命周期不可变
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```
&nbsp;

> break changes:
>
> break changes 指明是否产生了破坏性修改，涉及break changes的改动必须指明该项，类似版本升级、接口参数减少、接口删除、迁移等。
>

&nbsp;

`Waker` 虽然是标准库实现的类型，但 `Executor` 可以通过为 `Waker` 内的 `vtable`和原始指针赋值进而指定该`Waker`的功能(宁愿暴露`vtable`也不用`Trait`约束行为，主要因为`Object Safety`相关的阻碍)。

```rust
// `Waker`是一个句柄，用于通过通知执行器，它已经准备好运行，可以唤醒任务了
// 这个句柄封装了一个 `RawWaker` 实例, 它定义了特定用于执行器的唤醒行为.

/// Implements [`Clone`], [`Send`], and [`Sync`].
#[repr(transparent)]
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct Waker {
    waker: RawWaker,
}
```

```rust
// 1. `RawWaker` 允许任务执行器的实现者，创建一个 [`Waker`]来提供定制的唤醒行为
// 2. 他由一个数据指针和一个虚拟函数指针表(vtable)[vtable]组成，用于自定义 `RawWaker`的行为
// 3. [vtable]: https://en.wikipedia.org/wiki/Virtual_method_table
#[derive(PartialEq, Debug)]
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct RawWaker {
    // 1. 一个数据指针，可以用来存储执行器需要的任意数据.
    // 2. 该字段的值作为第一个参数传递给属于vtable的所有函数.
    data: *const (),
   
    // 自定义唤醒器行为
    vtable: &'static RawWakerVTable,
}
```

&nbsp;

对于 `Future` 的实现者来说，可以通过 `Waker` 通知 `Executor`，其所在的`Task`目前已经处于就绪状态，可以再次被调用 `poll`方法。

```rust
// 1. 指定 `RawWaker`行为的虚拟函数指针表(vtable)
// 2. 传递给vtable内所有函数的指针是来自 `RawWaker`对象的`data`指针
// 3. 该结构体函数仅允许`RawWaker`里的`data`指针上调用

#[stable(feature = "futures_api", since = "1.36.0")]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RawWakerVTable {
    // 当 `RawWaker` 被克隆时，此函数被调用
    // Eg: `RawWaker`的 `waker`被克隆
    clone: unsafe fn(*const ()) -> RawWaker,

    
    // 在Waker上调用wake时，将调用此函数
    // 它必须唤醒与RawWaker关联的任务
    // 必须确保释放与RawWaker和相关任务的此实例关联任何资源
    wake: unsafe fn(*const ()),

    // 在Waker上调用wake_by_ref时，将调用此函数
    // 它必须唤醒与rawWaker关联的任务
    // 类似于wake，但是引用，不会对数据指针有影响
    wake_by_ref: unsafe fn(*const ()),

    // RawWaker被删除时，该函数被调用
    // 必须确保释放与RawWaker和相关任务的此实例关联的任何资源
    drop: unsafe fn(*const ()),
}
```
