# Rust 学习笔记

[![License](https://img.shields.io/npm/l/heroicons.svg)](https://github.com/xuesongbj/Rust-Notes/blob/main/LICENSE)

Rust是一门系统级编程语言，和C、C++、Go等语言的设计思想有较大差异。学习Rust并不仅仅学习一门语言，更重要是学习它的设计思想。

![rust programming language](./rust_language.jpeg)

&nbsp;

## 为什么使用Rust？

我们都是人，我们的注意力是有限的，我们的记忆是不稳定的--换句话说，我们容易犯错。

计算机和软件无处不在：在太空、地面、海洋、甚至我们的身体中。每天都有越来越多的系统实现自动化，越来越多的人依赖软件(高稳定性、高质量)。

航空电子设备、自动驾驶汽车、核电站、交通控制系统、植入式心脏起搏器等。此类系统中的错误几乎总是危机人类生命。

「通过测试检查程序正确性」和「逻辑证明程序正确性」之间存在着巨大差异。不幸的是，即使我们对代码的每一样都进行了测试，我们仍然无法确定它是否正确。

&nbsp;

### Rust优势

「Rust作为一种编程语言」的不同之处，学习Rust不是因为它花哨的语法或热情的社区，而是因为人们在用它便携程序时获的的信心。**Rust非常严格和严谨的编译器检查您使用的每个变量以及您引用的每个内存地址** 。看起来它会影响您编写效率、富有表现力的代码，但令人惊讶的是，恰恰相反：**编写一个有效且惯用的Rust程序实际上比编写潜在漏洞的程序更容易** 。

在后一种情况下，您将与编译器冲突，因为您的每个操作都可能导致内存安全漏洞。

![](./linux-cves-in-2018.png)

[talk-konstanz-may-2018](https://phil-opp.github.io/talk-konstanz-may-2018/#14)

上图右侧显示了并发性和内存安相关的问题，这些问题根源上不可能发生在常规(非unsafe)Rust代码中。所以，**只需要换成Rust，他们就可以杜绝这段时间内一大半的Bug**，因为它们会导致机密泄漏、拒绝服务和远程代码执行漏洞。

Linux 内核是由业内顶级的 **5%** 中的佼佼者编写的，然而每年仍然可以在CVE(CVE是国际著名的安全漏洞库)中发现50多个安全漏洞。当然，与数百万行代码相比，这 **50** 个错误微不足道。但是，生死问题，还记得吗？ 当我们谈论关键系统时，即使是微小的错误也可能导致灾难性的后果。更不用说这 **50** 个是发现的错误，谁知道还有多少没有被发现的？ **如果使用Rust，我们会在编译发现这些问题** 。

&nbsp;

#### 运行速度优势

现在编程语言中的 **内存安全伴随着垃圾回收的成本** ，并发通常通过同步原子性锁定所受影响的数据结构和执行路径进行解决。对于Rust来说，并不会采用运行时GC这种方式保证内存安全，Rust在编译时就解决了这些问题。

在`C++`中，只需要为使用的内存负责。例如，在Rust中，只有在绝对需要时才使用Mutex，而且Rust编译器会强制你使用它，所以你永远不会忘记添加它。而这一切基本上都是零成本的。由于大多数检查是在编译器执行的，因此编译后的程序与`C`或`C++`编译器生成的程序集没有太大区别。正因为如此，**Rust现在在嵌入式电子、物联网，甚至操作系统开发领域都非常有前途** -- 以前由于高控制要求和严格的资源和性能限制，这些领域由`C`主导。

Rust最新版本甚至为用户空间带来了SIMD支持。之前，由于API稳定性限制，它仅在`beta`版本中可用。现在，你可以通过直接使用向量指令或使用方便的lib库来释放硬件的潜力。

&nbsp;

#### 内存安全可保证

* 没有无效的内存访问
    * 没有缓冲区溢出(No buffer overflows)
    * 没有悬垂指针(No dangling pointers)
    * 没有数据竞争(No data race)

&nbsp;

#### 不断完善的工具链

* rustup: 不同的目录可以构建不同的rust版本
* cargo: 自动下载、编译和链接依赖项
* rustfmt: 根据样式格式化rust代码
* Rust Playground: 以浏览器方式运行和共享代码片段

![rust playground](./rust_playground.png)

* clippy: 语法检查工具
* proptest: 属性测试框架
* bootimage: 从 Rust 内核创建可引导磁盘映像

&nbsp;

#### 持续迭代的新功能

* 隐含特征(impl Trait): 从函数返回闭包
* Non-Lexical生命周期: 更智能的借用检查器
* WebAssembly: 在浏览器中运行Rust
* Async: Async/Await、Generators(yield)

#### 日益壮大的Rust基金会

继亚马逊AWS、华为、谷歌Google、微软Microsoft和Mozilla后，**FaceBook宣布加入Rust基金会**，并承诺将加大对Rust采用。

Rust董事会成员共有12人，创始成员承诺：在2年内，将提供**每年超过100W美元的预算**，用于Rust项目的维护、开发和推广。

![Board of Directors](./rust_board.png)

[rust foundation members](https://foundation.rust-lang.org/members/)

[Rust Foundation Overview](https://foundation.rust-lang.org/static/rust-foundation-overview.pdf)

&nbsp;

#### 活跃社区及Linus认可

2021/07/04号，Linux支持Rust作为第二语言以支持补丁到Linux内核，得到Linux基金会的充分认可。

![rust for linux](./imgs/rust-for-linux.jpg)

[LKML Archive on lore.kernel.org](https://lore.kernel.org/lkml/20210704202756.29107-1-ojeda@kernel.org/)

[Rust-for-Linux](https://github.com/Rust-for-Linux/linux)

&nbsp;

## 笔记目录

```bash
.
├── Rust学习笔记_库_2021
│   ├── Rust构建Web应用程序
│   │   └── hyper
│   │       ├── hyper
│   │       ├── hyperurl
│   │       │   ├── Cargo.lock
│   │       │   ├── Cargo.toml
│   │       │   └── src
│   │       │       ├── main.rs
│   │       │       ├── service.rs
│   │       │       └── shortener.rs
│   │       └── shorten
│   │           ├── Cargo.lock
│   │           ├── Cargo.toml
│   │           └── src
│   │               └── main.rs
│   ├── Rust网络编程
│   │   ├── README
│   │   ├── 异步网络IO
│   │   └── 构建同步Redis服务器
│   ├── exercises
|   |   ├── advanced_errors
|   |   ├── clippy 
│   │   ├── collections
|   |   ├── conversions
│   │   ├── enums
|   |   ├── error_handling
│   │   ├── functions
|   |   ├── generics
|   |   ├── if
|   |   ├── macros
|   |   ├── modules
|   |   ├── move_semantics
│   │   ├── options
|   |   ├── primitive_types
|   |   ├── standard_library_types
│   │   ├── strings
│   │   ├── structs
|   |   ├── tests
|   |   ├── threads
│   │   ├── traits
│   │   └── variables
|   |   ├── quiz1.rs
|   |   ├── quiz2.rs
|   |   ├── quiz3.rs
|   |   ├── quiz4.rs
│   ├── 日志
│   │   ├── rust日志记录
│   │   ├── rust日志记录_log4rs
│   │   └── rust日志记录_slog
│   └── 网络编程
├── Rust学习笔记_基础_2021
│   ├── rust快查手册
│   ├── unsafe和外部函数调用
│   │   ├── rust_c
│   │   └── unsafe
│   ├── 包
│   │   ├── 包
│   │   │   └── 包
│   │   ├── 箱
│   │   │   └── 箱
│   │   ├── 模块
│   │   │   └── 模块
│   │   ├── 导入
│   │   └── 工作空间
│   │       └── 工作空间
│   ├── 函数
│   │   ├── 闭包
│   │   │   ├── 闭包
│   │   │   ├── 作为参数
│   │   │   └── 作为返回值
│   │   └── 函数
│   ├── 对象
│   │   ├── 复制
│   │   ├── 方法
│   │   ├── 析构
│   │   ├── 泛型
│   │   ├── 特征
│   │   │   ├── 传递
│   │   │   ├── 关联
│   │   │   ├── 同名
│   │   │   ├── 多态
│   │   │   ├── 泛型
│   │   │   ├── 继承
│   │   │   ├── 其它
│   │   │   ├── 特征
│   │   │   ├── 操作符
│   │   │   ├── 特征区间
│   │   │   ├── 特征对象
│   │   │   └── 特征对象和对象安全性
│   │   └── 迭代器
│   ├── 工具
│   │   └── 工具
│   ├── 并行
│   │   └── 计算机系统结构
│   ├── 并发
│   │   ├── 同步
│   │   ├── 异步
│   │   |   ├── Pin和Unpin.md
│   │   |   ├── async和await!
│   │   |   ├── future原理
│   │   |   └── waker
│   │   ├── 消息
│   │   ├── 线程
|   |   |   ├── barrier(屏障)
|   |   |   ├── mutex(互斥锁)
|   |   |   ├── rwlock(读写锁)
|   |   |   ├── channel
|   |   |   ├── Rayon
|   |   |   ├── atomic(原子类型)
|   |   |   ├── send_sync
|   |   |   ├── thread_pool(线程池)
|   |   |   ├── unsafeCell
|   |   |   ├── crossbeam
|   |   |   ├── convar(条件变量)
|   |   |   ├── send和sync
|   |   |   ├── 代码执行流程
│   │   |   └── 线程的并发模型
│   ├── 数据
│   │   ├── 切片
│   │   ├── 向量
│   │   ├── 指针
│   │   ├── 枚举
│   │   ├── 结构
│   │   ├── 联合
│   │   ├── 字符串
│   │   └── 全局值
│   ├── 测试
│   │   ├── 单元测试
│   │   └── 基准测试
│   ├── 类型
│   │   ├── 别名
│   │   ├── 变量
│   │   ├── 常量
│   │   ├── 类型
│   │   ├── 转换
│   │   ├── 类型转换
│   │   │   ├── From和Into
│   │   │   ├── ToString和FromStr
│   │   │   └── TryFrom和TryInto
│   │   └── 类型大小
│   ├── 进阶
│   │   ├── 宏
│   │   └── 注释
│   ├── 错误
│   │   ├── 异常处理
│   │   ├── 可恢复错误
│   │   └── 不可恢复错误
│   ├── 附录
│   │   ├── 安装
│   │   ├── 编译
│   │   └── 资源
│   ├── 所有权
│   │   ├── Borrow
│   │   ├── 所有权
│   │   ├── 生命周期
│   │   └── 引用和借用
│   ├── 表达式
│   │   ├── 控制流
│   │   ├── 表达式
│   │   ├── 迭代器
│   │   └── 模式匹配
│   ├── 内存管理
│   ├── 内部实现
│   └── 宏和元编程
│       ├── 宏
│       └── 元编程
└──
```
