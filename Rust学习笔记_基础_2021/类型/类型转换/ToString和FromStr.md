# ToString 和 FromStr

## ToString

将其它类型转换成 `String` 类型，只需要实现那个类型的 `ToString` trait即可。 然而不要直接这么做，应该实现 `fmt::Display` trait，它会自动提供 `ToString`，并且还可以用来打印类型。

```rust
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

&nbsp;

## 解析字符串

经常需要把字符串转成数字，可以使用 `parse` 函数。我们需要提供要转换到的类型，这可以通过不使用类型推断，或者用 `turbofish` 实现。

只要目标类型实现了 `FromStr` trait，就可以用 `parse` 把字符串转成目标类型。标准库已经有很多类型实现了 `FromStr`。如果要转换到用户定义类型，只要手动实现 `FromStr` 就行。

```rust
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
```
