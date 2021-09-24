# collections

## HashMap

检查两个`HashMap`是否具有相同的key/value.

```rust
// 1. 使用Iterator::all会大大缩短代码
// 2. 使用HashMap::contains_key优于检查的结果HashMap::get
// 3. 首先检查长度，因为这是一种最高效的测试，应首先进行

fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}
```

&nbsp;

## vec

如下是 `vec!` 的具体实现(简化版本)：

```rust
// macro_rules! 定义了名字vec后面跟上大括号
// ( $( $x:expr ), *), 这是一个模式(pattern)，如果模式匹配了后面相关的代码就会执行; 紧随逗号之后的，说明该模式匹配零个或者多个之前的任何模式
// vec![1, 2, 3]; 调用宏时，$x模式与三个表达式1、2和 3进行了3次匹配
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), *) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}
```
