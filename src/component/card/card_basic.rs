use bevy::prelude::Component;

#[derive(Component)]
pub struct CardBasic<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub sub_name: Option<&'a str>,
    pub unique: bool,
    pub card_image_path: &'a str,
    pub card_amount_max: u8,
}
