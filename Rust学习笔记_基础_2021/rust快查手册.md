# Rust快查手册

## Data Structure

&nbsp;

### 通过关键字定义的数据类型和内存位置

* `struct S{}`：定义一个名称为`S`的结构体
    * `struct S{ x: T }`：定义结构体字段 `x` 类型为 `T`
    * `struct S(T)`：定义 `tuple` 结构体，`.0` 元素为类型 `T`
    * `struct S`：定义一个 Zero sized 的单元结构体。不占用空间，编译器优化
* `enum E{}`：定义枚举
    * `enum E{ A, B(), C{} }`：定义`enum`，可以是 `crate A`、`tuple B()`、`struct c{}`
    * `enum E{ A = 1 }`：枚举 `A = 1`
* `union U{}`：定义联合
* `static X:T = T()`：全局变量 `static X`，进程级生命周期
* `const X:T = T()`：定义常量，使用的时候拷贝到临时内存
* `let x:T`：分配 `T` 数据，绑定到 `x`(stack)，赋值一次，不可变
* `let mut x: T`：和上面类似，是可变的，借用可变
    * `x = y`：如果 `y` 不能被 `Copy`，移动(move) `y` 到 `x`，`y` 将失效，否则复制 `y`

&nbsp;

### 创建和访问数据结构，和更多的符号类型

* `S { x: y }`：创建 `struct S{}` 或者 `enum E::S{}` 的 `x` 字段设置 `y` 值
* `S { x }`：设置变量 `x` 到 `struct x` 字段
* `S { ..s }`：使用 `s` 的全部同字段填充
* `S { 0: x }`：tuple struct `.0` 设置x
* `S(x)`：创建`struct S(T)`(tuple)，或用 `enum E::S()` x 赋值给元组结构体 `.0` 元素
* `S`：`S crate struct`，`enum E::S` 创建S
* `()`：空tuple
* `(x)`：扩号表达式
* `(x,)`：单元素tuple
* `(S,)`：单元素类型tuple
* `[S]`：Slice(不知道长度的Type Array)
* `[S; n]`：元素类型为 `S`，长度为 n Array
* `[x; n]`：Array实例 `n` 个 `x` 的拷贝
* `[x, y]`：Array实例 `x, y` 元素
* `x[0]`：集合中获取索引value
* `x[..]`：获取slice全部数据
* `x[a..b]`：获取a到b个元素(不包含b)
* `x[..b]`：获取0到b个元素(不包含b)
* `x[a..=b]`：获取a到b个元素(包含b)
* `x[..=b]`：获取0到b个元素(包含b)
* `s.x`：命名字段访问,如果 `x` 不是类型 `S` 的一部分，可能是 `Deref`
* `s.0`：元组类型访问