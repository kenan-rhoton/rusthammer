use super::Unit;
use super::weapons::Weapon;
use super::leader::Leader;

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
        super::Unit {
            weapons: clone.weapons.iter()
                .map(|w| {
                    if w.name == target.name {
                        w.merge(target)
                    } else {
                        w.clone()
                    }
                })
                .collect(),
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

fn test_unit_two_weapons() -> Unit {
        Unit{
            weapons: vec![
                Weapon {
                    name: String::from("Potato"),
                    ..Default::default()
                },
                Weapon {
                    name: String::from("Tomatoes"),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
}

fn test_unit_one_weapon() -> Unit {
        Unit{
            weapons: vec![
                Weapon {
                    name: String::from("Potato"),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
}

#[test]
fn test_remove_weapon(){
    assert_eq!(
        test_unit_two_weapons().apply_change(
            &Change::RemoveWeapon(String::from("Tomatoes"))
        ),
        test_unit_one_weapon());
}

#[test]
fn test_add_weapon(){
    assert_eq!(
        test_unit_one_weapon().apply_change(
            &Change::AddWeapon(
                Weapon {
                    name: String::from("Tomatoes"),
                    ..Default::default()
                })
        ),
        test_unit_two_weapons());
}
