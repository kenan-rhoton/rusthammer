
#[derive(Deserialize,Debug,PartialEq,Clone)]
pub enum Change {
    RemoveWeapon(String),
    AddWeapon(super::weapons::Weapon),
    ModifyWeapon(super::weapons::Weapon),
    AddSpecial(String)
}

impl super::Unit {

    fn remove_weapon(&self, target : &String) -> super::Unit {
        let clone = self.clone();
        super::Unit {
            weapons: clone.weapons.into_iter()
                .filter(|w| w.name != *target)
                .collect(),
            ..clone
        }
    }

    fn add_weapon(&self, target : &super::weapons::Weapon) -> super::Unit {
        let mut clone = self.clone();
        clone.weapons.push(target.clone());
        super::Unit {
            ..clone
        }
    }

    fn modify_weapon(&self, target : &super::weapons::Weapon) -> super::Unit {
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

    fn add_special(&self, special : &String) -> super::Unit {
        let mut clone = self.clone();
        clone.special.push(special.clone());
        super::Unit {
            ..clone
        }
    }

    pub fn apply_change(&self, change : &Change) -> super::Unit {
        match change {
            Change::RemoveWeapon(w) => self.remove_weapon(w),
            Change::AddWeapon(w) => self.add_weapon(w),
            Change::ModifyWeapon(w) => self.modify_weapon(w),
            Change::AddSpecial(s) => self.add_special(s),
        }
    }
}

fn test_unit_two_weapons() -> super::Unit {
        super::Unit{
            weapons: vec![
                super::weapons::Weapon {
                    name: String::from("Potato"),
                    ..Default::default()
                },
                super::weapons::Weapon {
                    name: String::from("Tomatoes"),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
}

fn test_unit_one_weapon() -> super::Unit {
        super::Unit{
            weapons: vec![
                super::weapons::Weapon {
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
                super::weapons::Weapon {
                    name: String::from("Tomatoes"),
                    ..Default::default()
                })
        ),
        test_unit_two_weapons());
}
