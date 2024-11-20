use crate::features::cards::{Card, Identity};

pub mod core_spider_man;

pub fn get_all(player_number: u8) -> Vec<(Identity, Vec<Card>)> {
    vec![(
        Identity::CoreSpiderMan,
        core_spider_man::get_all(player_number),
    )]
}
