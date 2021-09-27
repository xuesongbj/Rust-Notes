fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(15));

    // Slice(切片)是一种没有所有权的数据类型。切片引用连续的内存分配而不是整个集合。
    // Slice(切片)不能直接创建，而是从现有变量创建的。切片由长度组成，并且可以是可变的或不可变的。
    // Slice(切片)的行为与数组相同。
    let mut numbers: [Option<u16>; 5] = [Some(0); 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4*16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}