# 类型转换

Rust提供了很多将给定类型的值转换为另一种类型的方法。

最简单的类型转换形式是类型转换表达式，可以使用二元运算符进行表达。例如, `println("{}", 1 + 1.0);` 不能编译，因为 `1` 是整数,而 `1.0` 是浮点数。但是，`println!("{}", 1 as f32 + 1.0);` 可以通过编译。详情查看: [using_as]()。

Rust还提供了类型转换特征。这些特征可以在 `convert` 模块下找到:

* `From` 和 `Into` 转换：[from_into]()
* `TryFrom` 和 `TryInto` 转换：[try_from_into]()
* `AsRef` 和 `AsMut` 转换：[as_ref_mut]()

此外，`std::str` 模块提供了一个 `FromStr` 特征，它有助于通过字符串 `parse` 方法将字符串转换为目标类型。 如果对给定的类型 `Person` 正确实现，那么让 `p: Person = "Mark,20".parse().unwrap()` 应该在编译器和运行时 不会出现panic。

这些应该是标准库中将数据转换为所需类型的主要方法。

&nbsp;

## 更多信息

更多信息请查阅标准库文档。

* [conversions](https://doc.rust-lang.org/std/convert/index.html)
* [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)