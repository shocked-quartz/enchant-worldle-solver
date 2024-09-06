use chrono::NaiveDate;

enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}

struct Card {
    name: String,
    cmc: u32,
    colors: Vec<scryfall::card::Color>,
    // TODO: change it back later
    // colors: Option<Vec<String>>,
    rarity: Rarity,
    types: Vec<String>,
    subtypes: Vec<String>,
    set: String,
    date: NaiveDate,
}

impl From<scryfall::card::Card> for Card {
    fn from(value: scryfall::card::Card) -> Self {
        let type_line = value
            .type_line
            .unwrap_or_else(|| panic!("Search Error: Missing Type"));
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
            date: value.released_at,
        }
    }
}

#[derive(Debug)]
enum Correctness {
    Incorrect,
    PartlyCorrect,
    AlmostCorrect,
}

#[derive(Debug)]
enum Direction {
    Higher(Correctness),
    Lower(Correctness),
}

#[derive(Debug)]
struct Quantity {
    correctness: Correctness,
    value: u32,
}

#[derive(Debug, Default)]
struct Feedback {
    // TODO: Names
    cmc: Option<Direction>,
    colours: Option<Quantity>,
    rarity: Option<Direction>,
    types: Option<Quantity>,
    subtypes: Option<Quantity>,
    set: Option<Direction>,
}

trait Guesser {
    fn next_guess(&self) -> String;
    fn feedback(&self, feedback: Feedback);
}

struct CliGuesser {
    stdin: std::io::Stdin,
}

impl Default for CliGuesser {
    fn default() -> Self {
        Self {
            stdin: std::io::stdin(),
        }
    }
}

impl Guesser for CliGuesser {
    fn next_guess(&self) -> String {
        let mut line = String::new();
        match self.stdin.read_line(&mut line) {
            Ok(s) => s,
            Err(e) => panic!("Error getting line from user: {e:?}"),
        };
        line
    }

    fn feedback(&self, feedback: Feedback) {
        println!("{feedback:?}");
    }
}

fn main() {
    let guesser = CliGuesser::default();

    let max_guesses = 20;
    for guess_index in max_guesses..1 {
        let guess = guesser.next_guess();
        let feedback = Feedback::default();
        // TODO: Calc diff
        guesser.feedback(feedback);
    }
}

// NOTE FOR MULTI FACE CARDS
// MELDS and FLIPS only have characteristics of the front face.
// SPLIT cards have all colors and types of both cards, cmcs added together, and have names written as X // Y.
// TRANSFORMs and MDFCs have the colors of both faces, but only the types of the front. Name is written as X // Y.
