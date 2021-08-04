# 字符串_String

String包含所有权类型，意味着保存String值的变量是其所有者。由于 String 是在堆上分配的，因此它可以被修改，并且能够在运行时根据需要增加长度。堆分配是一种开销相对昂贵的操作，`Vec` 分配内存时使该成本按使用量平摊而降低。

```rust
fn main() {
    let a: String = "Hello".to_string();
    let b = String::from("Hello");
    let c = "World".to_owned();
    let d = c.clone();

    println!("{}, {}, {}, {}", a, b, c, d);
}
```

```x86asm
(gdb) ptype a
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
(gdb) ptype b
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
(gdb) ptype c
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
(gdb) ptype d
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
```

```rust
// 数据结构

String               heap
+=======+           +=========//========+
| ptr   | --------> | u8 data ...       |
+-------+           +=========//========+
| cap   |
+-------+
| len   |
+=======+
```

&nbsp;

## String 标准库

String在标准库中还包含很多便捷方法：

* `String::new()`：分配一个空的String类型。
* `String::from(s: &str)`: 分配一个新的String类型，并通过字符串切片来填充它。
* `String::with_capacity(capacity: usize)`：预先分配一个预定义大小、空的String类型。
* `String::from_utf8(vec: Vec<u8>)`：从bytestring分配一个新的String类型。

```rust
fn main() {
    let mut empty_string = String::new();
    let empty_string_with_capacity = String::with_capacity(50);
    let string_from_bytestring: String = String::from_utf8(vec![82, 85, 83, 84]).expect("Creating String from bytestring failed");

    println!("Length of the empty string is {}", empty_string.len());
    println!("Length of the empty string with capacity is {}", empty_string_with_capacity.len());
    println!("Length of the string from a bytestring is {}", string_from_bytestring.len());

    println!("Bytestring says {}", string_from_bytestring);

    empty_string.push('1');
    println!("{}", empty_string);

    empty_string.push_str("2345");
    println!("{}", empty_string);

    println!("{}", empty_string.len());
}
```
