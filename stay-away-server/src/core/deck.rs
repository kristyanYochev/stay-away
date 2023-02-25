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

    pub fn total_card_count(&self) -> u8 {
        self.0.iter().map(|(_, count)| count).sum()
    }

    pub fn card_count(&self, card_kind: CardKind) -> u8 {
        if let Some(count) = self.0.get(&card_kind) {
            *count
        } else {
            0
        }
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
                (ChangePlaces, 2),
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
                (BlindDate, 1),
                (Ooops, 0),
                (BetweenUs, 0),
                (Revelations, 0),
            ])),
            5 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 8),
                (Flamethrower, 2),
                (Analysis, 1),
                (Axe, 1),
                (Suspicious, 4),
                (Whiskey, 1),
                (Resolute, 2),
                (WatchYourBack, 1),
                (ChangePlaces, 2),
                (YoudBetterRun, 2),
                (Seduction, 2),
                (Scary, 1),
                (ImComfortable, 1),
                (NoThanks, 1),
                (Missed, 1),
                (NoBBQ, 1),
                (Quarantine, 1),
                (BarredDoor, 1),
                (RottenRopes, 0),
                (OneTwo, 1),
                (ThreeFour, 1),
                (SoThisIsTheParty, 1),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 1),
                (CantWeBeFriends, 0),
                (BlindDate, 1),
                (Ooops, 0),
                (BetweenUs, 0),
                (Revelations, 0),
            ])),
            6 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 10),
                (Flamethrower, 3),
                (Analysis, 2),
                (Axe, 1),
                (Suspicious, 4),
                (Whiskey, 2),
                (Resolute, 3),
                (WatchYourBack, 1),
                (ChangePlaces, 2),
                (YoudBetterRun, 2),
                (Seduction, 3),
                (Scary, 2),
                (ImComfortable, 2),
                (NoThanks, 2),
                (Missed, 2),
                (NoBBQ, 2),
                (Quarantine, 1),
                (BarredDoor, 1),
                (RottenRopes, 1),
                (OneTwo, 1),
                (ThreeFour, 1),
                (SoThisIsTheParty, 1),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 1),
                (CantWeBeFriends, 0),
                (BlindDate, 1),
                (Ooops, 0),
                (BetweenUs, 0),
                (Revelations, 0),
            ])),
            7 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 12),
                (Flamethrower, 3),
                (Analysis, 2),
                (Axe, 1),
                (Suspicious, 5),
                (Whiskey, 2),
                (Resolute, 3),
                (WatchYourBack, 1),
                (ChangePlaces, 3),
                (YoudBetterRun, 3),
                (Seduction, 4),
                (Scary, 2),
                (ImComfortable, 2),
                (NoThanks, 2),
                (Missed, 2),
                (NoBBQ, 2),
                (Quarantine, 1),
                (BarredDoor, 2),
                (RottenRopes, 1),
                (OneTwo, 1),
                (ThreeFour, 1),
                (SoThisIsTheParty, 1),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 1),
                (CantWeBeFriends, 1),
                (BlindDate, 1),
                (Ooops, 0),
                (BetweenUs, 1),
                (Revelations, 0),
            ])),
            8 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 13),
                (Flamethrower, 3),
                (Analysis, 2),
                (Axe, 1),
                (Suspicious, 6),
                (Whiskey, 2),
                (Resolute, 3),
                (WatchYourBack, 1),
                (ChangePlaces, 3),
                (YoudBetterRun, 3),
                (Seduction, 5),
                (Scary, 3),
                (ImComfortable, 2),
                (NoThanks, 3),
                (Missed, 2),
                (NoBBQ, 2),
                (Quarantine, 1),
                (BarredDoor, 2),
                (RottenRopes, 1),
                (OneTwo, 1),
                (ThreeFour, 1),
                (SoThisIsTheParty, 1),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 1),
                (CantWeBeFriends, 1),
                (BlindDate, 1),
                (Ooops, 0),
                (BetweenUs, 1),
                (Revelations, 1),
            ])),
            9 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 15),
                (Flamethrower, 4),
                (Analysis, 3),
                (Axe, 2),
                (Suspicious, 7),
                (Whiskey, 2),
                (Resolute, 4),
                (WatchYourBack, 2),
                (ChangePlaces, 4),
                (YoudBetterRun, 4),
                (Seduction, 5),
                (Scary, 3),
                (ImComfortable, 2),
                (NoThanks, 3),
                (Missed, 2),
                (NoBBQ, 2),
                (Quarantine, 2),
                (BarredDoor, 2),
                (RottenRopes, 2),
                (OneTwo, 2),
                (ThreeFour, 2),
                (SoThisIsTheParty, 2),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 2),
                (CantWeBeFriends, 2),
                (BlindDate, 2),
                (Ooops, 0),
                (BetweenUs, 2),
                (Revelations, 1),
            ])),
            10 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 17),
                (Flamethrower, 4),
                (Analysis, 3),
                (Axe, 2),
                (Suspicious, 8),
                (Whiskey, 3),
                (Resolute, 5),
                (WatchYourBack, 2),
                (ChangePlaces, 4),
                (YoudBetterRun, 4),
                (Seduction, 6),
                (Scary, 3),
                (ImComfortable, 2),
                (NoThanks, 3),
                (Missed, 2),
                (NoBBQ, 2),
                (Quarantine, 2),
                (BarredDoor, 2),
                (RottenRopes, 2),
                (OneTwo, 2),
                (ThreeFour, 2),
                (SoThisIsTheParty, 2),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 2),
                (CantWeBeFriends, 2),
                (BlindDate, 2),
                (Ooops, 1),
                (BetweenUs, 2),
                (Revelations, 1),
            ])),
            11 | 12 => Some(HashMap::from([
                (TheThing, 1),
                (Infected, 20),
                (Flamethrower, 5),
                (Analysis, 3),
                (Axe, 2),
                (Suspicious, 8),
                (Whiskey, 3),
                (Resolute, 5),
                (WatchYourBack, 2),
                (ChangePlaces, 5),
                (YoudBetterRun, 5),
                (Seduction, 7),
                (Scary, 4),
                (ImComfortable, 3),
                (NoThanks, 4),
                (Missed, 3),
                (NoBBQ, 3),
                (Quarantine, 2),
                (BarredDoor, 3),
                (RottenRopes, 2),
                (OneTwo, 2),
                (ThreeFour, 2),
                (SoThisIsTheParty, 2),
                (OutOfHere, 1),
                (Forgetful, 1),
                (RoundAndRound, 2),
                (CantWeBeFriends, 2),
                (BlindDate, 2),
                (Ooops, 1),
                (BetweenUs, 2),
                (Revelations, 1),
            ])),
            _ => None,
        }
    }
}
