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

    if args.len() < 2 {
        eprintln!("Need at least one argument!")
    } else {
        commands::command(args)
    }

}

