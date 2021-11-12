# Pin 和 Unpin

## Pin

`Pin` 是一个智能指针，它内部包含了另一个指针 `P`，只要`P`指针指向的内容(`T`)没有实现 `Unpin`，则可以保证 `T` 永远不会被移动(`move`)。`Pin` 单词很形象的表示 `Pin` 就像钉子一样可以把 `T` 钉住。所以 `Pin`一般来说用 `Pin<P<T>>` 这种方式表示(`P` 是Pointer 的缩写， `T` 是 Type的缩写)。

```rust
/// A pinned pointer.
///
/// This is a wrapper around a kind of pointer which makes that pointer "pin" its
/// value in place, preventing the value referenced by that pointer from being moved
/// unless it implements [`Unpin`].
#[stable(feature = "pin", since = "1.33.0")]
#[lang = "pin"]
#[fundamental]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Pin<P> {
    pointer: P,
}

#[stable(feature = "pin", since = "1.33.0")]
impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        Pin::get_ref(Pin::as_ref(self))
    }
}

#[stable(feature = "pin", since = "1.33.0")]
impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        Pin::get_mut(Pin::as_mut(self))
    }
}
```

* `Pin`是一个智能指针(实现了`Deref`和`DerefMut`)。
* `Pin`包裹的内容只能是指针，不能是其它类型。比如`Pin<u32>` 没有意义。
* `Pin`具有"钉住"`T`不能移动的功能，这个功能是否有效取决于 `T` 是否 `impl Unpin`。如果 `T` 实现了 `Unpin`，`Pin` 的“钉住”功能完全失效了，此时 `Pin<P<T>>` 等价于 `P<T>`。
* `Unpin` 是一个 `auto trait`，编译器默认会给所有类型实现 `Unpin`。但`PhantomPinned`和编译为`async/await` desugar(脱糖)之后生成的 `impl Future`的结构体除外，它们实现的是`!Unpin`。
* 所以 `Pin<P<T>>`默认情况下的 "钉住"功能是不生效的，只针对上面说的这几个 `impl !Unpin`情况有效。

> Pin主要是为了解决 `async/await` 自动生成 Future的问题。问题就是自引用，移动自引用结构体会造成指针失效。

&nbsp;

### move

所有权转移的这个过程就是`move`。

```rust
fn main() {
  let mut s1 = String::from("Hello");
  let s2 = s1; // s1的所有权转移给了s2，这里发生了move
  // let s3 = s1; // s1的所有权以及转移走了，不能再move，否则会报错：error[E0382]: use of moved value: `s1`
}
```

* `let s2 = s1;` 实现原理:
