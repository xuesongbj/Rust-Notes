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

```rust
// library/alloc/src/sync.rs
unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}
```
