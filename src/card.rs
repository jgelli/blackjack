use std::fmt;

pub const NAIPES: [Naipe; 4] = [Naipe::Heart, Naipe::Diamond, Naipe::Club, Naipe::Spade];
pub const FACE_CARDS: [FaceCard; 3] = [FaceCard::King, FaceCard::Queen, FaceCard::Jack];


#[derive(Debug, Copy, Clone)]
pub enum Naipe {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Naipe {
    pub fn symbol(&self) -> char {
        match self {
            Naipe::Heart => '♥',
            Naipe::Diamond => '♦',
            Naipe::Club => '♣',
            Naipe::Spade => '♠',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FaceCard {
    King,
    Queen,
    Jack
}

impl FaceCard {
    pub fn letter(&self) -> char {
        match self {
            FaceCard::King => 'K',
            FaceCard::Queen => 'Q',
            FaceCard::Jack => 'J',
        }
    }
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
    pub fn display(&self) -> String {
        format!("{} {}", self.naipe.symbol(), match &self.value {
            CardValue::Number(n) => n.to_string(),
            CardValue::Face(face) => face.letter().to_string(), 
        })
    }

    pub fn numeric_value(&self) -> u8 {
        match &self.value {
            CardValue::Number(n) => *n,
            CardValue::Face(FaceCard::King | FaceCard::Queen | FaceCard::Jack) => 10,
        }
    }
}