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
  let s: Value = serde_json::from_str(fs::read_to_string("data/weapon.json"));
  let v: Vec<WeaponInfo> = Vec::new(); 

  for i in s["a"] {
    v.push(WeaponInfo {
      name: i["name"].clone(),
      range: i["range"] as f32,
      damage: i["damage"] as u8,
      reload: i["reload"] as f32,
    }
  }

  v
}

pub enum WeaponType {
  melee(WeaponInfo),
  range(WeaponInfo),
}

pub struct WeaponInfo {
  name: String,
  range: f32,
  damage: u8,
  reload: f32,
}
