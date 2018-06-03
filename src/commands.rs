pub fn single_unit(command : &String, source : &super::units::Unit) {
    println!("{}", source.name);
    exec_one(command, source);
    source.retry.iter().for_each(|r| {
        print!("{} - ", r.name);
        exec_one(command, &source.merge(r));
    });
}

pub fn two_units(command : &String, source : &super::units::Unit, target : &super::units::Unit ) {
    println!(
        "{} vs {} (Wounds: {} Size: {} Save: {})",
        source.name, target.name, target.wounds, target.size, target.save
        );

    exec_two(command, source, target);
    source.retry.iter().for_each(|r| {
        println!("{}: ", r.name);
        exec_two(command, &source.merge(r), target)
    });
}


fn exec_one(command : &String, source : &super::units::Unit) {
    match command.as_ref() {
        "precision" => print_all("Precision", source.precision()),
        "threat" => print_all("Threat:", source.threat()),
        a => eprintln!("Unrecognized command: {}", a),
    }
}

fn exec_two(command : &String, source : &super::units::Unit, target : &super::units::Unit ) {
    match command.as_ref() {
        "unsaved" => print_all("Unsaved wounds:", source.unsaved(target)),
        "damage" => print_all("Expected Damage:", source.expected_damage(target)),
        a => eprintln!("Unrecognized command: {}", a),
    }
}



fn print_all(title : &str, result_list : Vec<super::units::weapons::AttackResult>) {
    let (ranged, melee) :
        (Vec<super::units::weapons::AttackResult>,
         Vec<super::units::weapons::AttackResult>)
         = result_list.iter().partition(|x| x.range > 3);

    print_kind("Ranged", ranged);
    print_kind("Melee", melee);

    println!("{}: {}", title, result_list.iter().fold(0.0, |a,x| a + x.value));
}

fn print_kind(title : &str, list : Vec<super::units::weapons::AttackResult>) {
    println!("    {}:", title);
    list.iter().for_each(|x| println!("        {} -> {}", x.range, x.value));
    println!("        TOTAL -> {}", list.iter().fold(0.0, |a,x| a + x.value));

}
