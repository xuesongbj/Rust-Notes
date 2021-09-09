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
* `trait T:R {}`：`T trait`是 `R trait`超集，任何`S` 必须先实现R，然后才能实现 `T`
* `impl S{}`：实现 `S` 的方法
* `impl T for S{}`：S类型实现 `T trait` 方法
* `impl !T for S{}`：禁用 `T trait` 的默认实现
* `fn f(){}`：定义函数，如果在 `impl` 内部，则实现该方法
* `fn f(){} -> S{}`：返回值 `S` 类型
* `fn f(&self) {}`：在 `impl` 内部定义方法
* `const fn f(){}`：常量函数，在编译时使用
* `async fn f() {}`：Async函数变体，`f` 函数返回 `impl Future`
* `async fn f() -> S{}`：同上，返回 `impl Future<Output=S>`
* `async { x }`：在函数内部使用，{ x } 返回 `impl Future<Output=X>`
* `fn() -> S`：函数指针
* `Fn() -> S`：Callable Trait，被闭包实现(impl)
* `|| {}`：闭包
    * `|x| {}`：闭包参数 `x`
    * `|x| x + x`：闭包返回简单表达式
    * `move |x| x + y`：闭包所有权转移；`y` 转移到闭包
    * `return || true`：返回闭包
* `unsafe`：操作非内存安全数据
    * `unsafe fn f() {}`：非安全(`unsafe`)函数
    * `unsafe trait T {}`：非安全(`unsafe`)特征(`trait`)
    * `unsafe { fn(); }`：`unsafe` 代码块
    * `unsafe impl T for S {}`：`unsafe` 实现(`impl`)

&nbsp;

## Control Flow (控制流)

* `while x{}`：如果 `x` 是 true一直执行
* `loop {}`：`loop`循环，直到遇到 `break`，退出循环
* `for x in iter{}`：循环迭代器
* `if x {} else {}`：条件分支
* `label: loop {}`：循环标签；用于嵌套循环中的流程控制
* `break`：退出循环
    * `break x`：退出循环，使用x值作为循环表达式的值
    * `break 'label`：跳出 `'label`的循环
    * `break 'label x`：跳出 `'label`的循环，使用x值作为循环表达式的值
* `continue`：跳过当次循环，继续
* `continue 'label`：跳过当次循环，继续 `'label`
* `x?`：`Result` 结果错误处理
* `x.await`：async编程时使用，直到Future或者x数据流到达，才会被执行`await`
* `return x`： 提前返回值
* `f()`：函数闭包调用
* `x.f()`：函数方法调用
* `X::f(x)`：除非 `impl Copy for X {}`，否则只能被调用一次
* `X::f(&x)`：方法调用
* `X::f(&mut x)`：方法调用
* `X::f()`：调用关联函数，例如：`X::new()`
    * `<X as T>::f()`：调用特征(`trait`) `T::f()` X 的实现

&nbsp;

## Organizing Code(组织代码)

将项目分割成更小的单元(crate)并最小化依赖性

* `mod m {}`：定义mod，从 `{}` 中获取 `mod` 定义代码
* `mod m;`：定义mod，获取定义内容 `m.rs` 或者 `m/mod.rs` 文件
* `a::b`：`Namespace` 路径
* `::b`：搜索 `b`，相对于单元(`crate`)根路径
* `crate::b`：搜索 `b`，相对于单元(`crate`)根路径
* `self::b`：搜索 `b`，相当于当前 `module`
* `super::b`：搜索 `b`，相当于当前 `parent`
* `use a::b`：直接使用
* `use a::{b, c};`：简写 `a::b`， `a::c`
* `use a::b as x;`：重命名
* `use a::b as _;`：将 `b` 匿名带入作用域，对于名称冲突的特征很有用
* `use a::*;`：将 `a` 下的所有符号全部导入
* `pub use a::b;`：将 `a::b` 带入当前作用域，并从此处退出
* `pub T`：导出
    * `pub(crate) T`：`T` 只在当前单元(`crate`)使用
    * `pub(super) T`：`T` 最多可在父级作用域中使用
    * `pub(self) T`：`T` 只能在当前模块使用(默认行为，和不加 `pub` 作用相同)
* `extern crate a;`：导入外部的包，在最新的rust 2018已经不需要声明
* `extern "C" {}`：编译器生成C ABI代码，函数调用遵循 `C` 调用方式进行使用
* `extern "C" fn f()`：从其它语言调用Rust
