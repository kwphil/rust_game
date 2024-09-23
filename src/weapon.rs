use bevy::prelude::*;

mod ammo;

#[derive(Component)]
pub struct Weapon {
  weapon_info: WeaponInfo,
  ammo: ammo::Ammo,
}

impl Weapon {
  pub fn weapon_system(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    query: Query<(Entity, &Weapon)>
  ) {

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
