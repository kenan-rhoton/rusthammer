use super::Change;
use ::units::Unit;
use ::units::weapons::Weapon;

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

#[test]
fn test_modify_weapon(){
    assert_eq!(test_unit_one_weapon().weapons[0].hit, 0);
    let changed = test_unit_one_weapon().apply_change(
            &Change::ModifyWeapon(
                Weapon {
                    name: String::from("Potato"),
                    hit: 3,
                    ..Default::default()
                })
        );
    assert_eq!(changed.weapons[0].hit, 3);
}

#[test]
fn test_add_special(){
    let base_unit = Unit{..Default::default()};
    assert!(base_unit.special.is_empty());

    let modified = base_unit.apply_change(
        &Change::AddSpecial(String::from("Potato")));
    
    assert!(modified.special.iter()
            .any(|x| x == &String::from("Potato")));
}

#[test]
fn test_set_size(){
    let base_unit = Unit{..Default::default()};
    assert_eq!(base_unit.size, 0);
    
    let modified = base_unit.apply_change(
        &Change::SetSize(50));
    
    assert_eq!(modified.size, 50);
}

#[test]
fn test_set_points(){
    let base_unit = Unit{..Default::default()};
    assert_eq!(base_unit.points, 0);
    
    let modified = base_unit.apply_change(
        &Change::SetPoints(50));
    
    assert_eq!(modified.points, 50);
}
