use bevy::prelude::*;

#[derive(Component)]
pub struct Card {
    image: Handle<Image>,
    size: Vec2,
}

pub const CARD_SIZE_SMALL: Vec2 = Vec2::new(64., 89.);
pub const CARD_SIZE_LARGE: Vec2 = Vec2::new(362., 503.);

impl Card {
    pub fn small(image: Handle<Image>) -> Self {
        Self {
            image,
            size: CARD_SIZE_SMALL,
        }
    }

    pub fn large(image: Handle<Image>) -> Self {
        Self {
            image,
            size: CARD_SIZE_LARGE,
        }
    }
}
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_card_added);
    }
}

fn on_card_added(trigger: Trigger<OnAdd, Card>, mut commands: Commands, card_q: Query<&Card>) {
    let card = card_q.get(trigger.entity()).unwrap();
    commands.entity(trigger.entity()).insert((
        ImageNode::new(card.image.clone()),
        Node {
            width: Val::Px(card.size.x),
            height: Val::Px(card.size.y),
            ..default()
        },
    ));
}
