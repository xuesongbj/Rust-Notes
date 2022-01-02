# 构建同步Redis服务器

Redis是一种数据结构服务器，通常用作内存数据存储。Redis客户端和服务端使用Redis序列化协议(REdis Serialization Protocol, RESP)，这是一种简单的基于流的有状态协议。

RESP支持各种消息，包括简单字符串、整数、数组及批量字符串等。RESP中的消息以`\r\n`字节序列结束。例如，从服务器到客户端到客户端的成功消息被编码并发发送为 `+OK\r\n`。 `+`表示成功回复。该命令以 `\r\n`结尾。若指令查询失败，Redis服务器将回复 `-Nil\r\n`。

&nbsp;

## 构建同步Redis服务器

&nbsp;

### 修改配置项

```toml
# rudis_sync/Cargo.toml

[dependencies]
lazy_static = "1.2.0"
resp = { git = "https://github.com/creativcoder/resp"}

```

&nbsp;

* lazy_static：将使用它来存储我们的内存数据库。
* resp: resp第三方库，用它解析来自客户端的字节流。

&nbsp;

### 具体实现

```rust
// rudis_sync/src/main.rs

use lazy_static::lazy_static;
use resp::Decoder;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
// use std::time::Duration;

mod commands;
use crate::commands::process_client_request;

type STORE = Mutex<HashMap<String, String>>;

// 可实现延迟初始化 static 常量
lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

// handle_client在stream变量中接收客户端TcpStream 套接字
fn handle_client(stream: TcpStream) {
    // 将客户端 stream包装到BufReader中
    let mut stream = BufReader::new(stream);

    // 作为可变引用传递给resp软件包的Decoder::new方法
    let decoder = Decoder::new(&mut stream).decode();
    match decoder {
        Ok(v) => {
            // 解码成功，调用process_client_request
            let reply = process_client_request(v);

            // 通过在客户端stream上调用write_all将reply写入客户端
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            // 解码失败，Shutdown::Both值关闭客户端套接字连接的读取和写入部分
            println!("Invalid command: {:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    }
}

fn main() {
    let addr = env::args()
                .skip(1)
                .next()
                .unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    println!("rudis_sync listening on {} ...", addr);

    // listener上调用incoming方法，然后返回新客户端连接迭代器
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New connection from: {:?}", stream);
        // thread::sleep(Duration::from_millis(3000))

        // 每当建立�的客户端连接时，生成一个新线程从主线程转移handle_client调用，从而允许主线程接受其它客户端连接
        thread::spawn(|| handle_client(stream));
    }
}
```

```rust
// rudis_sync/src/commands.rs

use crate::RUDIS_DB;
use resp::Value;

// process_client_request 已经获取解码后的Value，并将其与已解析的查询进行匹配
pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_msg {
        match &v[0] {
            // Value::Bulk 将命令包装成字符串
            Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
            Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
            other => unimplemented!("{:?} is not supported as of now", other),
        }
    } else {
        Err(Value::Error("Invalid Command".to_string()))
    };

    match reply {
        Ok(r) | Err(r) => r.encode(),
    }
}

// handle_get 检查GET命令在查询是否包含相应的key，在查询失败时，现实错误信息
pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() {
        return Err(Value::Error("Expected 1 argument for GET command".to_string()))
    }

    let db_ref = RUDIS_DB.lock().unwrap();
    let reply = if let Value::Bulk(ref s) = &v[0] {
        db_ref.get(s).map(|e|
    Value::Bulk(e.to_string())).unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    Ok(reply)
}

// handle_set 将&v[0]和&v[1]向匹配的键和值插入RUDIS_DB中
pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error("Expected 2 arguments for SET command".to_string()))
    }
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            let _ = RUDIS_DB
                    .lock()
                    .unwrap()
                    .insert(k.to_string(), v.to_string());
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
    }
    Ok(Value::String("Ok".to_string()))
}
```

&nbsp;

### 运行Redis

```bash
root@8d75790f92f5:~/rs/rudis_sync/src# cargo r &
[1] 611
root@8d75790f92f5:~/rs/rudis_sync/src#     Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `/root/rs/rudis_sync/target/debug/rudis_sync`
rudis_sync listening on 127.0.0.1:6378 ...
```

&nbsp;

* 连接Rudis服务器

```bash
root@8d75790f92f5:~/rs/rudis_sync/src# redis-cli -p 6378
New connection from: TcpStream { addr: 127.0.0.1:6378, peer: 127.0.0.1:59186, fd: 4 }
127.0.0.1:6378> set k v
New connection from: TcpStream { addr: 127.0.0.1:6378, peer: 127.0.0.1:59188, fd: 5 }
Ok
127.0.0.1:6378> get k
New connection from: TcpStream { addr: 127.0.0.1:6378, peer: 127.0.0.1:59190, fd: 4 }
"v"
```
