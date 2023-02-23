use std::collections::HashMap;

pub struct Deck(HashMap<CardKind, u8>);

/// Represents a single kind of card (i.e. Flamethrower)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl Deck {
    fn from_card_counts(card_counts: HashMap<CardKind, u8>) -> Self {
        Self(card_counts)
    }

    pub fn for_player_count(player_count: u8) -> Self {
        // todo!("Properly handle player count bounds");
        Self::from_card_counts(Self::player_count_to_card_counts(player_count).unwrap())
    }
}

impl Deck {
    fn player_count_to_card_counts(player_count: u8) -> Option<HashMap<CardKind, u8>> {
        use CardKind::*;

        match player_count {
            4 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 8),
                (Flamethrower, 2),
                (Analysis, 0),
                (Axe, 1),
                (Suspicious, 4),
                (Whiskey, 1),
                (Resolute, 2),
                (WatchYourBack, 1),
                (OutOfHere, 2),
                (YoudBetterRun, 2),
                (Seduction, 2),
                (Scary, 0),
                (ImComfortable, 1),
                (NoThanks, 1),
                (Missed, 1),
                (NoBBQ, 1),
                (Quarantine, 0),
                (BarredDoor, 1),
                (RottenRopes, 0),
                (OneTwo, 0),
                (ThreeFour, 1),
                (SoThisIsTheParty, 0),
                (OutOfHere, 0),
                (Forgetful, 1),
                (RoundAndRound, 1),
                (CantWeBeFriends, 0),
                (BlindDate, 4),
                (Ooops, 0),
                (BetweenUs, 0),
                (Revelations, 0),
            ])),
            _ => None,
        }
    }
}
