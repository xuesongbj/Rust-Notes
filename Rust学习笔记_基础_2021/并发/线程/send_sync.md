# send 和 sync

在Rust中使用 `std::marker::Send` 和 `std::marker::Sync` 两个特殊的 `trait` 实现线程之间的安全。这两个标记 `trait` 反映了 Rust 看待线程安全的哲学： **多线程共享内存并非线程不安全问题所在，问题在于错误地共享数据。**

* **实现了 `Send` 的类型，可以安全地在线程间传递所有权**。也就是说，可以跨线程移动。
* **实现了 `Sync` 的类型，可以安全地在线程间传递不可变借用**。也就是说，可以跨线程共享。

通过 `Send` 和 `Sync` 将类型(`struct`)贴上标签，由编译器来识别这些类型是否可以在多线程之间移动或共享，从而做到在编译期就能发现线程不安全的问题。和 `Send`、`Sync` 相反的标记是 `!Send`、`!Sync`，表示不能在线程之间安全传递的类型。

```rust
// sync、Send 函数源码实现

// library/std/src/thread/mod.rs
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

`spawn` 函数中的闭包 `F` 与闭包的返回类型 `T` 都被加上了 `Send` 和 `'static` 限定。`Send` 限定了闭包的类型以及闭包的返回值都必须是实现了 `Send` 的类型，只有实现了 `Send` 的类型才可以在线程间传递。

闭包的类型是和捕获变量相关的，如果捕获变量的类型实现了 `Send`，那么闭包就实现了 `Send`。捕获变量是`Rc<String>`类型，实现的是 `!Send`，不能在县城间进行安全移动。

`'static` 限定表示类型 `T` 只能是**非引用类型**。闭包在线程间传递，如果直接携带引用类型，生命周期无法保证，很容易出现悬垂指针，造成内存不安全。

&nbsp;

如何才能在多个线程之间安全地共享变量呐？如果是不可变变量，可以通过 `Arc<T>` 进行共享(`Arc<T>`内部使用了原子操作，所以默认线程安全)。

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let s = Arc::new("hello");
    for _ in 0..3 {
        let c = Arc::clone(&s);
        thread::spawn(move || {
            println!("{}", c);
        });
    }
}
```

```rust
// library/alloc/src/sync.rs

// 只要T实现了Send和Sync的类型，那么Arc<T>也会实现Send和Sync。
unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}
```

`Send` 和 `Sync` 这两个`trait`是`unsafe`的，如果开发者自定义类型手动实现这两个`trait`,编译器不保证线程安全的。在Rust标准库 `std::marker` 模块中，所有类型默认实现了 `Send` 和 `Sync`。

```rust
// 所有类型实现了Send和Sync。Send和Sync只是标记trait，没有任何默认的方法。如果想使用这种方式，需要满足以下两个条件:
// 1. impl 和 trait 必须在同一个模块中
// 2. 在该trait内部不能有任何方法
unsafe impl Send for .. {}
unsafe impl Sync for .. {}

// *const T 和 *mut T 类型实现了 !Send和!Sync，这两种trait的类型不能在线程间安全传递
impl<T: ?Sized> !Send for *const T {}
impl<T: ?Sized> !Send for *mut T {}
impl<T: ?Sized> !Sync for *const T {}
impl<T: ?Sized> !Sync for *mut T {}

// &'a T 和 &'a mut T实现了Send
// &'a T要求T必须实现了Sync类型，表示只要实现了Sync类型，不可变借用就可以安全地线程间共享；
// &'a mut T 要求T必须实现了Send的类型，表示只要实现了Send类型，可变借用就可以安全地在线程间移动.
mod impls {
    unsafe impl<'a, T: Sync + ?Sized> Send for &'a T {}
    unsafe impl<'a, T: Send + ?Sized> Send for &'a mut T {}
}
```

除了以上在 `std::marker` 模块中标记的未实现 `Send` 和 `Sync`类型外，其他模块中也有。比如: `Cell`和`RefCell`都实现了 `!Sync`,无法跨线程共享；`Rc` 实现了 `!Send`，无法跨线程移动。

&nbsp;

如下实例，**在线程间传递可变字符串**。如下代码存在数据竞争隐患。使用了所有权已经被启动的值 `s`，违反了Rust所有权机制。

```rust
use std::thread;

fn main() {
    let s = "hello".to_string();
    for _ in 0..3 {
        thread::spawn(move || {
            s.push_str(" Rust");                //  use of moved value: `s`       
        });
    }
}
```

&nbsp;

尝试使用`Rc`共享所有权。`spawn` 函数传入的闭包没有实现 `Send`, 这是因为捕获变量没有实现 `Send`。捕获变量是`Rc<String>`类型，实现的是 `!Send`，不能在县城间进行安全移动。

```rust
use std::thread;
use std::rc::Rc;

fn main() {
    let s = Rc::new("hello".to_string());
    for _ in 0..3 {
        let mut s_clone = s.clone();
        thread::spawn(move || {                 // `Rc<String>` cannot be sent between threads safely
            s_clone.push_str(" Rust");
        });
    }
}
```

&nbsp;

尝试使用`Arc`共享所有权。

```rust
use std::thread;
use std::sync::Arc;
use std::cell::RefCell;

fn main() {
    // Arc<T> 默认不可变的，如果像使用Arc完成效果，需要具备内部可变形类型, RefCell
    let s = Arc::new(RefCell::new("hello".to_string()));
    for _ in 0..3 {
        let s_clone = s.clone();
        thread::spawn(move || {                         // ^^^^^^^^^^^^^ `RefCell<String>` cannot be shared between threads safely
            let s_clone = s_clone.borrow_mut();
            s_clone.push_str(" Rust");
        });
    }
}
```

`Cell` 和 `RefCell` 均是线程不安全的容器类型，它们实现了 `!Sync`，无法跨线程共享。
