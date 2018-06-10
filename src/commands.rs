use ::units::Unit;

pub fn single_unit(command : &String, source : &mut Unit) {
    source.init();
    source.retry.iter().for_each(|r| {
        print!("{} - ", r.name);
        exec_one(command, &source.merge(r));
    });
}

pub fn two_units(command : &String, source : &mut Unit, target : &mut super::units::Unit ) {
    println!(
        "{} vs {} (Wounds: {} Size: {} Save: {})",
        source.name, target.name, target.wounds, target.size, target.save
        );

    source.init();
    target.init();
    source.retry.iter().for_each(|r| {
        target.retry.iter().for_each(|l| {
            println!("{} vs {}: ", r.name, l.name);
            exec_two(command, &source.merge(r), &target.merge(l));
        });
    });
}


fn exec_one(command : &String, source : &Unit) {
    match command.as_ref() {
        "precision" => print_all("Precision", source.precision(), source.points),
        "threat" => print_all("Threat:", source.threat(), source.points),
        "ekl" => println!("EKL: {:?}", source.ekl()),
        a => eprintln!("Unrecognized command: {}", a),
    }
}

fn exec_two(command : &String, source : &Unit, target : &Unit ) {
    match command.as_ref() {
        "unsaved" => print_all("Unsaved wounds:", source.unsaved(target), source.points),
        "damage" => print_all("Expected Damage:", source.expected_damage(target), source.points),
        "fight" => println!("Fight Result: {:?}", source.fight(target)),
        a => eprintln!("Unrecognized command: {}", a),
    }
}



fn print_all(title : &str, result_list : Vec<super::units::weapons::AttackResult>, points : i32) {
    let (ranged, melee) :
        (Vec<super::units::weapons::AttackResult>,
         Vec<super::units::weapons::AttackResult>)
         = result_list.iter().partition(|x| x.range > 3);

    print_kind("Ranged", &ranged);
    print_kind("Melee", &melee);

    let res = ranged.iter().min_by_key(|x| x.range).unwrap_or(&super::units::weapons::AttackResult{range: 0, value:0.0}).value +
        melee.iter().min_by_key(|x| x.range).unwrap_or(&super::units::weapons::AttackResult{range: 0, value:0.0}).value;
    println!("{}: {} --- EFFICIENCY: {}", title, res, 100.0 * res / points as f64);
}

fn print_kind(title : &str, list : &Vec<super::units::weapons::AttackResult>) {
    println!("    {}:", title);
    list.iter().for_each(|x| println!("        {} -> {}", x.range, x.value));
    let res = list.iter().min_by_key(|x| x.range).unwrap_or(&super::units::weapons::AttackResult{range: 0, value:0.0}).value;
    println!("        TOTAL -> {}", res);

}
