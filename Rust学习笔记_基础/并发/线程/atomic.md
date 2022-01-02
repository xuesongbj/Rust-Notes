# 原子类型(Atomic)

互斥锁(`Mutex`)、读写锁(`RWLock`)等同步事件(原语)确实可以满足基本的线程安全需求，但是有时候使用锁会影响性能，甚至存在死锁之类的风险，因此引入原子类型。

原子类型内部封装了编程语言和操作系统的「契约」，基于此「契约」来实现一些自带原子操作的类型，而不需要对其使用锁来保证原子性，从而实现无锁(`Lock-Free`)并发编程。这种契约就是**多线程内存模型**，Rust的多线程内存模型借鉴了`C++ 11`，它保证了多线程并发的顺序一致性，不会因为底层的各种优化重排行为而失去原子性。

原子性操作类型:

* `Load`：从一个原子类型内部读取值。
* `Store`：往一个原子类型内部写入值。
* 「读取-修改-写入」操作:
    * `CAS(Compare-And-Swap)`：比较并交换。
    * `Swap`：原子交换操作。
    * `Compare-Exchange`：比较/交换操作。
    * `Fetch-*`：`fetch_add`、`fetch_sub`、`fetch_and`、`fetch_or` 等一系列原子的加减或逻辑运算。

&nbsp;

## Rust提供原子类型

Rust标准库 `std::sync::atomic` 模块中提供了4个原子类型：`AtomicBool`、`AtomicIsize`、`AtomicPtr` 和 `AtomicUsize`。

使用原子类型实现一个简单自旋锁(`Spinlock`)。

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // 原子类型本身虽然可以保证原子性，但它自身不提供在多线程中共享的方法
    // 需要使用 Arc<T> 将其跨线程共享
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        // 使用pinlock_clone的store方法，将其内部AtomicUsize类型值写为0
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // main主线程使用spinlock的load方法读取内部原子类型的值，如果不为0
    // 则不停地循环测试读取锁的状态，直到其状态被设置为0为止(自旋锁)
    while spinlock.load(Ordering::SeqCst) != 0 {}

    // 使用join方法阻塞main主线程等待子线程完成
    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
```

&nbsp;

## 内存顺序

原子类型除了提供基本的原子操作之外，还提供了内存顺序参数。每个原子类型虽然对开发者而言是一个“黑盒”，但也可以通过提供内存顺序参数来控制底层线程执行顺序。控制内存顺序实际上就是控制底层线程同步，以便消除底层因为编译器优化或者指令重排而可能引发的竞态条件。

`std::sync::atomic::Ordering` 模块中定义了Rust支持的5种内存顺序，可以将其分为三大类：

* **排序一致性顺序**：`Ordering::SeqCst`
* **自由顺序**：`Ordering::Relaxed`
* **获取-释放顺序**：`Ordering::Release`、`Ordering::Acquire` 和 `Ordering::AcqRet`

```rust
// library/core/src/sync/atomic.rs
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

Rust支持的5种内存顺序与底层的LLVM支持的内存顺序是一致的。

&nbsp;

### 排序一致性顺序(SeqCst)

排序一致性顺序，也就是指定 `Ordering::SeqCst`的原子操作，都必须是先存储(store)再加载(load)。多线程下，所有原子写操作都必须在读操作之前完成。通过这种规定，强行指定了底层多线程的执行顺序，从而保证了多线程中所有操作的全局一致性。但需要付出代价的，这种方式需要所有的线程进行全局同步，存在性能消耗。

日常开发中如何选择内存顺序和底层硬件环境也有关系，一般情况下建议使用 `Ordering::SeqCst`。在需要性能优化的情况下，先调研硬件环境，再优先选择获取-释放顺序(`Ordering::Relase`、`Ordering::Acquire` 和 `Ordering::AcqRel`)。除非必要，否则不要使用 `Ordering::Relaxed`。

&nbsp;

### 自由顺序(Release)

**自由顺序** 与排序一致性顺序相反，线程只进行原子操作，但线程之间会存在竞态条件。使用这种内存顺序比较危险的，只有在明确了使用场景且必须使用它的情况下(Eg：只有读操作)，才可使用自由顺序。

&nbsp;

### 获取-释放顺序(load-store)

**获取-释放顺序是除排序一致性之外的优先选择**。这种内存顺序并不会对全部的线程进行统一强制性的执行顺序要求。在该内存顺序内，`store` 代表释放(Release)，`load` 代表获取(Acquire)，通过这两种操作的协作实现线程同步。

进行释放(`Ordering::Release`)操作时，之前所有的获取(`Ordering::Acquire`)操作都是可见的；进行获取(`Ordering::Acquire`)操作时，之前所有释放(`Ordering::Release`)操作都是可见的。`Ordering::AcqRel`代表读时使用 `Ordering::Acquire`，写时使用`Ordering::Release`操作。

获取-释放顺序虽然不像排序一致性顺序那样对全局线程统一排序，但是它让每个线程都按固定的顺序执行。
