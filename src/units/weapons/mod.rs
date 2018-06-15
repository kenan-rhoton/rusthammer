mod unit_extension;

#[derive(Deserialize,Debug,PartialEq,Default,Clone)]
pub struct Weapon {
    pub name: String,
    #[serde(default)]
    pub reach: i32,
    #[serde(default)]
    pub attacks: f64,
    #[serde(default)]
    pub hit: i32,
    #[serde(default)]
    pub wound: i32,
    #[serde(default)]
    pub rend: i32,
    #[serde(default)]
    pub damage: f64,
    #[serde(default)]
    pub extra: Vec<Weapon>,
    #[serde(default = "default_quantity")]
    pub quantity: f64
}

fn default_quantity() -> f64 {
    1.0
}

impl Weapon {

    pub fn merge_weapon(weapon_list : &Vec<Weapon>, target : &Weapon) -> Vec<Weapon> {
        weapon_list.iter().map(|w| {
            if w.name == target.name {
                w.merge(target)
            } else {
                w.clone()
            }
        })
        .collect()
    }

    pub fn merge(&self, w : &Weapon) -> Weapon {
        Weapon {
            name: w.name.clone(),
            reach: self.reach + w.reach,
            attacks: self.attacks + w.attacks,
            hit: self.hit + w.hit,
            wound: self.wound + w.wound,
            rend: self.rend + w.rend,
            damage: self.damage + w.damage,
            quantity: if w.quantity > 0.0 { w.quantity }
                        else { self.quantity },
            extra: if w.extra.len() > 0 {
                w.extra.clone()
            } else {
                self.extra.clone()
            }
        }
    }

}

