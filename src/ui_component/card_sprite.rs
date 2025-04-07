use bevy::prelude::*;

use crate::util::UiUtils;

#[derive(Component)]
pub struct CardSprite {
    image: Handle<Image>,
    size: Vec2,
    is_vertical: bool,
}

const CARD_SPRITE_SIZE: Vec2 = Vec2::new(64., 89.);

impl CardSprite {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            image,
            size: CARD_SPRITE_SIZE,
            is_vertical: true,
        }
    }
}
pub struct CardSpritePlugin;

impl Plugin for CardSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_card_sprite_added);
    }
}

fn on_card_sprite_added(
    trigger: Trigger<OnAdd, CardSprite>,
    mut commands: Commands,
    card_sprite_q: Query<&CardSprite>,
) {
    let card_sprite = card_sprite_q.get(trigger.entity()).unwrap();
    commands.entity(trigger.entity()).insert(Sprite {
        image: card_sprite.image.clone(),
        custom_size: Some(card_sprite.size),
        ..default()
    });
}
