use super::Unit;
use super::weapons::Weapon;
use super::leader::Leader;

#[cfg(test)]
mod tests;

#[derive(Deserialize,Debug,PartialEq,Clone)]
pub enum Change {
    RemoveWeapon(String),
    AddWeapon(Weapon),
    ModifyWeapon(Weapon),
    AddSpecial(String),
    SetSize(i32),
    SetPoints(i32)
}

impl Unit {

    fn remove_weapon(&self, target : &String) -> Unit {
        let clone = self.clone();
        Unit {
            weapons: clone.weapons.into_iter()
                .filter(|w| w.name != *target)
                .collect(),
            ..clone
        }
    }

    fn add_weapon(&self, target : &Weapon) -> Unit {
        let mut clone = self.clone();
        clone.weapons.push(target.clone());
        Unit {
            ..clone
        }
    }

    fn modify_weapon(&self, target : &Weapon) -> Unit {
        let clone = self.clone();
        Unit {
            leader: match clone.leader {
                None => None,
                Some(lead) => Some(Leader{
                    weapons: Weapon::merge_weapon(&lead.weapons, target),
                    ..lead.clone()
                }),
            },
            weapons: Weapon::merge_weapon(&clone.weapons, target),
            ..clone
        }
    }

    fn add_special(&self, special : &String) -> Unit {
        let mut clone = self.clone();
        clone.special.push(special.clone());
        Unit {
            ..clone
        }
    }

    fn set_size(&self, size : i32) -> Unit {
        Unit {
            size,
            ..self.clone()
        }
    }

    fn set_points(&self, points : i32) -> Unit {
        Unit {
            points,
            ..self.clone()
        }
    }

    pub fn apply_change(&self, change : &Change) -> Unit {
        match change {
            Change::RemoveWeapon(w) => self.remove_weapon(w),
            Change::AddWeapon(w) => self.add_weapon(w),
            Change::ModifyWeapon(w) => self.modify_weapon(w),
            Change::AddSpecial(s) => self.add_special(s),
            Change::SetSize(i) => self.set_size(*i),
            Change::SetPoints(i) => self.set_points(*i),
        }
    }
}
