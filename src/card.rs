

pub const NAIPES: [Naipe; 4] = [Naipe::Heart, Naipe::Diamond, Naipe::Club, Naipe::Spade];
pub const FACE_CARDS: [FaceCard; 3] = [FaceCard::King, FaceCard::Queen, FaceCard::Jack];


#[derive(Debug, Copy, Clone)]
pub enum Naipe {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Clone, Copy)]
pub enum FaceCard {
    King,
    Queen,
    Jack
}

#[derive(Debug)]
pub enum CardValue {
    Number(u8),
    Face(FaceCard),
}

#[derive(Debug)]
pub struct Card {
    pub naipe: Naipe,
    pub value: CardValue,
}

impl Card {
    pub fn numeric_value(&self) -> u8 {
        match &self.value {
            CardValue::Number(n) => *n,
            CardValue::Face(FaceCard::King | FaceCard::Queen | FaceCard::Jack) => 10,
        }
    }
}