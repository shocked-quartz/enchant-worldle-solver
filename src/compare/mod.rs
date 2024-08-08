use scryfall::Card;

pub enum Feedback<T> {
    Correct,
    Incorrect(T),
}

pub enum Direction {
    Higher,
    Lower,
}

pub enum RangeResponse {
    Close(Direction),
    Far(Direction),
}

pub enum FractionalResponse {
    Some(f32),
    More,
}

struct _Response {
    name: Option<Feedback<Direction>>,
    cmc: Feedback<RangeResponse>,
    colors: Feedback<FractionalResponse>,
    rarity: Feedback<Direction>,
    types: Feedback<FractionalResponse>,
    subtype: Feedback<FractionalResponse>,
    set: Feedback<RangeResponse>,
}

pub fn compare(guess: Card, solution: Card) {}
