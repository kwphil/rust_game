use bevy::prelude::*;

mod ammo;

#[derive(Component)]
pub struct Weapon {
  weapon_id: WeaponType,
  ammo: ammo::Ammo,
  damage: u8,
  range: f32,
  reload: f32,
}

impl Weapon {
  pub fn melee() {
    
  }
}

pub struct WeaponInfo {
  name: String,
  range: f32,
  damage: u8,
  reload: f32,
}
