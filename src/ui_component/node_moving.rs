use bevy::prelude::*;

const DELTA_AMOUNT: u8 = 10;

#[derive(Event)]
pub struct NodeMoveRemoveEvent(pub Entity);

#[derive(Component)]
#[require(Node)]
pub struct NodeMove {
    delta: Vec2,
    current_delta: u8,
}

impl NodeMove {
    pub fn from_delta(delta: Vec2) -> Self {
        Self {
            delta: delta / DELTA_AMOUNT as f32,
            current_delta: DELTA_AMOUNT,
        }
    }
}

pub struct NodeMovingPlugin;

impl Plugin for NodeMovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_movement);
    }
}

fn handle_movement(
    mut commands: Commands,
    mut move_components_q: Query<(Entity, &mut NodeMove, &mut Node)>,
) {
    for (entity, mut move_to, mut node) in move_components_q.iter_mut() {
        let (Val::Px(x), Val::Px(y)) = (node.left, node.top) else {
            warn!("Wrong type of Val for NodeMove");
            continue;
        };
        if move_to.current_delta == 0 {
            commands.entity(entity).remove::<NodeMove>();
            commands.trigger(NodeMoveRemoveEvent(entity));
            continue;
        };
        node.left = Val::Px(x + move_to.delta.x);
        node.top = Val::Px(y + move_to.delta.y);
        move_to.current_delta -= 1;
    }
}
