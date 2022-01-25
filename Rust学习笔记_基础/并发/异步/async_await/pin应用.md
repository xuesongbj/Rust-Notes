# pin应用

## 异步编程中Pin应用

生成器由编译器生成相应的结构体来记录状态，当生成器包含对本地变量的引用时，该结构体会生成一种**自引用结构体(Self-referential Struct)**

* 源代码

```rust
let mut gen = || {
    yield 1;
    yield 2;
    yield 3;
    return 4;
};
```

* 生成器实例生成代码

```rust
enum __Gen<'a> {
    Start,
    State(State1<'a>),
    State(State2),
    State(State3),
    Done
}

// 生成自引用结构体
// ref_x是对x的引用
struct State1<'a> { x: u64, ref_x: &'a u64 }

impl<'a> Generator for __Gen<'a> {
    type Yield = u64;
    type Return = u64;

    unsafe fn resume(&mut self) -> GeneratorState<u64, u64> {
        // 移动指针的内存位置
        match std::mem::replace(self, __Gen::Done) {       // replace(dest, src)
            __Gen::Start => {
                // 生成一个自引用结构体实例
                let x = 1;
                let state1 = State1{ x: x, ref_x: &x };
                *self = __Gen::State(state1);

                // 挂起生成器(对应 yield 1)，等待再次触发resume
                GeneratorState::Yielded(1)
            }
            __Gen::State1(State1{ x: 1, ref_x: &1}) => {
                *self = __Gen::State2(State2{x: 2});
                GeneratorState::Yielded(2)
            }

            // ...
        }
    }
}
```

如上实例，`std::mem::replace(self, __Gen::Done)` 会发生移动指针内存位置，将 `State1`替换为`State2`。意味着 `State1`的所有权已经发生了转移。`State1` 内存位置的改变会影响到字段`x`的位置，而此时其内部的字段`ref_x`还在引用字段`x`的值，这就造成了**悬垂指针**。

针对以上问题，可以使用 `Pin`方案进行解决。

```rust
use std::pin::Pin;
use std::marker::{PhantomPinned, Unpin};
use std::ptr::NonNull;

// 定义自引用结构体
struct Unmovable {
    data: String,
    
    // 创建自引用结构体使用的; 该字段会引用 data字段
    slice: NonNull<String>,
    
    // 自定义结构体有了PhantomPinned属性，则该结构体即可实现 !Unpin
    _pin: PhantomPinned,
}

// 手动为 `Unmovable` 实现 `Unpin`
impl Unpin for Unmovable {}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        
        // NonNull::from 函数将boxed实例的data字段转换为NonNull指针，绑定给slice变量
        let slice = NonNull::from(&boxed.data);
        unsafe {
            // 通过 Pin::as_mut函数从&mut boxed得到一个 Pin<&mut Self>类型的值 mut_ref(Pin<&mut Unmovable>)
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);

            // 将slice字段的值赋值为slice变量，创建一个自引用结构体的实例
            // Pin::get_unchecked_mut： 获取对Pin内部数据的可变引用。
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }   
}

fn main() {
    // 使用new方法创建了Unmovable实例unmoved，然后将其赋值给新的变量 `still_unmoved`,
    // 目的是想要转移unmoved的所有权。
    
    // 从如下断言可得知，该结构体实例被转移后，字段的地址并没有被改变。slice字段引用的data字段最初的地址，
    // 现在断言相等，证明data字段的地址没有变。

    // Pin<T> 类型起作用了
    let unmoved = Unmovable::new("hello".to_string());
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
 
    // 使用std::mem::swap交换new_unmoved 和 still_unmoved 的引用地址
    // 可以正常被编译通过，是因为上面手工为Unmovable结构体实现了Unpin
    let mut new_unmoved = Unmovable::new("world".to_string());
    std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
```

在之前异步编程中的`GenFuture` 实现 `poll`方法时，使用了 `Pin<&mut Self>`，就是确保该类型最终生成的生成器不会出现因为自引用结构体而产生未定义行为的情况。**然后在需要的时候，使用`Pin::get_mut_unchecked`函数获取其包含的可变借用。**

### async/await pin 的应用

```rust
use futures::{
    executor::ThreadPool,
};
use std::future::Future;
use std::pin::Pin;
use std::task::*;

pub struct AlmostReady {
    ready: bool,
    value: i32,
}

pub fn almost_ready(value: i32) -> AlmostReady {
    AlmostReady{ ready: false, value }
}

impl Future for AlmostReady {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value + 1)
        } else {
            unsafe { Pin::get_unchecked_mut(self).ready = true; }
            let waker = ctx.waker().clone();
            waker.wake();
            Poll::Pending
        }
    }
}

fn main() {
    let pool = ThreadPool::new().unwrap();
    let future = async {
        println!("howdy!");
        let x = almost_ready(5).await;
        println!("done: {:?}", x);
    };
    pool.spawn_ok(future);
}
```
