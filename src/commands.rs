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
        "high-save" => Unit::high_save(&fetch_unit(&args[2])).json(),
        "ekl" => Unit::ekl(&fetch_unit(&args[2])).json(),
        "unsaved" => Unit::unsaved(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "damage" => Unit::damage(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "fight" => Unit::fight(&fetch_unit(&args[2]), &fetch_unit(&args[3])).json(),
        "top-threat-efficiency" => Unit::top_threat(args).json(),
        "top-high-save-efficiency" => Unit::top_high_save(args).json(),
        _ => format!("Unknown command: {}\n", args[1]),
    }
}
