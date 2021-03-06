pub mod weapons;
mod probabilities;
mod change;
mod leader;
mod fight;
mod special;
extern crate serde_json;
use std;

#[derive(Debug,PartialEq,Deserialize,Default,Clone)]
pub struct Unit {
    pub name: String,
    pub points: i32,
    pub bravery: i32,
    pub movement: i32,
    pub save: i32,
    pub size: i32,
    pub weapons: Vec<weapons::Weapon>,
    pub wounds: i32,
    #[serde(default)]
    pub leader: Option<leader::Leader>,
    #[serde(default)]
    pub retry: Vec<UnitOption>,
    #[serde(default)]
    pub special: Vec<String>
}

#[derive(Deserialize,Debug,PartialEq,Default,Clone)]
pub struct UnitOption {
    pub name: String,
    pub changes: Vec<change::Change>
}

#[derive(Serialize,Clone)]
struct UnitResult {
    scenario: String,
    result: f64,
    efficiency: f64
}

impl UnitResult {

    pub fn combine(&self, other: &UnitResult) -> UnitResult {
        UnitResult {
            scenario: self.scenario.clone(),
            result: self.result + other.result,
            efficiency: self.efficiency + other.efficiency
        }
    }
}

#[derive(Serialize,Clone)]
pub struct UnitResultList {
    title: String,
    results: Vec<UnitResult>
}

impl UnitResultList {

    pub fn new(title: String) -> UnitResultList {
        UnitResultList {
            title,
            results: Vec::new()
        }
    }

    pub fn add_result(&mut self, scenario : String, result: f64, efficiency: f64) {
        self.results.push(UnitResult{scenario, result, efficiency});
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn insert_sorted_by_efficiency(&mut self, new_result : UnitResult) {
        for i in 0..self.results.len() {
            if self.results[i].efficiency < new_result.efficiency {
                self.results.insert(i,new_result);
                return
            }
        }
        self.results.push(new_result);
    }

    fn max(&mut self, num : usize) {
        self.results.truncate(num);
    }

    pub fn double(&self) -> UnitResultList {
        UnitResultList {
            results: self.results.iter()
                .map(|x| UnitResult {
                    scenario: x.scenario.clone(),
                    result: x.result * 2.0,
                    efficiency: x.efficiency * 2.0
                }).collect(),
            title: self.title.clone()
        }
    }

    fn find_result(&self, s : &String) -> UnitResult {
        self.results.clone().into_iter().find(|x| x.scenario == s.to_string())
            .unwrap_or(UnitResult{scenario: s.to_string(), result: 0.0, efficiency: 0.0})
    }

    pub fn combine(&self, other: &UnitResultList) -> UnitResultList {
        UnitResultList {
            results: self.results.iter()
                .map(|x| {
                    x.combine(&other.find_result(&x.scenario))
                }).collect(),
            title: self.title.clone()
        }
    }

}

impl Unit {

    fn merge(&self, opt : &UnitOption) -> Unit {
        opt.changes.iter().fold(self.clone(),
            |current, change_data| current.apply_change(change_data))
    }

    fn calculate(&self, action: fn(&Unit) -> f64) -> UnitResultList {
        let mut results = UnitResultList::new(self.name.clone());
        self.retry.iter().for_each(|unit| {
            let merged = self.merge(unit);
            let res = action(&merged);
            results.add_result(
                format!("{}", unit.name),
                res,
                res * 100.0 / merged.points as f64);
        });
        results
    }

    pub fn precision(&self) -> UnitResultList {
        self.calculate(Unit::weapons_precision)
    }

    pub fn threat(&self) -> UnitResultList {
        self.calculate(Unit::weapons_threat)
    }

    fn versus(&self, opponent: &Unit, action : fn(&Unit, &Unit) -> f64) -> UnitResultList {
        let mut results = UnitResultList::new(
            format!("{} vs {}", self.name.clone(), opponent.name.clone()));
        self.retry.iter().for_each(|unit| {
            opponent.retry.iter().for_each(|opp| {
                let merged = self.merge(unit);
                let res = action(&merged, &opponent.merge(opp));
                results.add_result(
                    format!("{} vs {}", unit.name, opp.name),
                    res,
                    res * 100.0 / merged.points as f64);
            });
        });
        results
    }

