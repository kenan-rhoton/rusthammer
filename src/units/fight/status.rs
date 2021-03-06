use super::Fight;
use ::units::Unit;
use std::cmp;

fn fight_result(round : i32, status : &UnitStatus, enemy_status : &UnitStatus) -> Fight {
    return Fight {
        winner: status.unit.name.clone(),
        rounds: round,
        wound_ratio: status.wound_ratio(),
        round_efficiency: 100.0 / (round as f64 * status.unit.points as f64),
        wound_efficiency: 100.0 * (status.wound_ratio() / status.unit.points as f64),
        kill_damage: enemy_status.wounds_suffered
    };
}

struct UnitStatus {
    wounds_suffered: f64,
    original_size: i32,
    unit: Unit,
    wound_buffer: f64,
    round_casualties: i32
}

impl UnitStatus {

    fn new(unit : &Unit) -> UnitStatus {
        UnitStatus{
            wounds_suffered: 0.0,
            original_size: unit.size,
            wound_buffer: (unit.size * unit.wounds) as f64,
            round_casualties: 0,
            unit: unit.clone()
        }
    }

    fn wound_ratio(&self) -> f64 {
        (self.wound_buffer - self.wounds_suffered) / self.wound_buffer
    }


    fn update_unit(&mut self) {
        let new_size = self.original_size - (self.wounds_suffered / self.unit.wounds as f64).floor() as i32;
        self.unit = Unit {
            size: new_size,
            ..self.unit.clone()
        };
        self.round_casualties = self.unit.size - new_size;
    }

    fn battleshock(&mut self) {
        let result = self.round_casualties + 3;
        let shock = cmp::min(result - self.unit.bravery, self.unit.size);
        if shock > 0 {
            self.unit = Unit {
                size: self.unit.size - shock,
                ..self.unit.clone()
            };
            self.wounds_suffered = ((self.original_size - self.unit.size)
                                    * self.unit.wounds) as f64;
        };
    }

    fn fight(&mut self, target : &mut UnitStatus) -> bool {
        target.wounds_suffered += self.unit.weapons_damage(&target.unit);

        target.update_unit();
        self.wounds_suffered += target.unit.deathrattle() * target.round_casualties as f64;
        self.wounds_suffered += target.unit.thornshields() *
            self.unit.weapons_precision();
        self.update_unit();
        return target.unit.size < 1;
    }
}

pub struct FightStatus {
    round: i32,
    attacker: UnitStatus,
    defender: UnitStatus
}

impl FightStatus {

    pub fn from_units(attacker : &Unit, defender : &Unit) -> FightStatus {
        FightStatus {
            round: 1,
            attacker: UnitStatus::new(attacker),
            defender: UnitStatus::new(defender)
        }
    }

    pub fn update_round(&mut self) {
        self.attacker.battleshock();
        self.defender.battleshock();
        self.round += 1;
    }

    pub fn attack_and_win(&mut self) -> bool {
        return self.attacker.fight(&mut self.defender)
    }

    pub fn defend_and_lose(&mut self) -> bool {
        return self.defender.fight(&mut self.attacker)
    }

    pub fn win(&self) -> Fight {
        fight_result(self.round, &self.attacker, &self.defender)
    }

    pub fn lose(&self) -> Fight {
        fight_result(self.round, &self.defender, &self.attacker)
    }

}
