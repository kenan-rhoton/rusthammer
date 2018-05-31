mod dice;

fn check(target : i32) -> f64 {
    dice::roll_over(target)
}

#[test]
fn test_check() {
    assert_approx_eq!(check(4), dice::roll_over(4));
    assert_approx_eq!(check(6), dice::roll_over(6));
    assert_approx_eq!(check(1), dice::roll_over(1));
}


pub fn attack_precision(hit : i32, wound : i32) -> f64 {
    check(hit) * check(wound)
}

#[test]
fn test_attack_precision() {
    assert_approx_eq!(attack_precision(6, 6), (1.0/6.0) * (1.0/6.0));
    assert_approx_eq!(attack_precision(3, 6), (4.0/6.0) * (1.0/6.0));
    assert_approx_eq!(attack_precision(1, 1), 1.0);
}

pub fn attack_unsaved(hit : i32, wound : i32, rend : i32, save : i32) -> f64{
    check(hit) * check(wound) * (1.0 - check(save - rend))
}

#[test]
fn test_attack_unsaved() {
    assert_approx_eq!(attack_unsaved(6, 6, 0, 5), (1.0/6.0) * (1.0/6.0) * (4.0/6.0));
    assert_approx_eq!(attack_unsaved(3, 6, -2, 4), (4.0/6.0) * (1.0/6.0) * (5.0/6.0));
    assert_approx_eq!(attack_unsaved(1, 1, 2, 1), 0.0);
}
