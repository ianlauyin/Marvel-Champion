use bevy::prelude::Component;

#[derive(Component)]
pub struct IdentityStats {
    hit_points: u8,
    hand_size: u8,
    atk: Option<u8>,
    thw: Option<u8>,
    def: Option<u8>,
    rec: Option<u8>,
}
