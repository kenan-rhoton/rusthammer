pub fn roll_over(num : i32) -> f64 {
    if num > 6 {
        0.0
    } else if num <= 1 {
        1.0
    } else {
        (7 - num) as f64 / 6.0
    }
}

#[test]
fn test_roll_over() {
    assert_eq!(roll_over(0),1.0);
    assert_eq!(roll_over(1),1.0);
    assert_eq!(roll_over(4),0.5);
    assert_eq!(roll_over(6),1.0/6.0);
    assert_eq!(roll_over(7),0.0);
}


