# Rust 日志记录

Rust中有很多灵活、可扩展的日志记录解决方案。Rust中日志记录生态系统分为两个部分。

* 日志记录外观： 此部分由 `log` 软件包实现，并提供与实现无关的日志记录API。这些日志记录API按日志级别进行分类，以便将日志记录输出到预配置的目标上。

* 日志记录实现：可以根据输出的位置和事件发生的方式提供实际的日志记录实现。例如：env_logger、imple_logger、log4rs及fern。

日志记录API和日志记录输出的底层机制之间是分离的，开发人员不需要在代码中更改其日志语句，可以根据需要轻松地切换基本的日志记录实现。

&nbsp;

## log

`log` 软件包提供了单独的宏来记录不同的日志级别，例如 `error!`、`warn!`、`info!`、`debug!` 及 `trace!`。这些宏是该软件包用户的主要交互点，他们在内部会调用此软件包的 `log` 宏，以便执行所有日志记录操作。

`log` 软件包还提供了一个名为 `STATIC_MAX_LEVEL` 的最大日志级别常量，可以在编译期于项目范围内配置。通过该常量可以使用 cargo 特性标记来设置应用程序的日志级别，这允许对应用程序及所有依赖项的日志进行编译期过滤。你可以使用编译器日志级别指定 `log` 程序库依赖关系。

```toml
[dependencies]
log = { version = "0.4.6", features = ["release_max_level_error", "max_level_trace"]}
```

&nbsp;

### log 和 env_logger 实例

&nbsp;

#### log_为Rust日志记录提供外观

创建日志程序库。

```bash
bash$ cargo new user_auth --lib

```

&nbsp;

使用 `log` 作为日志提供外观。

```toml
# user_auth/Cargo.toml

[dependencies]
log = "0.4.6"
```

#### env_logger

使用 env_logger 软件包实现日志的记录，它允许通过环境变量RUST_LOG将日志输出到stdout或stderr。

```bash
bash$ cargo new env_logger_demo
```

&nbsp;

配置 `Cargo.toml` 文件，使用log lib库。

```toml
# env_logger_demo/Cargo.toml

[dependencies]
env_logger = "0.6.0"
user_auth = { path = "../user_auth" }
log = { version = "0.4.6", features = ["release_max_level_error", "max_level_trace"]}
```

```rust
// env_logger_demo/src/main.rs

use log::debug;
use user_auth::User;

fn main() {
    env_logger::init();
    debug!("env logger demo started");
    let user = User::new("bob", "super_sekret");
    user.sign_in("super_secret");
    user.sign_in("super_sekret");
}
```

```bash
root@8d75790f92f5:~/rs/env_logger_demo/src# RUST_LOG=user_auth=info,env_logger_demo=info cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `/root/rs/env_logger_demo/target/debug/env_logger_demo`
[2021-08-27T02:19:10Z INFO  user_auth] Signing in user: bob
[2021-08-27T02:19:10Z ERROR user_auth] Login failed for user: bob 
```

我们将 `user_auth` 设置为info级别，将 `env_logger_demo` 软件包的日志记录设置为 debug 级别。

RUST_LOG接受 `RUST_LOG=path::to_module=log_level[,]` 模式，其中 `path::to_module` 用于指定记录器，并且以软件包为基础的任何模块的路径。 `log_level`是日志软件包中定义的日志级别。

运行上述程序还可以使用 `env` 模块中的 `set_var` 方法设置环境变量进行使用:

```rust
use std::env::set_var;
use log::debug;
use user_auth::User;

fn main() {
    set_var("RUST_LOG", "user_auth=info,env_logger_demo=info cargo run");
    env_logger::init();
    debug!("env logger demo started");
    let user = User::new("bob", "super_sekret");
    user.sign_in("super_secret");
    user.sign_in("super_sekret");
}
```
