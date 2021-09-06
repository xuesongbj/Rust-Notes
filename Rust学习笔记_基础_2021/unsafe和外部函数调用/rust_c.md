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

cc软件包能够自动识别需要调用的C编译器。将 `Cargo.toml` 文件中将 `cc` 软件包作为依赖项构建:

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

#### 改进版本

```rust
fn safe_mystrlen(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) {
        Ok(c) => c,
        Err(_) => return None
    };

    unsafe {
        Some(mystrlen(c_string.as_ptr()))
    }
}

fn main() {
    let count = "Hello, Rust";
    println!("c_string's length is {}", count);
}
```

&nbsp;

## 通过C语言调用Rust代码

### 配置Cargo.toml

`[lib]` 配置中将软件包制定为 `cdylib`，表示将生成一个动态加载的程序库，它在Linux中通常被称为共享对象文件(.so)。

```toml
# {program}/Cargo.toml

[lib]
name = "stringutils"
crate-type = ["cdylib"]
```

&nbsp;

### rust具体实现的lib库

```rust
// {program}/src/lib.rs

use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub enum Order {
    Gt,
    Lt,
    Eq
}

#[no_mangle]
pub extern "C" fn compare_str(a: *const c_char, b: *const c_char) -> Order
{
    let a = unsafe { CStr::from_ptr(a).to_bytes() };
    let b = unsafe { CStr::from_ptr(b).to_bytes() };
    if a > b {
        Order::Gt
    } else if a < b {
        Order::Lt
    } else {
        Order::Eq
    }
}
```

如上代码有以下注释：

* `extern`：表示将其暴露给C语言。
* `[no_mangle]`：Rust默认会在函数名称中添加随机字符，以防止类型名称和函数名称在模块和软件包之间发生冲突(被称为名称改编)。

&nbsp;

### C调用Rust代码

```c
#include <stdint.h>
#include <stdio.h>

int32_t compare_str(const char* value, const char* substr);

int main() {
    printf("%d\n", compare_str("amanda", "brian"));
    return 0;
}
```

### Makefile

```makefile
main:
    cargo build
    gcc main.c -L ./target/debug -lstringutils -o main
```

&nbsp;

* 运行 `main` 函数

```bash
shell$ export LD_LIBRARY_PATH=./target/debug
shell$ ./main
```

&nbsp;

## Rust使用外部C/C++ 程序

可以使用 `bindgen` 的便捷软件包，它可以自动生成对 C/C++的FFI(Foreign Function Interface)绑定库。`bindgen` 依赖以下软件包:

```bash
bahs$ apt-get install llvm-3.9-dev libclang-3.9-dev clang3.9

```

&nbsp;

### 配置Cargo

修改 `Cargo` 配置，将 `bindgen` 和 `cc` 软件包添加到构建依赖项:

```toml
[build-dependencies]
bindgen = "0.43.0"
cc = "1.0"
```

&nbsp;

### rust 代码

```rust
// edit_distance/build.rs

use std::path::PathBuf;

fn main() {
    // 如果当前目录中的任何文件发生变化，Cargo程序库重新运行该文件
    println!("cargo:rustc-rerun-if-changed=.");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=levenshtein");

    // 创建编译管道
    cc::Build::new()
        .file("lib/levenshtein.c")
        .out_dir(".")
        .compile("levenshtein.so");

    let bindings = bindgen::Builder::default()
        .header("lib/levenshtein.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/");
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
```

```rust
// edit_distance/src/lib.rs

use crate::bindings::levenshtein;
use std::ffi::CString;

pub fn levenshtein_safe(a: &str, b: &str) -> u32 {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    let distance = unsafe { levenshtein(a.as_ptr(), b.as_ptr())};
    distance
}
```

```rust
// edit_distance/examples/basic.rs

use edit_distance::levenshtein_safe;

fn main() {
    let a = "foo";
    let b = "foo";

    assert_eq!(1, levenshtein_safe(a, b));
}
```