    pub fn unsaved(&self, opponent: &Unit) -> UnitResultList {
        self.versus(&opponent, Unit::weapons_unsaved)
    }

    pub fn damage(&self, opponent: &Unit) -> UnitResultList {
        self.versus(&opponent, Unit::weapons_damage)
    }

    pub fn penetration(&self) -> UnitResultList {
        self.damage(&Unit {
            name: String::from("Save 2+"),
            save: 2,
            retry: vec![UnitOption{
                name: String::from("Save 2+"),
                changes: Vec::new()
            }],
            ..Default::default()
        })
    }

    fn only_ranged(&self) -> Unit {
        Unit {
            weapons: self.weapons.clone().into_iter().filter(|x| x.reach > 3).collect(),
            retry: self.retry.clone().into_iter()
                .map(|x|
                     UnitOption {
                         name: x.name,
                         changes: x.changes.into_iter()
                             .filter(|c| match c {
                                 change::Change::AddWeapon(w) =>
                                     w.reach > 3,
                                 _ => true,
                             }).collect()
                     }).collect(),
            ..self.clone()
        }
    }

    pub fn ranged_threat(&self) -> UnitResultList {
        self.only_ranged().threat()
    }

    pub fn ranged_penetration(&self) -> UnitResultList {
        self.only_ranged().penetration()
    }

    fn only_combat(&self) -> Unit {
        Unit {
            weapons: self.weapons.clone().into_iter().filter(|x| x.reach <= 3).collect(),
            retry: self.retry.clone().into_iter()
                .map(|x|
                     UnitOption {
                         name: x.name,
                         changes: x.changes.into_iter()
                             .filter(|c| match c {
                                 change::Change::AddWeapon(w) =>
                                     w.reach <= 3,
                                 _ => true,
                             }).collect()
                     }).collect(),
            ..self.clone()
        }
    }

    pub fn combat_threat(&self) -> UnitResultList {
        self.only_combat().threat()
    }

    pub fn combat_penetration(&self) -> UnitResultList {
        self.only_combat().penetration()
    }

    pub fn effective_threat(&self) -> UnitResultList {
        self.combat_threat().double().combine(&self.ranged_threat())
    }

    pub fn effective_penetration(&self) -> UnitResultList {
        self.combat_penetration().double().combine(&self.ranged_penetration())
    }

    fn top_efficiency(unit_list : Vec<String>, action : fn(&Unit) -> UnitResultList, name: String) -> UnitResultList {
        let mut results = UnitResultList::new(name);

        unit_list.iter().skip(2).for_each(|u| {
            let unit = Unit::from_file(u.to_string()).unwrap();
            let unit_result = action(&unit);

            unit_result.results.into_iter().for_each(|mut res| {
                res.scenario = format!("{} - {}", unit.name, res.scenario);
                results.insert_sorted_by_efficiency(res);
            });
        });
        results.max(20);
        results
    }

    pub fn top_threat_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::effective_threat,
            String::from("Top Threat Efficiency"))
    }

    pub fn top_penetration_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::effective_penetration,
            String::from("Top Penetration Efficiency"))
    }

    pub fn top_ranged_threat_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::ranged_threat,
            String::from("Top Ranged Threat Efficiency"))
    }

    pub fn top_ranged_penetration_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::ranged_penetration,
            String::from("Top Ranged Penetration Efficiency"))
    }

    pub fn top_combat_threat_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::combat_threat,
            String::from("Top Combat Threat Efficiency"))
    }

    pub fn top_combat_penetration_efficiency(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::combat_penetration,
            String::from("Top Combat Penetration Efficiency"))
    }

    pub fn from_file(filename : String) -> Result<Unit, Box<std::error::Error>> {
        let file = std::fs::File::open(filename)?;

        let mut u : Unit = serde_json::from_reader(file)?;
        
        u.retry.insert(0, ::units::UnitOption{
            name: String::from("Base"),
            changes: Vec::new()
        });
        
        Ok(u)
    }

}

#[cfg(test)]
mod tests;
