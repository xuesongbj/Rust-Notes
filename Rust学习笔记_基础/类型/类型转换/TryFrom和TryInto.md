# TryFrom 和 TryInto

类似于 `From` 和 `Into`，`TryFrom` 和 `TryInto` 是类型转换的通用 trait。不同的是 `From/Into` 的是，`TryFrom` 和 `TryInfo` trait用于易出错的转换，所以返回值是 `Result` 类型。

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0  {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```
