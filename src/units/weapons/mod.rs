#[derive(Deserialize,Debug,PartialEq,Default,Clone)]
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

impl Weapon {

    pub fn merge(&self, w : &Weapon) -> Weapon {
        Weapon {
            name: w.name.clone(),
            reach: self.reach + w.reach,
            attacks: self.attacks + w.attacks,
            hit: self.hit + w.hit,
            wound: self.wound + w.wound,
            rend: self.rend + w.rend,
            damage: self.damage + w.damage,
            extra: self.extra.clone()
        }
    }

}

impl super::Unit {

    fn each_weapon<C>(&self, mut action : C) -> f64 where C: FnMut(&Weapon) -> f64 {
        self.size as f64 *
            self.weapons.iter().fold(0.0, |acc, x| {
                acc + action(x) + x.extra.iter().fold(0.0, |acc, x| acc + action(x))
            })
    }

    pub fn precision(&self) -> f64 {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound)
        })
    }

    pub fn threat(&self) -> f64 {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) * x.damage
        })
    }

    pub fn unsaved(&self, opponent : &super::Unit) -> f64 {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend))
        })
    }

    pub fn expected_damage(&self, opponent : &super::Unit) -> f64 {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend)) * x.damage
        })
    }
}
