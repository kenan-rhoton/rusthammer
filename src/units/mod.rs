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

#[derive(Serialize)]
struct UnitResult {
    scenario: String,
    result: f64,
    efficiency: f64
}

#[derive(Serialize)]
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

    pub fn high_save(&self) -> UnitResultList {
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

    pub fn top_threat(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::threat,
            String::from("Top Threat Efficiency"))
    }

    pub fn top_high_save(unit_list : Vec<String>) -> UnitResultList {
        Unit::top_efficiency(
            unit_list,
            Unit::high_save,
            String::from("Top Penetration Efficiency"))
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
