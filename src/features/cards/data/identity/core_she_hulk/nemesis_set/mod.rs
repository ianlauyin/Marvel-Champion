use crate::features::cards::Card;

mod genetically_enhanced;
mod personal_challenge;
mod titania;
mod titanias_fury;

pub fn get_nemesis_set() -> Vec<Card> {
    vec![
        personal_challenge::get_personal_challenge(),
        titania::get_titania(),
        genetically_enhanced::get_genetically_enhanced(),
        titanias_fury::get_titanias_fury(),
        titanias_fury::get_titanias_fury(),
    ]
}
