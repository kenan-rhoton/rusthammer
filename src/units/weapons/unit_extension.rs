use super::Weapon;
use ::units::Unit;

fn calculate_value<C>(weapon : &Weapon, action : &C) -> f64
where C: Fn(&Weapon) -> f64 {
    weapon.quantity *
        (action(weapon) + 
         weapon.extra.iter().fold(0.0, |acc, x| acc + action(x)))
}

impl ::units::Unit {

    fn get_size(&self) -> i32 {
        match self.leader {
            None => self.size,
            Some(_) => self.size - 1,
        }
    }

    fn leader_weapon<C>(&self, action : &C) -> f64
        where C: Fn(&Weapon) -> f64 {
            match &self.leader {
                None => 0.0,
                Some(x) => x.weapons.iter().fold(
                    0.0,
                    |a,weapon| a + calculate_value(weapon, &action)),
            }
        }

    fn each_weapon<C>(&self, action : C) -> f64
        where C: Fn(&Weapon) -> f64 {
            self.weapons.iter().fold(
                0.0,
                |a,weapon| a + calculate_value(weapon, &action))
                * self.get_size() as f64
                + self.leader_weapon(&action)
        }

    pub fn weapons_precision(&self) -> f64 {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound)
        })
    }

    pub fn weapons_threat(&self) -> f64 {
        self.each_weapon(move |x: &Weapon| {
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) * x.damage
        })
    }

    pub fn weapons_unsaved(&self, opponent : &Unit) -> f64 {
        self.each_weapon(move |w: &Weapon| {
            let x = opponent.affect_opponent_weapon(w);
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend))
        })
    }

    pub fn weapons_damage(&self, opponent : &Unit) -> f64 {
        self.each_weapon(move |w: &Weapon| {
            let x = opponent.affect_opponent_weapon(w);
            x.attacks * self.to_hit(x.hit) * self.to_wound(x.wound) *
                (1.0 - opponent.to_save(x.rend)) * x.damage
        })
    }
}
