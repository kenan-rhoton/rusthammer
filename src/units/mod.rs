mod weapons;
extern crate serde_json;
use std;

#[derive(Deserialize)]
pub struct Unit {
    pub save: i32,
    pub weapons: Vec<weapons::Weapon>
}

impl Unit {

    pub fn precision(&self) -> f64 {
        self.weapons.iter().fold(0.0, |acc, x| acc + x.precision())
    }

    pub fn threat(&self) -> f64 {
        self.weapons.iter().fold(0.0, |acc, x| acc + x.threat())
    }

    pub fn unsaved(&self, opponent : &Unit) -> f64 {
        self.weapons.iter().fold(0.0, |acc, x| acc + x.unsaved(opponent.save))
    }

    pub fn expected_damage(&self, opponent : &Unit) -> f64 {
        self.weapons.iter().fold(0.0, |acc, x| acc + x.expected_damage(opponent.save))
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
                save: 6,
                weapons: vec![
                    super::weapons::Weapon {
                        name: String::from(""),
                        reach: 2, attacks: 4.0, hit: 3, wound: 3, rend: -1, damage: 3.0
                    }
                ]
            }
        )
    }

    macro_rules! complex_unit {
        () => (
            super::Unit {
                save: 3,
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
                ]
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
            3.0 * (4.0/6.0) * (4.0/6.0) + 6.0 * 0.5 * (4.0/6.0) + 3.5 * (4.0/6.0) * (5.0/6.0));
    }

    #[test]
    fn test_threat() {
        assert_approx_eq!(
            simple_unit!().threat(),
            4.0 * (4.0/6.0) * (4.0/6.0) * 3.0);

        assert_approx_eq!(
            complex_unit!().threat(),
            3.0 * (4.0/6.0) * (4.0/6.0) +
            6.0 * 0.5 * (4.0/6.0) * 3.0 +
            3.5 * (4.0/6.0) * (5.0/6.0) * 2.0);
    }

    #[test]
    fn test_unsaved() {
        assert_approx_eq!(
            simple_unit!().unsaved(&complex_unit!()),
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5);

        assert_approx_eq!(
            complex_unit!().unsaved(&simple_unit!()),
            3.0 * (4.0/6.0) * (4.0/6.0) * (5.0/6.0)+
            6.0 * 0.5 * (4.0/6.0) +
            3.5 * (4.0/6.0) * (5.0/6.0) );
    }

    #[test]
    fn test_expected_damage() {
        assert_approx_eq!(
            simple_unit!().expected_damage(&complex_unit!()),
            4.0 * (4.0/6.0) * (4.0/6.0) * 0.5 * 3.0);

        assert_approx_eq!(
            complex_unit!().expected_damage(&simple_unit!()),
            3.0 * (4.0/6.0) * (4.0/6.0) * (5.0/6.0)+
            6.0 * 0.5 * (4.0/6.0) * 3.0 +
            3.5 * (4.0/6.0) * (5.0/6.0) * 2.0 );
    }
}
