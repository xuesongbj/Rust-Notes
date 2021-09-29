# Traits

特征是方法的集合。

数据类型(`struct`) 可以实现特征(`trait`)。为此，为数据类型定义了构成特征的方法。例如，`String`类型实现了 `From<&str> trait`，允许用户编写 `String::from("hello")`

一些额外的常见Rust特征包括:

* `Clone`：`clone`方法。
* `Display`：允许通过 `{}`进行格式化显示。
* `Debug`：允许通过 `{:?}` 进行格式化显示。

因为特征(`trait`)指示数据类型之间的共享行为，所以它们在编写泛型时很有用。

&nbsp;

## Further information

* [traits](https://doc.rust-lang.org/book/ch10-02-traits.html)