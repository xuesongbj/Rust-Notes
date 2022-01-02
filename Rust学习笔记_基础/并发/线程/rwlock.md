# 读写锁(rwLock)

读写锁(`RwLock<T>`)和互斥锁(`Mutex<T>`)十分类似，不同在于`RwLock<T>` 对线程进行**读者(Reader)** 和 **写者(Writer)** 的区分，不像 `Mutex<T>` 只能独占访问。

该锁支持多个读者线程和一个写者线程，其中读线程只允许进行只读访问，而写线程只能进行独占写操作。只要线程没有拿到写锁，`RwLock<T>`就允许任意数量的读线程获得读锁。

和 `Mutex<T>` 一样，`RwLock<T>` 也会因为panic而“中毒”。

&nbsp;

###  读写锁实例

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}
```

**读锁和写锁要使用显示作用域块隔开**，这样的话，读锁或写锁才能在离开作用域之后自动释放；否则会引起死锁，因为**读锁和写锁不能同时存在**。
