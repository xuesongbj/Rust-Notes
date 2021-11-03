fn calculate_apple_price(num: i32)  ->i32 {
    if num <= 40 {
        return num * 2;
    }
    num
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(40);
    let price3 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}