# phantom type

虚类型(phantom type) 参数是一种在运行时不出现，仅在编译时进行静态检查的类型参数。

可以用额外的泛型类型参数指定数据类型，该类型可以充当标记，也可以供编译时类型检查使用。这些额外的参数没有存储值，也没有运行时行为。

```rust
use std::marker::PhantomData;

// 该虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`
struct PhantomTuple<A, B>(A, PhantomDta<B>);

// 该虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B` 
struct PhantomStruct<A, B>{ first: A, phantom: PhantomData<B>}
```

对于泛型 `A` 会分配存储空间，但 `B` 不会。因此，`B` 不能参与运算。

