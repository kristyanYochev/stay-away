use std::collections::HashMap;

pub struct Deck(HashMap<CardKind, u8>);

/// Represents a single kind of card (i.e. Flamethrower)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardKind {
    // Role cards
    TheThing,
    Infected,
    // Action cards
    Flamethrower,
    Analysis,
    Axe,
    Suspicious,
    Whiskey,
    Resolute,
    WatchYourBack,
    ChangePlaces,
    YoudBetterRun,
    Seduction,
    // Defence cards
    Scary,
    ImComfortable,
    NoThanks,
    Missed,
    NoBBQ,
    // Obstacle Cards
    Quarantine,
    BarredDoor,
    // Panic Cards,
    RottenRopes,
    OneTwo,
    ThreeFour,
    SoThisIsTheParty,
    OutOfHere,
    Forgetful,
    RoundAndRound,
    CantWeBeFriends,
    BlindDate,
    Ooops,
    BetweenUs,
    Revelations,
}

/// Represents a sort of cards with similar behavior (e.g. Defence cards)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardSort {
    Role,
    Action,
    Defence,
    Obstacle,
    Panic,
}

impl CardKind {
    pub fn sort(&self) -> CardSort {
        use CardKind::*;
        match *self {
            TheThing | Infected => CardSort::Role,

            Flamethrower | Analysis | Axe | Suspicious | Whiskey | Resolute | WatchYourBack
            | ChangePlaces | YoudBetterRun | Seduction => CardSort::Action,

            Scary | ImComfortable | NoThanks | Missed | NoBBQ => CardSort::Defence,

            Quarantine | BarredDoor => CardSort::Obstacle,

            RottenRopes | OneTwo | ThreeFour | SoThisIsTheParty | OutOfHere | Forgetful
            | RoundAndRound | CantWeBeFriends | BlindDate | Ooops | BetweenUs | Revelations => {
                CardSort::Panic
            }
        }
    }
}
