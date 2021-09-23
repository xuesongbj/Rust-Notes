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
