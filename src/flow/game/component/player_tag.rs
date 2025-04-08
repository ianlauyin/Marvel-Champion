#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub enum PlayerTag {
    #[default]
    One,
    Two,
    Three,
    Four,
}

impl PlayerTag {
    pub fn to_string(&self) -> &str {
        match self {
            PlayerTag::One => "1",
            PlayerTag::Two => "2",
            PlayerTag::Three => "3",
            PlayerTag::Four => "4",
        }
    }

    pub fn get_all_asc() -> Vec<PlayerTag> {
        vec![
            PlayerTag::One,
            PlayerTag::Two,
            PlayerTag::Three,
            PlayerTag::Four,
        ]
    }
}
