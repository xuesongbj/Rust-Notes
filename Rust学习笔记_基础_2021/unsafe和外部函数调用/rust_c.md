# rust和C

## 在Rust中调用C代码

cc软件包能够自动识别需要调用的C编译器，通过正确的链接器标志，cc软件包完成了将C源代码编译和链接为二进制文件的所有繁重工作。

&nbsp;

### 实例


&nbsp;

#### 创建C代码

```c
// c_from_rust/mystrlen.c

unsigned int mystrlen(char *str) {
    unsigned int c;
    for (c = 0; *str != '\0'; c++, *str++);
    return c;
}
```

以上 `mystrlen` 函数会返回传递给它的字符串长度。我们希望从Rust 调用 `mystrlen`，为此需要将C源代码编译为静态库。

&nbsp;

#### cc软件包作为依赖项构建

将 `Cargo.toml` 文件中将 `cc` 软件包作为依赖项构建:

```toml
# c_from_rust/Cargo.toml

[build-dependencies]
cc = "1.0"
```

&nbsp;

#### 声明构建命令

声明构建命令，需要在软件包根目录下添加一个 `build.rs`。

```rust
// c_from_rust/build.rs

fn main() {
    cc::Build::new().file("mystrlen.c")
                    .static_flag(true)          // 静态共享包
                    .compile("mystrlen")        // 声明构建命令名称
}
```

&nbsp;

#### Rust使用C代码

```rust
// c_from_rust/src/main.rs

use std::os::raw::{c_char, c_uint};     // 数字类型，类型前面的单个字母表示该类型是无符号的
use std::ffi::CString;

// "C": 用于指定，我们希望编译器的代码生成器是C ABI(cdecl)，以便函数调用完全遵循 C 语言的函数调用方式
extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}

fn main() {
    let c_string = CString::new("C From Rust").expect("failed");
    let count = unsafe {
        mystrlen(c_string.as_ptr())
    };
    println!("c_string's length is {}", count);
}
```

&nbsp;

ffi模块主要包含两种字符串类型：

* `std::ffi::CStr`：表示一个类似于 `&str` 的借用C字符串。它可以引用C语言中创建的字符串。
* `std::ffi::CString`：表示与外部C函数兼容并且包含所有权的字符串。它通常用于将字符串从Rust中传递到外部C函数。

&nbsp;

cc软件包能够自动识别需要调用的C编译器。
