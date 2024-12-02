use crate::features::cards::{Card, TreacheryCard};

pub fn get_sound_manipulation() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_124",
        name: "Sound Manipulation",
        description: "When Revealed (Alter-Ego): Klaw heals 4 damage. If no damage was healed this way, this card gains surge. When Revealed (Hero): Take 2 damage. Klaw heals 2 damage.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_124.png",
        boost: 2,
        traits: vec![],
        keywords: vec![],
    })
}
