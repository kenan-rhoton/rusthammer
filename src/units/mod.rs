mod weapons;
extern crate serde_json;
use std;

#[derive(Deserialize,Default,Clone)]
pub struct Unit {
    pub name: String,
    pub bravery: i32,
    pub movement: i32,
    pub save: i32,
    pub size: i32,
    pub weapons: Vec<weapons::Weapon>,
    pub wounds: i32,
    #[serde(default)]
    pub retry: Vec<weapons::WeaponOption>,
    #[serde(default)]
    pub special: Vec<String>
}

impl Unit {

    pub fn precision(&self) -> f64 {
        self.size as f64 *
            self.weapons.iter().fold(0.0, |acc, x| acc + x.precision())
    }

    pub fn threat(&self) -> f64 {
        self.size as f64 *
            self.weapons.iter().fold(0.0, |acc, x| acc + x.threat())
    }

    fn special_saves(&self, rend : i32) -> f64 {
        if self.special.iter().any(|s| s == "Reroll 1s on Save") {
            1.0 - 1.0/6.0 * weapons::probabilities::check(self.save - rend) as f64
        } else if self.special.iter().any(|s| s == "Reroll Failed Saves") {
            1.0 - weapons::probabilities::check(self.save - rend) as f64
        } else {
            1.0
        }
    }

    pub fn unsaved(&self, opponent : &Unit) -> f64 {
        self.size as f64 *
            self.weapons.iter().fold(0.0, |acc, x| {
                println!("POTATOES {}", opponent.special_saves(x.rend));
                opponent.special_saves(x.rend) *
                    x.unsaved(opponent.save) + acc
            })
    }

    pub fn expected_damage(&self, opponent : &Unit) -> f64 {
        self.size as f64 *
            self.weapons.iter().fold(0.0, |acc, x| {
                opponent.special_saves(x.rend) *
                    x.expected_damage(opponent.save) + acc
            })
    }

    pub fn merge(&self, weapon : &weapons::WeaponOption) -> Unit {
        Unit {
            weapons: self.weapons.iter().map(|w| {
                if w.name == weapon.replace {
                    w.merge(weapon)
                } else {
                    (*w).clone()
                }
            }).collect(),
            ..(*self).clone()
        }
    }

    pub fn from_file(filename : String) -> Result<Unit, Box<std::error::Error>> {
        let file = std::fs::File::open(filename)?;

        let u = serde_json::from_reader(file)?;

        Ok(u)
    }

}

#[cfg(test)]
mod tests {

    macro_rules! simple_unit {
        () => (
            super::Unit {
                name: String::from("Simple"),
                save: 6,
                size: 1,
                wounds: 4,
                bravery: 6,
                movement: 4,
                weapons: vec![
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 2, attacks: 4.0, hit: 3, wound: 3, rend: -1, damage: 3.0, extra: vec![]
                    }
                ],
                retry: vec![],
                special: vec![]
            }
            )
    }

    macro_rules! complex_unit {
        () => (
            super::Unit {
                name: String::from("Complex"),
                save: 3,
                size: 9,
                wounds: 4,
                bravery: 6,
                movement: 4,
                weapons: vec![
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 1, attacks: 3.0, hit: 3, wound: 3, rend: 0, damage: 1.0, extra: vec![]
                    },
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 2, attacks: 6.0, hit: 4, wound: 3, rend: -2, damage: 3.0, extra: vec![]
                    },
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 1, attacks: 3.5, hit: 3, wound: 2, rend: -3, damage: 2.0, extra: vec![]
                    }
                ],
                retry: vec![],
                special: vec![String::from("Reroll 1s on Save")]
            }
        )
    }

    #[test]
    fn test_precision() {
        assert_approx_eq!(
            simple_unit!().precision(),
            4.0 * (4.0/6.0) * (4.0/6.0));

        assert_approx_eq!(
            complex_unit!().precision(),
            9.0 * (3.0 * (4.0/6.0) * (4.0/6.0) + 6.0 * 0.5 * (4.0/6.0) + 3.5 * (4.0/6.0) * (5.0/6.0)));
    }

    #[test]
    fn test_threat() {
        assert_approx_eq!(
            simple_unit!().threat(),
            4.0 * (4.0/6.0) * (4.0/6.0) * 3.0);

        assert_approx_eq!(
            complex_unit!().threat(),
            9.0 * (
                3.0 * (4.0/6.0) * (4.0/6.0) +
                6.0 * 0.5 * (4.0/6.0) * 3.0 +
                3.5 * (4.0/6.0) * (5.0/6.0) * 2.0));
    }

    #[test]
    fn test_unsaved() {
        assert_approx_eq!(
            simple_unit!().unsaved(&complex_unit!()),
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5 * (1.0 - 1.0/6.0 * 0.5));

        assert_approx_eq!(
            complex_unit!().unsaved(&simple_unit!()),
            9.0 * (
                3.0 * (4.0/6.0) * (4.0/6.0) * (5.0/6.0)+
                6.0 * 0.5 * (4.0/6.0) +
                3.5 * (4.0/6.0) * (5.0/6.0)));
    }

    #[test]
    fn test_expected_damage() {
        assert_approx_eq!(
            simple_unit!().expected_damage(&complex_unit!()),
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5 * 3.0 * (1.0 - 1.0/6.0 * 0.5));

        assert_approx_eq!(
            complex_unit!().expected_damage(&simple_unit!()),
            9.0 * (
                3.0 * (4.0/6.0) * (4.0/6.0) * (5.0/6.0)+
                6.0 * 0.5 * (4.0/6.0) * 3.0 +
                3.5 * (4.0/6.0) * (5.0/6.0) * 2.0 ));
    }
}
