use bevy::prelude::*;
use serde_json::{Result, Value};

mod ammo;
mod character;

#[derive(Component)]
pub struct Weapon {
  weapon_info: WeaponInfo,
  ammo: ammo::Ammo,
}

impl Weapon {
  pub fn player_fire(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Character>,
  ){
    if buttons.just_pressed(MouseButton::Left) {
      Self::fire(&mut query.single_mut().weapons, &mut commands);
    }
  }

  pub fn fire(
    weapon: &mut Vec<Self>,
    commands: &mut Commands,
  ) {
    
  }
}

fn weapon_data() -> Vec<WeaponInfo> {
  let v: Value = serde_json::from_str(fs::read_to_string("data/weapon.json"));

  for i in v {
    
  }
}

pub enum WeaponType {
  melee(WeaponInfo),
  range(WeaponInfo),
}

pub struct WeaponInfo {
  weapon_id: u8,
  name: String,
  range: f32,
  damage: u8,
  reload: f32,
}
