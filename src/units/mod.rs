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
    pub retry: Vec<weapons::WeaponOption>
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

    pub fn unsaved(&self, opponent : &Unit) -> f64 {
        self.size as f64 *
        self.weapons.iter().fold(0.0, |acc, x| acc + x.unsaved(opponent.save))
    }

    pub fn expected_damage(&self, opponent : &Unit) -> f64 {
        self.size as f64 *
        self.weapons.iter().fold(0.0, |acc, x| acc + x.expected_damage(opponent.save))
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
                        reach: 2, attacks: 4.0, hit: 3, wound: 3, rend: -1, damage: 3.0
                    }
                ],
                retry: vec![]
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
                        reach: 1, attacks: 3.0, hit: 3, wound: 3, rend: 0, damage: 1.0
                    },
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 2, attacks: 6.0, hit: 4, wound: 3, rend: -2, damage: 3.0
                    },
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 1, attacks: 3.5, hit: 3, wound: 2, rend: -3, damage: 2.0
                    }
                ],
                retry: vec![]
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
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5);

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
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5 * 3.0);

        assert_approx_eq!(
            complex_unit!().expected_damage(&simple_unit!()),
            9.0 * (
                3.0 * (4.0/6.0) * (4.0/6.0) * (5.0/6.0)+
                6.0 * 0.5 * (4.0/6.0) * 3.0 +
                3.5 * (4.0/6.0) * (5.0/6.0) * 2.0 ));
    }
}
