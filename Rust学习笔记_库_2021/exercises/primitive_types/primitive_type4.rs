#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..=3];

    assert_eq!([2, 3, 4], nice_slice);
}