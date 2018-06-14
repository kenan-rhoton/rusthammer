mod status;

use super::Unit;
use self::status::FightStatus;


#[cfg(test)]
mod tests;

extern crate serde_json;

#[derive(Debug,PartialEq,Serialize)]
pub struct Fight {
    winner: String,
    rounds: i32,
    wound_ratio: f64,
    round_efficiency: f64,
    wound_efficiency: f64,
    kill_damage: f64
}

impl Fight {

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

}


fn liberators() -> Unit {
    serde_json::from_str(include_str!("liberators.json")).unwrap()
}

impl Unit {

    pub fn fight(&self, enemy : &Unit) -> Fight {
        let mut status = FightStatus::from_units(self, enemy);

        loop {
            if status.attack_and_win() {
                return status.win()
            }
            if status.defend_and_lose() {
                return status.lose()
            }
            status.update_round()
        }
    }

    pub fn ekl(&self) -> Fight {
        let libs = liberators();
        self.fight(&libs)
    }

}
