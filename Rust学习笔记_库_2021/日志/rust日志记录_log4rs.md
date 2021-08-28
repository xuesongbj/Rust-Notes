# log4rs

相比 `env_logger` 软件包，`log4rs` 软件包要比它强大的多。允许用户通过YAML文件进行细粒度的记录配置。

&nbsp;

## 使用log4rs

### 工作空间

使用log4rs之前，首先创建一个项目的工作空间。

```bash
# 创建工作空间
bash$ mkdir log4rs_demo
bash$ cd log4rs_demo

# 创建多个包
bash$ cargo new my_lib --lib
bash$ cargo new my_app

# 创建工作空间配置，添加包成员
bash$ cat > Cargo.toml << end
[workspace]
members = ["my_lib", "my_app"]
end
```

&nbsp;

### my_lib

`my_lib` 有一个名为 Config 的结构体，包含了一个名为 `load_global_config` 方法，它在 `debug` 级别记录日志消息。

```rust
// log4rs_demo/my_lib/src/lib.rs

use log::debug

pub struct Config;
impl Config {
    pub fn load_global_config() {
        debug!("Configuration files loaded");
    }
}
```

&nbsp;

`my_lib` 包的 `Cargo.toml` 配置：

```toml
# log4rs_demo/my_lib/Cargo.toml

[dependencies]
log = "0.4.8"
```

&nbsp;

### my_app

```rust
// log4rs_demo/my_app/src/main.rs

use log::error;

use my_lib::Config;

fn main() {
    // 通过init_file方法初始化log4rs记录器，并将路径传递给 log4rs.yaml 配置文件
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    error!("Sample app v{}", env!("CARGO_PKG_VERSION"));
    Config::load_global_config();
}
```

```toml
# log4rs_demo/my_app/Cargo.toml

[dependencies]
my_lib = { path = "../my_lib" }
log = "0.4.8"
log4rs = "0.10.0"
```

&nbsp;

#### 加载log4rs.yaml 配置文件

```yaml
refresh_rate: 5 seconds

appenders:
  stdout:
    kind: console
  my_lib_append:
    kind: file
    path: "log/my_lib.log"
    encoder:
      pattern: "{d} - {m}{n}"

root:
  level: error
  appenders:
    - stdout

loggers:
  my_lib:
    level: debug
    appenders:
      - my_lib_append
```

&nbsp;

如上配置解析:

* refresh_rate: 重新加载该yaml文件时间间隔
* root: 根记录器，它是所有记录器的父记录器
* appenders: 输出源
    * stdout: 控制台输出源类型
    * my_lib_append: 自定义输出源类型
* loggers: 日志记录器
    * my_lib: 自定义日志记录器，它用于适配`my_lib`软件包
    * level: 日志级别为
    * appenders: 输出源

&nbsp;

### 执行结果

```bash
root@8d75790f92f5:~/rs/log4rs# cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/my_app`
2021-08-28T11:55:40.448922+08:00 ERROR my_app - Sample app v0.1.0
2021-08-28T11:55:40.449751500+08:00 DEBUG my_lib - Configuration files loaded
```
