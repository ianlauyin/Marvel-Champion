use bevy::prelude::*;

use crate::{
    constant::{CARD_SIZE_LARGE, CARD_SIZE_MEDIUM, CARD_SIZE_SMALL},
    util::UiUtils,
};

#[derive(Component)]
pub struct CardNode {
    image: Handle<Image>,
    size: Vec2,
    is_vertical: bool,
}

impl CardNode {
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

    fn bundle(&self, original_node_op: Option<&Node>) -> impl Bundle {
        (
            ImageNode::new(self.image.clone()),
            self.node(original_node_op),
            Transform::from_rotation(Quat::from_axis_angle(
                Vec3::Z,
                if self.is_vertical {
                    0.
                } else {
                    UiUtils::angle_to_radian(90.)
                },
            )),
            BorderRadius::all(Val::Percent(5.)),
        )
    }

    fn node(&self, original_node_op: Option<&Node>) -> Node {
        let mut node = original_node_op
            .unwrap_or(&Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            })
            .clone();
        node.width = Val::Px(self.size.x);
        node.height = Val::Px(self.size.y);
        node
    }
}
pub struct CardNodePlugin;

impl Plugin for CardNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_added);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, CardNode>,
    mut commands: Commands,
    card_q: Query<(&CardNode, Option<&Node>)>,
) -> Result<(), BevyError> {
    let (card, node_op) = card_q.get(trigger.target())?;
    commands
        .entity(trigger.target())
        .insert(card.bundle(node_op));
    Ok(())
}
