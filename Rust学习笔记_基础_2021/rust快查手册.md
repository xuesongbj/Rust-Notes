# Rust快查手册

Rust Cheat Sheet(Rust语言备忘录)原版本，可参考[该链接](https://cheats.rs/)

&nbsp;

## Data Structure(数据结构)

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

&nbsp;

## References & Pointers(引用和指针)

* `&S`：共享引用
    * `&[S]`：特殊的slice引用(包含: address, length)
    * `&str`：特殊的string slice引用(包含: address, length)
    * `&mut S`：允许可变性的独占引用(`&mut [S]`,`&mut dyn S`, ...)
    * `&dyn T`：特殊 `Trait object` 引用(包括:address, vtable)

* `&s`：共享借用
    * `&mut s`：独享可变借用对象
    * `&raw const s`：`unsafe` 原始指针
    * `&raw mut s`：可变原始指针(原始指针需要未对齐压缩字段)

* `ref s`：通过引用绑定
    * `let ref r = s`：同 `let r = &s` 效果相同
    * `let S {ref mut x} = s`：可变绑定(`let x = &mut s.x`)简写版本
    
* `*r`：指针解引用
    * `*r = s`：如果 `r` 是可变引用，`move or copy s` 到目标 `memory`
    * `s = *r`：如果 `r` 可以 `Copy`，复制 `r`
    * `s = *r`：如果 `r` 不可以 `Copy`，错误
    * `s = *my_box`：`Box`特殊用法，如果 `Box` 未实现 `Copy`，则 `Box` 进行 `move`(所有权转移)

* `'a`：生命周期
    * `&'a S`：只接受一个带有 `s` 的地址，地址生命周期比 `'a` 更长
    * `&'a mut S`：同上，但是 `s` 指向的内存可变
    * `struct S<'a> {}`：`S`地址的生命周期是`'a`，创建 `S` 决定了 `'a`生命周期长短
    * `trait T<'a> {}`：`S`实现`T`, `S` 决定了 `'a`生命周期长短
    * `fn f<'a>(t: &'a T)`：调用者决定了 `'a` 生命周期长短

* `'static`：持续整个程序执行的特殊生命周期

&nbsp;

## Function & Behavior(函数和行为)

定义代码单元(crate)及其抽象

* `trait T {}`：定义一个 `trait`；其它人可以实现 `implement`
