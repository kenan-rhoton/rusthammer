use ::units::Unit;
use ::units::weapons::Weapon;
use super::EKL;

#[test]
fn test_fail_super_weak() {
    let super_weak = Unit {
        size: 1, wounds: 1, save: 10, ..Default::default()
    };
    assert_eq!(super_weak.ekl(), EKL::Fail(1));
}

#[test]
fn test_fail_sort_of_weak() {
    // Note: Liberators with Shields have a threat of 8.666
    let sort_of_weak = Unit {
        size: 1, wounds: 9, save: 10, ..Default::default()
    };
    assert_eq!(sort_of_weak.ekl(), EKL::Fail(2));
}

#[test]
fn test_fail_weak_with_save() {
    // Note: Liberators with Shields have a threat of 8.666
    let weak_with_save = Unit {
        size: 1, wounds: 7, save: 5, ..Default::default()
    };
    assert_eq!(weak_with_save.ekl(), EKL::Fail(2));
}

#[test]
fn test_fail_eventually() {
    // Note: Liberators with Shields have a threat of 8.666
    //       remember the Grandhammers have Rend -1, though
    let meatbag = Unit {
        size: 1, wounds: 100, save: 2, ..Default::default()
    };
    assert_eq!(meatbag.ekl(), EKL::Fail(51));
}

#[test]
fn test_wipe_instantly() {
    let over_powered = Unit {
        size: 1, wounds: 1, save: 7,
        weapons: vec![
            Weapon{
                reach: 1,
                attacks: 1.0,
                hit: -10,
                wound: -10,
                rend: -100,
                damage: 3000000000.0,
                quantity: 1.0,
                ..Default::default()
            }
        ],
        ..Default::default()
    };
    assert_eq!(over_powered.ekl(), EKL::Wipe);
}

#[test]
fn test_simple_win() {
    // Note: Liberators with Shields have a threat of 8.666
    //       a save of 4+, 2 wounds and a size of 10
    let quick_maths = Unit {
        size: 1, wounds: 5, save: 3, points:100,
        weapons: vec![
            Weapon{
                reach: 1,
                attacks: 1.0,
                hit: -10,
                wound: -10,
                rend: -100,
                damage: 10.0,
                quantity: 1.0,
                ..Default::default()
            }
        ],
        ..Default::default()
    };
    assert_eq!(quick_maths.ekl(), EKL::Win{
        rounds: 2,
        wound_ratio: 0.311111148,
        round_efficiency: 0.5,
        wound_efficiency: 0.311111148
    });
}
