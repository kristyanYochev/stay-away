use std::collections::HashMap;

use rand::seq::SliceRandom;

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

    pub fn empty() -> Self {
        Self(HashMap::default())
    }

    pub fn for_player_count(player_count: u8) -> Self {
        Self::from_card_counts(Self::player_count_to_card_counts(player_count).unwrap())
    }

    pub fn total_card_count(&self) -> u8 {
        self.0.iter().map(|(_, count)| count).sum()
    }

    pub fn card_count(&self, card_kind: CardKind) -> u8 {
        *self.0.get(&card_kind).unwrap_or(&0)
    }

    pub fn draw_specific_card(
        &mut self,
        card_kind: CardKind,
    ) -> Result<CardKind, errors::NoSuchCard> {
        let count = self
            .0
            .get_mut(&card_kind)
            .ok_or(errors::NoSuchCard(card_kind))?;

        if *count != 0 {
            *count -= 1;
            Ok(card_kind)
        } else {
            Err(errors::NoSuchCard(card_kind))
        }
    }

    pub fn draw_random_card(&mut self) -> Result<CardKind, errors::EmptyDeck> {
        let picked_card_kind = self.choose_random_card()?;
        let picked_card_count = self.0.get_mut(&picked_card_kind).unwrap();

        *picked_card_count -= 1;

        Ok(picked_card_kind)
    }

    pub fn choose_random_card(&self) -> Result<CardKind, errors::EmptyDeck> {
        if self.total_card_count() == 0 {
            return Err(errors::EmptyDeck);
        }

        let mut rng = rand::thread_rng();
        let possible_card_kinds_vec = self.0.keys().collect::<Vec<_>>();
        let possible_card_kinds = possible_card_kinds_vec.as_slice();

        let picked_card_kind = possible_card_kinds
            .choose_weighted(&mut rng, |card| self.0.get(*card).unwrap_or(&0))
            .unwrap();

        Ok(**picked_card_kind)
    }

    pub fn add_card(&mut self, card_kind: CardKind) {
        if let Some(count) = self.0.get_mut(&card_kind) {
            *count += 1;
        } else {
            self.0.insert(card_kind, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_deck_has_a_card_count_of_0() {
        let deck = Deck::empty();

        assert_eq!(deck.total_card_count(), 0);
    }

    #[test]
    fn cannot_draw_cards_from_an_empty_deck() {
        let mut deck = Deck::empty();

        let result = deck.draw_specific_card(CardKind::Analysis);
        assert!(result.is_err());

        let error = result.expect_err("This was supposed to be a NoSuchCard error");
        assert_eq!(error.0, CardKind::Analysis);
    }

    #[test]
    fn cannot_draw_cards_from_a_deck_not_containing_such_card() {
        let mut deck = Deck::from_card_counts(HashMap::from([(CardKind::Analysis, 3)]));
        assert_eq!(deck.card_count(CardKind::Analysis), 3);

        let result = deck.draw_specific_card(CardKind::BarredDoor);
        assert!(result.is_err());

        let error = result.expect_err("This was supposed to be a NoSuchCard error");
        assert_eq!(error.0, CardKind::BarredDoor);
    }

    #[test]
    fn drawing_a_specific_card_decreases_count_by_1() {
        let mut deck = Deck::from_card_counts(HashMap::from([(CardKind::Analysis, 3)]));
        assert_eq!(deck.card_count(CardKind::Analysis), 3);

        let result = deck.draw_specific_card(CardKind::Analysis);
        assert!(result.is_ok());

        let drawn_card = result.expect("This is supposed to be an Analysis card");
        assert_eq!(drawn_card, CardKind::Analysis);
        assert_eq!(deck.card_count(CardKind::Analysis), 2);
    }

    #[test]
    fn drawing_a_random_card_errors_out_when_deck_is_empty() {
        let mut deck = Deck::empty();

        let result = deck.draw_random_card();
        assert!(result.is_err());
    }

    #[test]
    fn drawing_a_random_card_decreases_total_count_by_1() {
        let mut deck = Deck::from_card_counts(HashMap::from([(CardKind::Analysis, 3)]));
        assert_eq!(deck.total_card_count(), 3);

        let result = deck.draw_random_card();
        assert!(result.is_ok());

        let drawn_card = result.expect("Should be an Analysis card");
        assert_eq!(drawn_card, CardKind::Analysis);
        assert_eq!(deck.total_card_count(), 2);
    }

    #[test]
    fn inserting_a_card_increases_its_count_by_1() {
        let mut deck = Deck::from_card_counts(HashMap::from([(CardKind::Analysis, 3)]));
        assert_eq!(deck.total_card_count(), 3);
        assert_eq!(deck.card_count(CardKind::Analysis), 3);
        assert_eq!(deck.card_count(CardKind::BarredDoor), 0);

        deck.add_card(CardKind::Analysis);
        assert_eq!(deck.total_card_count(), 4);
        assert_eq!(deck.card_count(CardKind::Analysis), 4);
        assert_eq!(deck.card_count(CardKind::BarredDoor), 0);

        deck.add_card(CardKind::BarredDoor);
        assert_eq!(deck.total_card_count(), 5);
        assert_eq!(deck.card_count(CardKind::Analysis), 4);
        assert_eq!(deck.card_count(CardKind::BarredDoor), 1);
    }
}

pub mod errors {
    use super::CardKind;

    #[derive(Debug)]
    pub struct NoSuchCard(pub CardKind);

    #[derive(Debug)]
    pub struct EmptyDeck;
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
