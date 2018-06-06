use super::Weapon;
use super::AttackResult;
use ::units::Unit;

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



impl ::units::Unit {

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

    pub fn unsaved(&self, opponent : &Unit) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend))
        })
    }

    pub fn expected_damage(&self, opponent : &Unit) -> Vec<AttackResult> {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend)) * x.damage
        })
    }
}

