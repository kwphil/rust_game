use bevy::prelude::*; 

mod weapon;

pub struct Character {
  pub name: String,
  pub health: u32,
  pub weapons: Vec<weapon::Weapon>,
}
