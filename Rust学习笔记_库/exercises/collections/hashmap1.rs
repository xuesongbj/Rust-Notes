use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 创建一个空HashMap
    let mut basket = HashMap::new();

    // 向HashMap插入K/V对
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("orange"), 2);

    basket
}

#[cfg(test)]
mod tests {
    // 引入当前模块(mod)的父模块所有资源
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        // basket.len() 返回HashMap内元素数
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();

        // basket.values() 返回HashMap元素的迭代器，
        // sum::<u32>() 获取迭代器值
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

/*
数据结构:

type = struct std::collections::hash::map::HashMap<alloc::string::String, i32, std::collections::hash::map::RandomState> {
  base: hashbrown::map::HashMap<alloc::string::String, i32, std::collections::hash::map::RandomState>,
}

内存布局:

basket = std::collections::hash::map::HashMap<alloc::string::String, i32, std::collections::hash::map::RandomState> {
  base: hashbrown::map::HashMap<alloc::string::String, i32, std::collections::hash::map::RandomState> {
    hash_builder: std::collections::hash::map::RandomState {
      k0: 17202417722039394206,
      k1: 10650482839738393711
    },
    table: hashbrown::raw::RawTable<(alloc::string::String, i32)> {
      bucket_mask: 3,
      ctrl: core::ptr::non_null::NonNull<u8> {
        pointer: 0x5555555adaa0 "\377(Q ", '\377' <repeats 13 times>, "(Q \000"
      },
      growth_left: 0,
      items: 3,
      marker: core::marker::PhantomData<(alloc::string::String, i32)>
    }
  }
}
*/