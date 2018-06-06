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
            quantity: self.quantity,
            extra: self.extra.clone()
        }
    }

}

fn value_for_lowest_range(result_list : &Vec<AttackResult>, reach : i32) -> f64 {
    result_list.iter()
        .filter(|x| (x.range > 3) == (reach > 3))
        .min_by_key(|x| x.range)
        .unwrap_or(&AttackResult{range:0,value:0.0})
        .value
}

fn add_result_at_range(result_list : &Vec<AttackResult>, reach: i32, value : f64) -> Vec<AttackResult> {
    result_list.iter().map(|x| {
        if x.range <= reach &&
            ((reach > 3) == (x.range > 3)) {
                AttackResult{value: x.value + value,range: x.range}
            } else {
                (*x).clone()
            }
    }).collect()
}

fn calculate_value<C>(weapon : &Weapon, action : &C) -> f64
where C: Fn(&Weapon) -> f64 {
    weapon.quantity *
        (action(weapon) + 
         weapon.extra.iter().fold(0.0, |acc, x| acc + action(x)))
}


fn results_from_weapons<C>(init : Vec<AttackResult>, weapons : &Vec<Weapon>, action : &C, size : i32) 
    -> Vec<AttackResult>
    where C: Fn(&Weapon) -> f64 {
        weapons.iter().fold(init, |mut result_list, weapon| {
            if !result_list.iter().any(|x| x.range == weapon.reach) {
                let val = value_for_lowest_range(&result_list, weapon.reach);
                result_list.push(AttackResult{range: weapon.reach,value: val})
            }

            let weapon_value = size as f64 * calculate_value(weapon, action);
            add_result_at_range(&result_list, weapon.reach, weapon_value)
        })
    }

impl super::Unit {

    fn get_size(&self) -> i32 {
        match self.leader {
            None => self.size,
            Some(_) => self.size - 1,
        }
    }

    fn leader_weapon<C>(&self, action : &C) -> Vec<AttackResult>
        where C: Fn(&Weapon) -> f64 {
            match &self.leader {
                None => Vec::new(),
                Some(x) => results_from_weapons(Vec::new(), &x.weapons, action, 1),
            }
        }

    fn each_weapon<C>(&self, action : C) -> Vec<AttackResult>
        where C: Fn(&Weapon) -> f64 {
            results_from_weapons(self.leader_weapon(&action), &self.weapons, &action, self.get_size())
        }

    pub fn precision(&self) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound)
        })
    }

    pub fn threat(&self) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) * x.damage
        })
    }

    pub fn unsaved(&self, opponent : &super::Unit) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend))
        })
    }

    pub fn expected_damage(&self, opponent : &super::Unit) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend)) * x.damage
        })
    }
}
