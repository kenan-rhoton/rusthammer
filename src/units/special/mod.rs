use super::Unit;
use ::units::weapons::Weapon;

fn get_number(save_string : &String) -> i32 {
    let v : Vec<&str> = save_string.matches(char::is_numeric).collect();
    v[0].parse::<i32>().unwrap()
}

impl Unit {

    fn check_special(&self, name : &str) -> bool {
        self.special.iter().any(|s| s == name)
    }

    pub fn special_save(&self) -> f64 {
        let super_save = self.special.iter().find(
            |x| x.contains("Supersave"));
        match super_save {
            None => 1.0,
            Some(x) => self.to_roll(get_number(x), "Supersave"),
        }
    }

    pub fn special_roll(&self, target : i32, name : &str) -> f64 {
        if self.check_special(&format!("Reroll 1s on {}", name)) {
            1.0/6.0
        } else if self.check_special(&format!("Reroll Failed {}s", name)) {
            (target - 1) as f64/6.0
        } else {
            0.0
        }
    }

    pub fn affect_opponent_weapon(&self, weapon : &Weapon) -> Weapon {
        if self.check_special("Half damage taken") {
            Weapon {
                damage: if weapon.damage > 1.0 {
                    weapon.damage / 2.0
                } else {
                    weapon.damage
                },
                ..weapon.clone()
            }
        } else {
            Weapon {
                ..weapon.clone()
            }
        }
    }

}
