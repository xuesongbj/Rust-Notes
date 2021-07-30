# 字符串_str

str 只能创建为引用类型(`&str`)。与String类型相反，str是编译器能够识别的内置类型，并且不属于标准库。字符串切片默认创建为`&str`(一个指向UTF-8编码字节序列的指针)。

```rust
fn main() {
    let message: &str = "wait, but why ?";

    println!("{:?}", message);
}
```

&nbsp;

str基本意味着一个固定大小的字符串序列，这与其所在的位置无关。它既可以是一个堆分配的字符串引用，也可以是存储在进程 `.data` 上的 `&'static str`字符串。

```rust
fn get_str_literal() -> &'static str {
    "from function"
}

fn main() {
    let my_str = "This is borrowed";
    let from_func = get_str_literal();
    println!("{} {}", my_str, from_func);
}
```

&nbsp;

`'static` 生命周期修饰表示字符串在程序运行期间保持不变。`&`表示它指向字符串文本的指针，而 `str` 表示不定长类型。`&str` 类型一旦创建就无法更改，因为默认情况下它是不可变的。`&str` 类型是将自身传递给函数或其他变量时推荐使用的类型。

&nbsp;

## 字符串切片和分块

Rust中的字符串不能像在其他语言中那样通过索引访问字符串，但是可以对字符串进行切片。

```rust
fn main() {
    let my_str = String::from("Strings are cool");
    let first_three = &my_str[0..3];
    println!("{:?}", first_three);
}
```

&nbsp;

### 字符串迭代访问

可以使用 `chars` 方法对字符串进行迭代访问。

```rust
fn main() {
    let hello = String::from("Hello");
    for c in hello.chars() {
        println!("{}", c);
    }
}
```

&nbsp;

## 在函数中使用字符串

`&string` 会自动被强制转换为 `&str`，因为 String 为 str 类型实现了类型强制性特征 `Deref`，该特征确保了 `&String` 到 `&str`的转换。

```rust
fn say_hello(to_whom: &str) {
    println!("Hey {}!", to_whom);
}

fn main() {
   let string_slice: &'static str = "you";
   let string: String = string_slice.into();

   say_hello(string_slice);
   say_hello(&string);
}
```

&nbsp;

## 字符串拼接

Rust语言不能将两个字符串使用 `+` 隐藏字符串链接。Rust不鼓励隐式堆分配，编译器建议我们通过显示将第一个字符串转换成包含所有权的字符串来实现两个字符串的拼接。

```rust
fn main() {
    let foo = "Foo";
    let bar = "Bar";
    let baz = foo.to_string() + bar;
    println!("{:?}", baz);
}
```

&nbsp;

## &str 和 String 应用场景

在Rust中使用字符串，最佳的做法是尽可能使用带有 `&str` 类型的API，因为当字符串已经分配到某处后，只需要用该字符串就可以节省复制和分配的成本。在程序中传递 `&str` 几乎是零成本的：它几乎不会产生分配成本，也不会复制内存。

```rust
fn hello(a: &str) {
    println!("{}", a);
}

fn main() {
    let a = "hello, rust!";
    hello(a);
}
```

```x86asm
// main 函数
(gdb) disassemble
Dump of assembler code for function ddd::main:
   0x0000555555559270 <+0>:	sub    rsp,0x18
   0x0000555555559274 <+4>:	lea    rax,[rip+0x32d86]        # 0x55555558c001
   0x000055555555927b <+11>:	mov    QWORD PTR [rsp+0x8],rax
   0x0000555555559280 <+16>:	mov    QWORD PTR [rsp+0x10],0xc
=> 0x0000555555559289 <+25>:	mov    rdi,rax
   0x000055555555928c <+28>:	mov    esi,0xc
   0x0000555555559291 <+33>:	call   0x5555555591e0 <ddd::hello>
   0x0000555555559296 <+38>:	add    rsp,0x18
   0x000055555555929a <+42>:	ret

// 实参
(gdb) p/x $rax
$1 = 0x55555558c001

// hello函数 形参
(gdb) p/x $rdi
$2 = 0x55555558c001

(gdb) x/12xb $rdi
0x55555558c001:	0x68	0x65	0x6c	0x6c	0x6f	0x2c	0x20	0x72
0x55555558c009:	0x75	0x73	0x74	0x21
```
