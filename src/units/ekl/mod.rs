use super::Unit;
use ::units::weapons::AttackResult;
#[cfg(test)]
mod tests;

extern crate serde_json;

#[derive(Debug,PartialEq)]
pub enum EKL {
    Fail(i32),
    Wipe,
    Win{
        rounds: i32,
        wound_ratio: f64,
        round_efficiency: f64,
        wound_efficiency: f64
    }
}

fn liberators() -> Unit {
    serde_json::from_str(include_str!("liberators.json")).unwrap()
}

impl Unit {

    fn to_the_death(&self, enemy : &Unit) -> EKL {
        let (mut my_wounds, mut enemy_wounds, mut round) = (0., 0., 1);
        let wound_buffer = (self.wounds * self.size) as f64;

        loop {
            enemy_wounds += AttackResult::total(
                self.expected_damage(enemy));

            if enemy_wounds >= (enemy.wounds * enemy.size) as f64 {
                return EKL::Win{
                    rounds: round,
                    wound_ratio: (wound_buffer - my_wounds)/wound_buffer,
                    round_efficiency: 100.0 * (round as f64 / self.points as f64),
                    wound_efficiency: 100.0 * (((wound_buffer - my_wounds)/wound_buffer) / self.points as f64)
                };
            }
            my_wounds += AttackResult::total(
                enemy.expected_damage(self));
            if my_wounds >= wound_buffer {
                return EKL::Fail(round);
            }
            round = round + 1;
        }
    }

    pub fn ekl(&self) -> EKL {
        let libs = liberators();
        if self.expected_damage(&libs).iter()
            .any(|x| x.value > (libs.wounds * libs.size) as f64) {
                return EKL::Wipe;
            }

        self.to_the_death(&libs)
    }

}
