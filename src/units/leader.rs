#[derive(Debug,PartialEq,Deserialize,Default,Clone)]
pub struct Leader{
    pub weapons: Vec<super::weapons::Weapon>
}
