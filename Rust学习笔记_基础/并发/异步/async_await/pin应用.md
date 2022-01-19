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
