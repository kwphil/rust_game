use bevy::prelude::*;

mod ammo;

#[derive(Component)]
pub struct Weapon {
  weapon_id: u8,
  ammo: ammo::Ammo,
}

impl Weapon {
  pub fn new() {
    
  }
}
