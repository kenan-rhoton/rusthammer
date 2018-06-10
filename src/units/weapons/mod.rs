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

#[derive(Deserialize,Debug,PartialEq,Default,Clone,Copy)]
pub struct AttackResult {
    pub range: i32,
    pub value: f64
}

impl AttackResult {
    pub fn total(result_list : Vec<AttackResult>) -> f64 {
        let (ranged, melee) : (Vec<AttackResult>,Vec<AttackResult>)
             = result_list.iter().partition(|x| x.range > 3);

        ranged.iter().min_by_key(|x| x.range).unwrap_or(&AttackResult{range: 0, value:0.0}).value +
            melee.iter().min_by_key(|x| x.range).unwrap_or(&AttackResult{range: 0, value:0.0}).value
    }
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

