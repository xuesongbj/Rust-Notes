# `async`和`await!`

`async/await!` 是Rust语法的特殊部分，可以实现多执行任务的异步执行。

## async使用方法

* `async fn`
* `async block`
* `async closure`

以上三种使用方法，返回一个实现 `Future` 特征的值。

```rust
// 返回一个实现 `Future<Output=u8>` 的结构体
async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = await!(foo());
        x + 5
    }
}

fn bar() -> impl Future<Output = u8> {
    let closure = async |x: u8| {
        await!(bar() + x)
    };
    closure(5)
}
```

使用 `await!` 运行 Future，如果 `Future` 被阻塞, 它将让出当前线程。当再次获取CPU时间片时，执行者获取 `Future` 继续执行。

&nbsp;

## async 生命周期

`async fn` 与传统函数不同，传递引用参数或其它非 `'static` 参数时，返回一个受参数生命周期限制的 `Future`：

```rust
async fn foo(x: &u8) -> u8 {
    *x
}

fn foo<'a>(x: &'a u8) -> impl Future<Output = ()> + 'a {
    async {
        *x
    }
}
```

返回的 `Future` 立即被 `await!` 执行(非 `'static` 参数)，生命周期仍然有效。如果将 `Future` 发送到其它线程，可能存在生命周期问题。

```rust
use core::future::Future;

async fn foo(x: &u8) -> u8 {
    *x
}

fn good() -> impl Future<Output = ()> {
    async {
        let x = 5;
        let _ = foo(&x).await;
    } 
}

fn main() {
    let _ = good();
}
```

通过将参数移动到 `async` 块中，生命周期延长到与调用 `foo` 返回 `Future` 的生命周期相匹配。

&nbsp;

### async move

`async move` 块将获取它引用变量的所有权，生命周期和`Future`相绑定，但该变量所有权为该`Future`独享。

```rust
fn foo() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        ...
        println!{"{}", my_string};
    }
}
```

&nbsp;

### `await!` 多线程执行者

使用多线程 `Future` 执行者时， `Future` 可能在多线程之间移动，因此在 `async` 作用域内使用的任何变量都必须能够在线程之间传递，因为任何 `await!` 变量都可能导致切换到新线程。

非安全:

* `Rc, &RefCell`
* 没有实现 `Send` 特征的类型
* 引用参数未实现 `Sync` 特征的类型

在 `await!` 作用域内要使用 `futures::lock` 中的 `Mutex`，而不要使用 `std::sync`中的 `Mutex`，防止产生死锁问题。
