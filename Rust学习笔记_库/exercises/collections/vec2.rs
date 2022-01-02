fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    // iter_mut: 返回一个允许修改的迭代器
    for i in v.iter_mut() {
        *i = *i*2;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_looop() {
        // filter: 创建一个迭代器
        // take: 元素个数限制
        // collect: 将迭代器转换成集合 
        let v: Vec<i32> = (1..).filter(|x| x%2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x*2).collect::<Vec<i32>>());
    }
}