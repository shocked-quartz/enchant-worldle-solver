use chrono::NaiveDate;

enum Color {
    White,
    Blue,
    Black,
    Red,
    Green
}

enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic
}

struct Card {
    name: String,
    cmc: u32,
    colors: Vec<scryfall::card::Color>,
// change it back later
//    colors: Option<Vec<String>>,
    rarity: Rarity,
    types: Vec<String>,
    subtypes: Vec<String>,
    set: String,
    date: NaiveDate
}

impl From<scryfall::card::Card> for Card {
    fn from(value: scryfall::card::Card) -> Self {
        let type_line = value.type_line.unwrap_or_else(||panic!("Search Error: Missing Type"));
        let l: Vec<String> = type_line.split(" — ").map(|s| s.to_string()).collect();
        let types: Vec<String> = l[0].split(" ").map(|s| s.to_string()).collect();
        let subtypes: Vec<String> = match l.get(1) {
            Some(line) => line.split(" ").map(|s| s.to_string()).collect(),
            None => vec![],
        };
        Card {
            name: value.name,
            cmc: value.cmc.unwrap_or_default() as u32,
            colors: value.color_identity,
            rarity: match value.rarity {
                scryfall::card::Rarity::Common => Rarity::Common,
                scryfall::card::Rarity::Uncommon => Rarity::Uncommon,
                scryfall::card::Rarity::Rare => Rarity::Rare,
                scryfall::card::Rarity::Mythic => Rarity::Mythic,
                _ => panic!("Search Error: Unsupported Rarity"),
            },
            types,
            subtypes,
            set: value.set.to_string(),
            date: value.released_at
        }
    }
}

fn main() {

}