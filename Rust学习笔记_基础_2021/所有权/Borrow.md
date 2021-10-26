# Borrow, BorrowMut, ToOwned

某些类型通过对数据类型的引用提供对底层的访问，该种类型属于 `borrow`类型，例如，可以将 `Box<T>` 借用为 `T`； 将 `String` 借用为 `str`。

Rust提供了类型转换为`borrow`的能力。

&nbsp;

## Borrow

`Borrow`提供了一个 `.borrow()` 方法，可以对具体数据类型转换为引用类型。`Borrow` 的前后类型之间要求必须内部等价性(比如: String和&str类型)，不具有这个等价性的两个类型之间，不能实现 `Borrow`。

`AsRef` 更通用，更普遍，覆盖类型更多，是 `Borrow`的超集。

```rust
use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn main() {
    let s = "Hello".to_string();
    check(s);

    let s = "Hello";
    check(s);
}
```

```x86asm
; let s = "Hello".to_string();
(gdb) ptype s
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}

; s = "Hello"
(gdb) ptype s
type = struct &str {
  data_ptr: *mut u8,
  length: usize,
}
```

&nbsp;

## BorrowMut

`BorrowMut` 提供了一个方法 `.borrow_mut()`， 它是 `Borrow<T>` 的可变(mutable) 引用版本。

一个类型为 `T` 的值 `foo`，如果 `T` 实现了 `BorrowMut<U>`，那么 `foo` 可执行 `.borrow_mut()` 操作，即 `foo.borrow_mut()`。操作的结果我们得到类型为 `&mut U` 的一个可变(mutable) 引用。

```rust
use std::borrow::BorrowMut;

fn check<T: BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}

fn main() {
    let v = vec![1, 2, 3];

    check(v);
}
```

&nbsp;

## ToOwned

`ToOwned` 特征针对可以转换为所有权版本的类型实现。例如, `&str` 类型为 `String` 实现了这个特征。这意味着 `&str` 类型有一个名为 `to_owned` 的函数，它可以将其转换为 `String` 类型，这是一种包含所有权的类型。

通常可使用 `Clone` 将某些类型从借用变为 `owned`。但是 `Clone` 仅适用于从 `&T` 到 `T`。 `ToOwned`包括了类型转换拥有的数据。

```rust
fn main() {
    let s: &str = "a";
    let ss: String = s.to_owned();

    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();

    println!("{:}, {:?}", ss, vv);
}
```

```x86asm
; let s: &str = "a";
(gdb) ptype s
type = struct &str {
  data_ptr: *mut u8,
  length: usize,
}

; let ss: String = s.to_owned(); 
(gdb) ptype ss
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}

; let v: &[i32] = &[1, 2]; 
(gdb) ptype v
type = struct &[i32] {
  data_ptr: *mut i32,
  length: usize,
}

; let vv: Vec<i32> = v.to_owned();
(gdb) ptype vv
type = struct alloc::vec::Vec<i32, alloc::alloc::Global> {
  buf: alloc::raw_vec::RawVec<i32, alloc::alloc::Global>,
  len: usize,
}
```
