mod probabilities;

#[derive(Deserialize,Clone)]
pub struct Weapon {
    pub name: String,
    pub reach: i32,
    pub attacks: f64,
    pub hit: i32,
    pub wound: i32,
    pub rend: i32,
    pub damage: f64
}

impl Weapon {

    pub fn precision(&self) -> f64 {
        self.attacks * probabilities::attack_precision(self.hit, self.wound)
    }

    pub fn threat(&self) -> f64 {
        self.precision() * self.damage
    }

    pub fn unsaved(&self, save : i32) -> f64 {
        self.attacks * probabilities::attack_unsaved(self.hit, self.wound, self.rend, save)
    }

    pub fn expected_damage(&self, save : i32) -> f64 {
        self.unsaved(save) * self.damage
    }

    pub fn merge(&self, w : &Weapon) -> Weapon {
        Weapon {
            name: self.name.clone(),
            reach: self.reach + w.reach,
            attacks: self.attacks + w.attacks,
            hit: self.hit + w.hit,
            wound: self.wound + w.wound,
            rend: self.rend + w.rend,
            damage: self.damage + w.damage
        }
    }

}

#[cfg(test)]
mod tests {

    macro_rules! weapon1 {
        () => (
            super::Weapon{
                name: String::from(""),
                reach: 1, attacks: 1.0, hit: 5, wound: 6, rend: 0, damage: 2.0}
        )
    }

    macro_rules! weapon2 {
        () => (
            super::Weapon{
                name: String::from(""),
                reach: 1, attacks: 2.0, hit: 6, wound: 4, rend: -2, damage: 9.0}
        )
    }

    #[test]
    fn test_weapon_precision() {
        assert_approx_eq!(
            weapon1!().precision(),
            2.0/6.0 * 1.0/6.0);
        assert_approx_eq!(
            weapon2!().precision(),
            2.0 * 3.0/6.0 * 1.0/6.0);
    }

    #[test]
    fn test_weapon_threat() {
        assert_approx_eq!(
            weapon1!().threat(),
            2.0/6.0 * 1.0/6.0 * 2.0);
        assert_approx_eq!(
            weapon2!().threat(),
            2.0 * 3.0/6.0 * 1.0/6.0 * 9.0);
    }

    #[test]
    fn test_weapon_unsaved() {
        assert_approx_eq!(
            weapon1!().unsaved(5),
            2.0/6.0 * 1.0/6.0 * 4.0/6.0);
        assert_approx_eq!(
            weapon2!().unsaved(2),
            2.0 * 3.0/6.0 * 1.0/6.0 * 0.5);
    }
}
