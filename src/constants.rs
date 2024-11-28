use bevy::prelude::*;

pub const WINDOW_RESOLUTION: Vec2 = Vec2::new(1366., 768.);
pub const CARD_SIZE: Vec3 = Vec3::new(64., 89., 1.);
pub const CARD_DETAIL_SIZE: Vec2 = Vec2::new(362., 503.);

pub struct AssetConst {
    key: &'static str,
    pub path: &'static str,
}

impl AssetConst {
    pub fn into_load_asset(&self, asset_server: Res<AssetServer>) -> (String, Handle<Image>) {
        (self.key.to_string(), asset_server.load(self.path))
    }
}

pub const GAME_MAT_ASSET: AssetConst = AssetConst {
    key: "game_mat",
    path: "embedded://game_mat.png",
};

pub const ENCOUNTER_CARD_BACK_ASSET: AssetConst = AssetConst {
    key: "encounter_card_back",
    path: "embedded://cards/card_backs/encounter_card_back.png",
};

pub const PLAYER_CARD_BACK_ASSET: AssetConst = AssetConst {
    key: "player_card_back",
    path: "embedded://cards/card_backs/player_card_back.png",
};

pub const VILLAIN_CARD_BACK_ASSET: AssetConst = AssetConst {
    key: "villain_card_back",
    path: "embedded://cards/card_backs/villain_card_back.png",
};
