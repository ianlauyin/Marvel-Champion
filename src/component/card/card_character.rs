use bevy::prelude::Component;

use super::Count;

#[derive(Component)]
pub struct CardCharacter {
    max_hit_points: Count,
    hit_points: Count,
    atk: u8,
    atk_con_dmg: u8,
    thw: u8,
    thw_con_dmg: u8,
    sch: u8,
    def: u8,
    rec: u8,
}

impl CardCharacter {
    pub fn villain(hit_points: Count, sch: u8, atk: u8) -> Self {
        Self {
            max_hit_points: hit_points.clone(),
            hit_points,
            atk,
            atk_con_dmg: 0,
            thw: 0,
            thw_con_dmg: 0,
            sch,
            def: 0,
            rec: 0,
        }
    }

    pub fn minion(hit_points: Count, sch: u8, atk: u8) -> Self {
        Self {
            max_hit_points: hit_points.clone(),
            hit_points,
            atk,
            sch,
            atk_con_dmg: 0,
            thw: 0,
            thw_con_dmg: 0,
            def: 0,
            rec: 0,
        }
    }

    pub fn ally(hit_points: u8, thw: u8, thw_con_dmg: u8, atk: u8, atk_con_dmg: u8) -> Self {
        Self {
            max_hit_points: Count::Constant(hit_points),
            hit_points: Count::Constant(hit_points),
            atk,
            atk_con_dmg,
            thw,
            thw_con_dmg,
            sch: 0,
            def: 0,
            rec: 0,
        }
    }

    pub fn hero(hit_points: u8, thw: u8, atk: u8, def: u8) -> Self {
        Self {
            max_hit_points: Count::Constant(hit_points),
            hit_points: Count::Constant(hit_points),
            thw,
            thw_con_dmg: 0,
            atk,
            atk_con_dmg: 0,
            sch: 0,
            def,
            rec: 0,
        }
    }

    pub fn alter_ego(hit_points: u8, rec: u8) -> Self {
        Self {
            max_hit_points: Count::Constant(hit_points),
            hit_points: Count::Constant(hit_points),
            atk: 0,
            atk_con_dmg: 0,
            thw: 0,
            thw_con_dmg: 0,
            sch: 0,
            def: 0,
            rec,
        }
    }
}
