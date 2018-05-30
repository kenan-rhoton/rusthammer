extern crate serde;
extern crate serde_json;


// For better and clearer tests
#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
#[macro_use]
extern crate serde_derive;

mod units;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => {
            let unit1 = units::Unit::from_file(args[2].parse().unwrap());
            match (args[1].as_ref(), unit1) {
                ("threat", Ok(u1)) =>
                    println!("{}\nThreat: {}", u1.name, u1.threat()),
                ("precision", Ok(u1)) =>
                    println!("{}\nPrecision: {}", u1.name, u1.precision()),
                (_, Err(_)) =>
                    eprintln!("Cannot find data file: {}", args[2]),
                (_, Ok(_)) =>
                    eprintln!("Unrecognized command: {}", args[1]),
            }
        },
        4 => {
            let unit1 = units::Unit::from_file(args[2].parse().unwrap());
            let unit2 = units::Unit::from_file(args[3].parse().unwrap());
            match (args[1].as_ref(), unit1, unit2) {
                ("damage", Ok(u1), Ok(u2)) =>
                    println!(
                        "{} vs {} (Wounds: {} Size: {} Save: {}\n\
                        Expected Damage: {}",
                        u1.name, u2.name, u2.wounds, u2.size, u2.save,
                        u1.expected_damage(&u2)),
                ("unsaved", Ok(u1), Ok(u2)) =>
                    println!(
                        "{} vs {} (Wounds: {} Size: {} Save: {}\n\
                        Expected Unsaved Wounds: {}",
                        u1.name, u2.name, u2.wounds, u2.size, u2.save,
                        u1.unsaved(&u2)),
                (_, Err(_), Ok(_)) =>
                    eprintln!("Cannot find data file: {}", args[2]),
                (_, Ok(_), Err(_)) =>
                    eprintln!("Cannot find data file: {}", args[3]),
                (_, Err(_), Err(_)) =>
                    eprintln!("Cannot find data files: {} {}", args[2], args[3]),
                (_, Ok(_), Ok(_)) =>
                    eprintln!("Unrecognized command: {}", args[1]),
            }
        },
        _ => {
            eprintln!("Invalid stuff!")
        }
    }
}

