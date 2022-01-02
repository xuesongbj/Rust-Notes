fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);     // mut vec0 borrow!!! 借用!!!

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);  // vec0 可以继续使用!!!

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec<'a>(vec: &'a mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    let vec1 = vec.clone();
    vec1
}