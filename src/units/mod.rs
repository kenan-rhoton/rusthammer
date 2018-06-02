mod weapons;
mod probabilities;
mod change;
extern crate serde_json;
use std;

#[derive(Debug,PartialEq,Deserialize,Default,Clone)]
pub struct Unit {
    pub name: String,
    pub bravery: i32,
    pub movement: i32,
    pub save: i32,
    pub size: i32,
    pub weapons: Vec<weapons::Weapon>,
    pub wounds: i32,
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

impl Unit {

    pub fn merge(&self, opt : &UnitOption) -> Unit {
        opt.changes.iter().fold(self.clone(),
            |current, change_data| current.apply_change(change_data))
    }

    pub fn from_file(filename : String) -> Result<Unit, Box<std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let u = serde_json::from_reader(file)?;
        Ok(u)
    }

}

#[cfg(test)]
mod tests;
