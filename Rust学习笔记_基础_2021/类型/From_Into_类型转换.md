# From 和 Into

实现将一种类型转换为另一种类型，我们可用 `From` 和 `Into` 特征，一个类型只需要实现 `From` 特征，就能自动获得 `Into` 特征的实现。在标准库中有无数 `From` 的实现，规定原生类型及其他常见类型的转换功能。

```rust
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```

&nbsp;

## str 和 String类型转换

```rust
fn main() {
    let s = "hello, rust!";
    let s1 = String::from(s);

    println!("{}", s1);
}
```

```x86asm
; let s = "hello, rust!";
(gdb) ptype s
type = struct &str {
  data_ptr: *mut u8,
  length: usize,
}

; let s1 = String::from(s);
(gdb) ptype s1
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
```

&nbsp;

## From

使用From对自定义类型转换。

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

```x86asm
(gdb) ptype num
type = struct ddd::Number {
  value: i32,
}
```

&nbsp;

## Into

使用 `Into` 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();

    println!("My number is {:?}", num);
}
```
