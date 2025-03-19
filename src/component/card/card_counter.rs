use bevy::ecs::component::Component;

#[derive(Component)]
pub enum CardCounter {
    Attack(u8),
    Web(u8),
    Medical(u8),
    Snoop(u8),
    Arrow(u8),
}
