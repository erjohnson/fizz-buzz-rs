fn div_by_three(num: int) -> bool {
  num % 3 == 0
}

#[test]
fn test_div_by_three() {
    let num = 1;
    if div_by_three(num) {
        fail!("{} is not a multiple of 3", num);
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        fail!("Three should be three");
    }
}
