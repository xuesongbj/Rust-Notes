# 不安全的Rust

编程语言安全性，它是一个涵盖不同层次的属性。语言可以是内存安全、类型安全的，也可以是并发安全的：

* 内存安全意味着程序不会写入禁用的内存地址，也不会访问无效的内存
* 类型安全意味着程序不允许为字符串变量分配数字，并且此检查会在编译器发生
* 并发安全意味着程序在执行多个线程时不会因为条件竞争而修改共享状态

&nbsp;

## Rust unsafe

Rust的不安全模式也会经常被用到，当程序员比编译器更了解某些细节，并且他们的代码出现一些比较棘手的问题时，因为编译器的所有权限规则过于苛刻，从而带来一些障碍。就可以使用 `unsafe` 模式。

Rust中认为一下操作是不安全的：

* 更新可变静态变量
* 解引用原始指针，例如: `*const T` 和 `*mut T`
* 调用不安全的函数
* 从联合类型中读取值
* 在 `extern` 代码块中调用某个声明的函数 --- 该元素来自其它语言。

在使用非安全模式的时候，会使用 `unsafe` 关键字。它只允许少数几个地方用 `unsafe` 关键字进行标记。

* 函数和方法
* 不安全的代码块表达式，例如 `unsafe {}`
* 特征
* 实现代码块

&nbsp;

### 不安全的函数和代码块

```rust
// 解引用原始指针是非安全的操作，需要使用unsafe模式进行操作
// 如下操作无法通过编译
fn get_value(i: *const i32) -> i32 {
    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    *i
}

fn main() {
    let foo = &1024 as *const i32;
    let _bar = get_value(foo);
}
```

&nbsp;

#### 使用不安全函数和代码块进行解引用

```rust
// unsafe 函数
unsafe fn get_value(i: *const i32) -> i32 {
    *i
}

fn main() {
    let foo = &1024 as *const i32;

    // unsafe 代码块
    let _bar = unsafe { get_value(foo) };
}
```

&nbsp;

#### unsafe 作用域不安全代码

将函数声明为不安全的会让它不能像普通函数那样被调用。可以通过直接在不安全代码处进行操作，以实现相同的操作。

```rust
fn get_value(i: *const i32) -> i32 {
    // 
    unsafe {
        *i
    }
}

fn main() {
    let foo = &1024 as *const i32;
    let _bar = get_value(foo);
}
```

### 不安全的特征和实现

除了函数之外，特征也可以标记为不安全的。需要不安全的特征的原因并不是很明显。将特征(trait)标记为 `unsafe`的动机主要有以下情况:

* 标记无法发送到线程或者线程之间共享的类型
* 封装一些列类型可能具有未定义行为的操作

将特征标记为不安全的**并不会让你的方法也不安全**。

* 可以拥有安全方法的不安全特征
* 也可以拥有不安全方法的安全特征(意味着特征是安全的)

```rust

/*------------------- trait ---------------------------*/

struct MyType;

unsafe trait UnsafeTrait {
    unsafe fn unsafe_func(&self);

    fn safe_func(&self) {
        println!("Things are fine here!");
    }
}

trait SafeTrait {
    unsafe fn look_before_you_call(&self);
}

/*------------------- impl -------------------------*/

unsafe impl UnsafeTrait for MyType {
    unsafe fn unsafe_func(&self) {
        println!("Highly unsafe");
    }
}

impl SafeTrait for MyType {
    unsafe fn look_before_you_call(&self) {
        println!("Something unsafe!");
    }
}

/*------------------ main -------------------------*/
fn main() {
    let my_type = MyType;
    my_type.safe_func();

    // 调用 safe 特征的 unsafe 方法
    unsafe {
        my_type.look_before_you_call();
    }
}
```

```x86asm
(gdb) disassemble
Dump of assembler code for function ddd::main:
   0x0000555555559200 <+0>: push   rax
=> 0x0000555555559201 <+1>: mov    rdi,rsp
   0x0000555555559204 <+4>: call   0x5555555592d0 <ddd::UnsafeTrait::safe_func>  ; 调用my_type 非安全trait
   0x0000555555559209 <+9>: mov    rdi,rsp
   0x000055555555920c <+12>:    call   0x5555555591c0 <<ddd::MyType as ddd::SafeTrait>::look_before_you_call>                                        ; 调用my_type 安全trait
   0x0000555555559211 <+17>:    pop    rax
   0x0000555555559212 <+18>:    ret
```
