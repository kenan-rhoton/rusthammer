mod dice;

fn check(target : i32) -> f64 {
    if target < 2 {
        dice::roll_over(2)
    } else {
        dice::roll_over(target)
    }
}

#[test]
fn test_check() {
    assert_eq!(check(4), dice::roll_over(4));
    assert_eq!(check(6), dice::roll_over(6));
    assert_eq!(check(1), dice::roll_over(2));
}


fn precision(hit : i32, wound : i32) -> f64 {
    check(hit) * check(wound)
}

#[test]
fn test_precision() {
    assert_eq!(precision(6, 6), (1.0/6.0) * (1.0/6.0));
    assert_eq!(precision(3, 6), (4.0/6.0) * (1.0/6.0));
    assert_eq!(precision(1, 1), (5.0/6.0) * (5.0/6.0));
}
