mod dice;

pub fn check(target : i32) -> f64 {
    dice::roll_over(target)
}

#[test]
fn test_check() {
    assert_approx_eq!(check(4), dice::roll_over(4));
    assert_approx_eq!(check(6), dice::roll_over(6));
    assert_approx_eq!(check(1), dice::roll_over(1));
}
