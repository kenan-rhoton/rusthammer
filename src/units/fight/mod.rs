use super::Unit;
use ::units::weapons::AttackResult;
#[cfg(test)]
mod tests;

extern crate serde_json;

#[derive(Debug,PartialEq)]
pub struct Fight {
    winner: String,
    rounds: i32,
    wound_ratio: f64,
    round_efficiency: f64,
    wound_efficiency: f64,
    kill_damage: f64
}

struct FightStatus {
    wounds: f64,
    original_size: i32,
    condition: Unit,
    buffer: f64
}

impl FightStatus {

    fn from_unit(source : &Unit) -> FightStatus {
        FightStatus {
            wounds: 0.0,
            original_size: source.size,
            condition: source.clone(),
            buffer: (source.wounds * source.size) as f64
        }
    }

    fn wound_ratio(&self) -> f64 {
        (self.buffer - self.wounds) / self.buffer
    }

    fn update_condition(&mut self) {
        self.condition = Unit {
            size: self.original_size - (self.wounds / self.condition.wounds as f64).floor() as i32,
            ..self.condition.clone()
        };
    }
}

fn liberators() -> Unit {
    serde_json::from_str(include_str!("liberators.json")).unwrap()
}

fn fight_result(round : i32, status : FightStatus, enemy_status : FightStatus) -> Fight {
    return Fight {
        winner: status.condition.name.clone(),
        rounds: round,
        wound_ratio: status.wound_ratio(),
        round_efficiency: 100.0 / (round as f64 * status.condition.points as f64),
        wound_efficiency: 100.0 * (status.wound_ratio() / status.condition.points as f64),
        kill_damage: enemy_status.wounds
    };
}

impl Unit {

    pub fn fight(&self, enemy : &Unit) -> Fight {
        let mut round = 1;
        let mut status = FightStatus::from_unit(self);
        let mut enemy_status = FightStatus::from_unit(enemy);

        loop {
            enemy_status.wounds += AttackResult::total(
                status.condition.expected_damage(&enemy_status.condition));

            if enemy_status.wounds >= enemy_status.buffer {
                return fight_result(round, status, enemy_status)
            }

            enemy_status.update_condition();

            status.wounds += AttackResult::total(
                enemy_status.condition.expected_damage(&status.condition));
            if status.wounds >= status.buffer {
                return fight_result(round, enemy_status, status)
            }
            status.update_condition();
            round = round + 1;
        }
    }

    pub fn ekl(&self) -> Fight {
        let libs = liberators();
        self.fight(&libs)
    }

}
