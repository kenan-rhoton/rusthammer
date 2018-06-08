use super::Unit;
use ::units::weapons::AttackResult;
#[cfg(test)]
mod tests;

extern crate serde_json;

#[derive(Debug,PartialEq)]
pub enum EKL {
    Fail{
        rounds: i32,
        wound_ratio: f64,
        round_efficiency: f64,
        wound_efficiency: f64,
        kill_damage: f64
    },
    Wipe(f64),
    Win{
        rounds: i32,
        wound_ratio: f64,
        round_efficiency: f64,
        wound_efficiency: f64,
        kill_damage: f64
    }
}

fn liberators() -> Unit {
    serde_json::from_str(include_str!("liberators.json")).unwrap()
}

impl Unit {

    pub fn fight(&self, enemy : &Unit) -> EKL {
        let (mut my_wounds, mut enemy_wounds, mut round) = (0., 0., 1);
        let mut my_condition = self.clone();
        let mut enemy_condition = enemy.clone();
        let wound_buffer = (self.wounds * self.size) as f64;
        let enemy_buffer = (enemy.wounds * enemy.size) as f64;

        loop {
            enemy_wounds += AttackResult::total(
                my_condition.expected_damage(&enemy_condition));

            if enemy_wounds >= (enemy.wounds * enemy.size) as f64 {
                if round == 1 {
                    return EKL::Wipe(enemy_wounds)
                }
                return EKL::Win{
                    rounds: round,
                    wound_ratio: (wound_buffer - my_wounds)/wound_buffer,
                    round_efficiency: 100.0 / (round as f64 * self.points as f64),
                    wound_efficiency: 100.0 * (((wound_buffer - my_wounds)/wound_buffer) / self.points as f64),
                    kill_damage: enemy_wounds
                };
            }
            
            enemy_condition = Unit {
                size: enemy.size - (enemy_wounds / enemy.wounds as f64).floor() as i32,
                ..enemy_condition
            };
           
            my_wounds += AttackResult::total(
                enemy_condition.expected_damage(&my_condition));
            if my_wounds >= wound_buffer {
                return EKL::Fail{
                    rounds: round,
                    wound_ratio: (enemy_buffer - enemy_wounds)/enemy_buffer,
                    round_efficiency: 100.0 / (round as f64 * enemy.points as f64),
                    wound_efficiency: 100.0 * (((enemy_buffer - enemy_wounds)/enemy_buffer) / enemy.points as f64),
                    kill_damage: my_wounds
                };
            }
            round = round + 1;

            my_condition = Unit {
                size: self.size - (my_wounds / self.wounds as f64).floor() as i32,
                ..my_condition
            };
        }
    }

    pub fn ekl(&self) -> EKL {
        let libs = liberators();
        self.fight(&libs)
    }

}
