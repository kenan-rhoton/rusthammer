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

#[derive(Deserialize,Debug,PartialEq,Default,Clone,Copy)]
pub struct AttackResult {
    pub range: i32,
    pub value: f64
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

    fn each_weapon<C>(&self, mut action : C) -> Vec<AttackResult>
        where C: FnMut(&Weapon) -> f64 {
            self.weapons.iter().fold(Vec::new(), |mut result_list, weapon| {
                let weapon_value = self.size as f64 * (action(weapon) + 
                    weapon.extra.iter().fold(0.0, |acc, x| acc + action(x)));

                if !result_list.iter().any(|x| x.range == weapon.reach) {
                    let val = result_list.iter()
                        .filter(|x| (x.range > 3) == (weapon.reach > 3))
                        .min_by_key(|x| x.range)
                        .unwrap_or(&AttackResult{range:0,value:0.0})
                        .value;

                        result_list.push(
                            AttackResult{
                            range: weapon.reach,
                            value: val
                        })
                }

                result_list.iter().map(|x| {
                    if x.range <= weapon.reach &&
                        ((weapon.reach > 3) == (x.range > 3)) {
                            AttackResult{
                                value: x.value + weapon_value,
                                range: x.range
                            }
                    } else {
                        (*x).clone()
                    }
                }).collect()
            })
    }

    pub fn precision(&self) -> Vec<AttackResult> {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound)
        })
    }

    pub fn threat(&self) -> Vec<AttackResult> {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) * x.damage
        })
    }

    pub fn unsaved(&self, opponent : &super::Unit) -> Vec<AttackResult> {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend))
        })
    }

    pub fn expected_damage(&self, opponent : &super::Unit) -> Vec<AttackResult> {
        self.each_weapon(|x| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend)) * x.damage
        })
    }
}
