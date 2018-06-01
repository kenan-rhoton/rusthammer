pub mod probabilities;

#[derive(Deserialize,Default,Clone)]
pub struct Weapon {
    pub name: String,
    pub reach: i32,
    pub attacks: f64,
    pub hit: i32,
    pub wound: i32,
    pub rend: i32,
    pub damage: f64,
    #[serde(default)]
    pub extra: Vec<Weapon>
}

#[derive(Deserialize,Clone)]
pub struct WeaponOption {
    pub name: String,
    pub replace: String,
    pub reach: i32,
    pub attacks: f64,
    pub hit: i32,
    pub wound: i32,
    pub rend: i32,
    pub damage: f64,
    #[serde(default)]
    pub extra: Vec<Weapon>
}

impl Weapon {

    pub fn merge(&self, w : &WeaponOption) -> Weapon {
        Weapon {
            name: w.name.clone(),
            reach: self.reach + w.reach,
            attacks: self.attacks + w.attacks,
            hit: self.hit + w.hit,
            wound: self.wound + w.wound,
            rend: self.rend + w.rend,
            damage: self.damage + w.damage,
            extra: w.extra.clone()
        }
    }

}
