use ::units::Unit;

fn fetch_unit(path : &String) -> Unit {
    Unit::from_file(path.to_string()).unwrap()
}

pub fn command(args : Vec<String>) {
    let res = exec(args);
    println!("{}", res);
}

fn exec(args : Vec<String>) -> String {
    match args[1].as_ref() {
        "precision" => Unit::precision(&fetch_unit(&args[2])).json(),
        "threat" => Unit::threat(&fetch_unit(&args[2])).json(),
        "penetration" => Unit::penetration(&fetch_unit(&args[2])).json(),
        "ranged-threat" => Unit::ranged_threat(&fetch_unit(&args[2])).json(),
        "ranged-penetration" => Unit::ranged_penetration(&fetch_unit(&args[2])).json(),
        "combat-threat" => Unit::combat_threat(&fetch_unit(&args[2])).json(),
        "combat-penetration" => Unit::combat_penetration(&fetch_unit(&args[2])).json(),
        "effective-threat" => Unit::effective_threat(&fetch_unit(&args[2])).json(),
        "effective-penetration" => Unit::effective_penetration(&fetch_unit(&args[2])).json(),
        "ekl" => Unit::ekl(&fetch_unit(&args[2])).json(),
        "unsaved" => Unit::unsaved(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "damage" => Unit::damage(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "fight" => Unit::fight(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "top-threat-efficiency" => Unit::top_threat_efficiency(args).json(),
        "top-penetration-efficiency" => Unit::top_penetration_efficiency(args).json(),
        "top-ranged-threat-efficiency" => Unit::top_ranged_threat_efficiency(args).json(),
        "top-ranged-penetration-efficiency" => Unit::top_ranged_penetration_efficiency(args).json(),
        "top-combat-threat-efficiency" => Unit::top_combat_threat_efficiency(args).json(),
        "top-combat-penetration-efficiency" => Unit::top_combat_penetration_efficiency(args).json(),
        _ => format!("Unknown command: {}\n", args[1]),
    }
}
