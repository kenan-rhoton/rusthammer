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

impl super::Unit {

    pub fn to_roll(&self, target : i32, name : &str) -> f64{
        check(target) +
            if self.special.iter().any(|s| s == &format!("Reroll 1s on {}", name)) {
                1.0/6.0 * check(target)
            } else if self.special.iter().any(|s| s == &format!("Reroll Failed {}s", name)) {
                (target - 1) as f64/6.0 * check(target)
            } else {
                0.0
            }
    }

    pub fn to_hit(&self, hit : i32) -> f64 {
        self.to_roll(hit, "Hit")
    }

    pub fn to_wound(&self, wound : i32) -> f64 {
        self.to_roll(wound, "Wound")
    }

    pub fn to_save(&self, rend : i32) -> f64 {
        self.to_roll(self.save - rend, "Save")
    }

}
