# Cow

`Cow` 是一种智能指针类型，提供两种版本的字符串，它表示在写入的基础上复制(Clone on write, Cow)，通过 `use std::borrow::Cow;` 引入。

`Cow` 类型具体签名:

```rust
pub enum Cow<'a, B: ?Sized + 'a>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

&nbsp;

`Cow`有两个变体:

* `Borrowed`: 表示某种类型 `B`的借用。
* `Owned`: 所有权变体，包括该类型的所有权版本。

&nbsp;

针对以上两种变体，`cow` 可用于以下两种场景:

* 对此对象的不可变访问（比如可直接调用此对象原有的不可变方法）。
* 如果遇到需要修改此对象，或者需要获得此对象的所有权情况，`Cow` 提供方法做克隆处理，并避免多次重复克隆。

&nbsp;

## Cow 设计目的

`Cow`的设计目的是提高性能(减少复制)同时增加灵活性，因为大部分情况下，因为大部分情况下，业务场景都是读多写少。利用 `Cow` 可以用统一，规范的形式实现，需要写的时候才做一次对象复制。这样就可能会大大减少复制的次数。

&nbsp;

## Cow 使用要点

使用 `Cow` 需要掌握以下要点:

1. `Cow<T>` 能直接调用 `T` 的不可变方法，因为 `Cow` 这个枚举，实现了 `Deref`；
2. 在需要写 `T` 的时候，可以使用 `.to_mut()` 方法得到一个具有所有权的值的可变借用。
    1. 注意，调用 `.to_mut()` 不一定产生克隆。
    2. 在已经具有所有权的情况下，调用 `.to_mut()` 有效，但是不会产生新的克隆。
    3. 多次调用 `.to_mut()` 只会产生一次克隆。
3. 在需要写 `T` 的时候，可以使用 `.into_owned()` 创建新的拥有所有权的对象，这个过程往往意味着内存拷贝并创建新对象；
    1. 如果之前 `Cow` 中的值是借用状态，调用此操作将执行克隆。
    2. 本方法，参数是 `self` 类型，它会 "吃掉" 原先的那个对象，调用之后原先的对象的生命周期就截止了，在 `Cow` 上不能调用多次；

&nbsp;

### to_mut 例子

```rust
use std::borrow::Cow;


fn main() {
    let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    let hello = cow.to_mut();

    assert_eq!(hello, &[1, 2, 3]);
}
```

&nbsp;

### into_owned 例子

```rust
use std::borrow::Cow;


fn main() {
    let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    let hello = cow.into_owned();

    assert_eq!(vec![1, 2, 3], hello);
}
```
