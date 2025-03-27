use bevy::prelude::*;

use crate::util::UiUtils;

#[derive(Component)]
pub struct Card {
    image: Handle<Image>,
    size: Vec2,
    is_vertical: bool,
}

const CARD_SIZE_SMALL: Vec2 = Vec2::new(64., 89.);
const CARD_SIZE_MEDIUM: Vec2 = Vec2::new(128., 178.);
const CARD_SIZE_LARGE: Vec2 = Vec2::new(362., 503.);

impl Card {
    pub fn small(image: Handle<Image>) -> Self {
        Self {
            image,
            size: CARD_SIZE_SMALL,
            is_vertical: true,
        }
    }

    pub fn medium(image: Handle<Image>) -> Self {
        Self {
            image,
            size: CARD_SIZE_MEDIUM,
            is_vertical: true,
        }
    }

    pub fn large(image: Handle<Image>, is_vertical: bool) -> Self {
        Self {
            image,
            size: CARD_SIZE_LARGE,
            is_vertical,
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
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            width: Val::Px(card.size.x),
            height: Val::Px(card.size.y),
            ..default()
        },
        Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Z,
            if card.is_vertical {
                0.
            } else {
                UiUtils::angle_to_radian(90.)
            },
        )),
        BorderRadius::all(Val::Percent(5.)),
    ));
}
