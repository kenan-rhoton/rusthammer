fn exec_one(command : &String, source : &super::units::Unit) {
    match command.as_ref() {
        "precision" => println!("Precision: {}", source.precision()),
        "threat" => println!("Threat: {}", source.threat()),
        a => eprintln!("Unrecognized command: {}", a),
    }
}

pub fn single_unit(command : &String, source : &super::units::Unit) {
    println!("{}", source.name);
    exec_one(command, source);
    source.retry.iter().for_each(|r| {
        print!("{} - ", r.name);
        exec_one(command, &source.merge(r));
    });
}

fn exec_two(command : &String, source : &super::units::Unit, target : &super::units::Unit ) {
    match command.as_ref() {
        "unsaved" => println!("Unsaved wounds: {}", source.unsaved(target)),
        "damage" => println!("Expected Damage: {}", source.expected_damage(target)),
        a => eprintln!("Unrecognized command: {}", a),
    }
}

pub fn two_units(command : &String, source : &super::units::Unit, target : &super::units::Unit ) {
    println!(
        "{} vs {} (Wounds: {} Size: {} Save: {})",
        source.name, target.name, target.wounds, target.size, target.save
        );

    exec_two(command, source, target);
    source.retry.iter().for_each(|r| {
        print!("{} - ", r.name);
        exec_two(command, &source.merge(r), target)
    });
}
