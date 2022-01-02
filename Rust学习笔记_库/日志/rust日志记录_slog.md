# slog

slog(structured log)，结构化日志。log、log4rs等日志记录软件包都是非结构化日志，直接记录一段话，没有具体的格式。如果程序的日志数量比较小，那么非结构化日志是可以满足要求的，如果日志的数据量很大，那么非结构化的日志就会带来诸多问题。比如，格式多种多样，难以进行查询和解析。

slog 软件的工作原理是应用程序中提供分层和可组合的日志记录配置，并支持语义事件记录。该软件包有两个重要概念:

* 记录器：记录器对象用于记录事件。
* 排水管(drain)：排水管是一个抽象，用于指定日志消息的位置以及它们如何送达(标准输出、文件、网络套接字)。

除了作为基础的slog软件包之外，还提供如下软件包：

* slog-async：提供异步日志记录，将日志记录调用与主线程分离。
* slog-json：将消息以JSON格式输出到任何写入器(writer)的管道。
* slog-term：终端输出
* slog-syslog：系统日志记录

&nbsp;

## slog 具体使用

### Cargo.toml

首先需要修改 `Cargo.toml` 配置文件，增加对 `slog` 支持。

```toml
[dependencies]
rand = "0.5.5"
slog = "2.4.1"
slog-async = "2.3.0"
slog-json = "2.2.0"
slog-term = "2.8.0"
slog-atomic = "3.1.0"
```

&nbsp;

### 具体实现

```rust
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate slog_atomic;
extern crate slog_async;

use slog::*;
use slog_atomic::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        n => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn main() {
    // 根据需要创建排水管(drain)
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();


    // AtomicSwitch: 包装drain
    let drain = AtomicSwitch::new(drain);
    let ctrl = drain.ctrl();

    // 获取root记录器
    let root = Logger::root(
        drain.fuse(),

        // 使用 `o!()` 宏，构建序列化日志
        o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => "8dfljdf"),
    );

    // 对可用日志数据构建日志上下文
    let log = root.new(o!("child" => 1));

    // 闭包捕获数据需要「发送+同步」
    let counter = Arc::new(AtomicUsize::new(0));
    let log = log.new(o!("counter" => {
        let counter = counter.clone();
        // Note the `move` to capture `counter`,
        // and unfortunate `|_ : &_|` that helps
        // current `rustc` limitations. In the future,
        // a `|_|` could work.
        slog::FnValue(  // FnValue 惰性闭包
            move |_ : &Record| { counter.load(SeqCst) }
                )
            }));

    // Loggers 记录器可以被克隆、在线程之间传递并轻松存储 
    let join = thread::spawn({
        let log = log.clone();
        move || {

            info!(log, "before-fetch-add"); // counter == 0
            counter.fetch_add(1, SeqCst);
            info!(log, "after-fetch-add");  // counter == 1

            let drain = Mutex::new(slog_json::Json::default(std::io::stderr()));

            // `AtomicSwitch` drain 原子交换(race-free)
            ctrl.set(
                // drains are composable and reusable
                slog::LevelFilter::new(drain, Level::Info)
                .map(slog::Fuse)
            );

            // Closures can be used for lazy evaluation:
            // This `slow_fib` won't be evaluated, as the current drain discards
            // "trace" level logging records.
            debug!(log, "debug"; "lazy-closure" => FnValue(|_ : &Record| slow_fib(40)));

            info!(log, "subthread"; "stage" => "start");
            thread::sleep(Duration::new(1, 0));
            info!(log, "subthread"; "stage" => "end");
        }
    });

    join.join().unwrap();
}
```

&nbsp;

输出：

```bash
root@8d75790f92f5:~/rs/slog_demo/src# cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `/root/rs/slog_demo/target/debug/slog_demo`
version: 0.1.0
 build-id: 8dfljdf
  child: 1
   counter: 1
    Aug 30 10:50:10.510 INFO before-fetch-add
    Aug 30 10:50:10.510 INFO after-fetch-add
{"msg":"subthread","level":"INFO","ts":"2021-08-30T10:50:10.510864964+08:00","counter":1,"child":1,"build-id":"8dfljdf","version":"0.1.0","stage":"start"}
{"msg":"subthread","level":"INFO","ts":"2021-08-30T10:50:11.511638959+08:00","counter":1,"child":1,"build-id":"8dfljdf","version":"0.1.0","stage":"end"}
```
