extern crate serde;
extern crate serde_json;


// For better and clearer tests
#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
#[macro_use]
extern crate serde_derive;

mod units;
mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => {
            let unit1 = units::Unit::from_file(args[2].parse().unwrap());
            match unit1 {
                Ok(u1) =>
                    commands::single_unit(&args[1], &u1),
                Err(_) =>
                    eprintln!("Cannot find data file: {}", args[2]),
            }
        },
        4 => {
            let unit1 = units::Unit::from_file(args[2].parse().unwrap());
            let unit2 = units::Unit::from_file(args[3].parse().unwrap());
            match (unit1, unit2) {
                (Ok(u1), Ok(u2)) =>
                    commands::two_units(&args[1], &u1, &u2),
                (Err(_), Ok(_)) =>
                    eprintln!("Cannot find data file: {}", args[2]),
                (Ok(_), Err(_)) =>
                    eprintln!("Cannot find data file: {}", args[3]),
                (Err(_), Err(_)) =>
                    eprintln!("Cannot find data files: {} {}", args[2], args[3]),
            }
        },
        _ => {
            eprintln!("Invalid stuff!")
        }
    }
}

