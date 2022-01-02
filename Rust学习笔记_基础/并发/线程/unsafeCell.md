# 内部可变性

Rust中提供的并发编程中，基本都支持内部可变性, 在行为上与 `Cell<T>`、`RefCell<T>` 比较相似。

```rust
// Mutex 源码实现

pub type MovableMutex = Box<Mutex>;

pub struct Mutex<T: ?Sized> {
    // 包装了用于底层操作系统API的sys::MovableMutex
    inner: sys::MovableMutex,

    // 标记该锁是否已经中毒(获取锁后，发生panic)
    poison: poison::Flag,

    // 锁包含的数据，使用 UnsafeCell
    data: UnsafeCell<T>,
}
```

内部可变性的结构体都是基于 `Unsafe<T>`实现的。`Unsafe<T>` 是一个泛型结构体，它属于**语言项(Lang Item)**，编译器会对它进行特殊处理。一般来讲，Rust中将不可变借用转换为可变借用**属于未定义行为**，编译器不允许开发者随意对这两种引用进行互相转换。但是，`UnsafeCell<T>` 是唯一的例外，这也是`UnsafeCell<T>`属于语言项的原因，**它属于Rust中将不可变转换为可变的唯一合法渠道，对于使用了 `UnsafeCell<T>` 的类型，编译器会关闭相关的检查**。

```rust
// UnsafeCell<T> 源码实现

#[lang = "unsafe_cell"]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}

impl<T: ?Sized> !Sync for UnsafeCell<T> {}

impl<T: ?Sized> UnsafeCell<T> {
    pub const fn get(&self) -> *mut T {
        self as *const UnsafeCell<T> as *const T as *mut T
    }
}
```
