use super::Unit;
use super::UnitOption;
use super::change::Change;

#[test]
fn merge_option_calls_change() {
    let test_unit = Unit{
       points: 50,
       ..Default::default()
    };
    let test_option = UnitOption{
        changes: vec![Change::SetPoints(999)],
       ..Default::default()
    };
    let res = test_unit.merge(&test_option);
    assert_eq!(res.points, 999);
}
