use crate::core::card::*;
use std::collections::HashSet;
use std::ops::Index;
use std::ops::{RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

/// Struct to hold cards.
///
/// This doesn't have the ability to easily check if a card is
/// in the hand. So do that before adding/removing a card.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hand {
  /// Where all the cards are placed un-ordered.
  cards: Vec<Card>,
}

impl Hand {
  /// Create the default empty hand.
  #[must_use]
  pub fn default() -> Self {
    Self { cards: Vec::with_capacity(5) }
  }
  /// Create the hand with specific hand.
  #[must_use]
  pub fn new_with_cards(cards: Vec<Card>) -> Self {
    Self { cards }
  }

  /// From a str create a new hand.
  ///
  /// # Examples
  ///
  /// ```
  /// use rs_poker::core::Hand;
  /// let hand = Hand::new_from_str("AdKd").unwrap();
  /// ```
  ///
  /// Anything that can't be parsed will return an error.
  ///
  /// ```
  /// use rs_poker::core::Hand;
  /// let hand = Hand::new_from_str("AdKx");
  /// assert!(hand.is_err());
  /// ```
  pub fn new_from_str(hand_string: &str) -> Result<Self, String> {
    // Get the chars iterator.
    let mut chars = hand_string.chars();
    // Where we will put the cards
    //
    // We make the assumption that the hands will have 2 plus five cards.
    let mut cards: HashSet<Card> = HashSet::with_capacity(7);

    // Keep looping until we explicitly break
    loop {
      // Now try and get a char.
      let vco = chars.next();
      // If there was no char then we are done.
      if vco == None {
        break;
      } else {
        // If we got a value char then we should get a
        // suit.
        let sco = chars.next();
        // Now try and parse the two chars that we have.
        let v = vco
          .and_then(Value::from_char)
          .ok_or_else(|| format!("Couldn't parse value {}", vco.unwrap_or('?')))?;
        let s = sco
          .and_then(Suit::from_char)
          .ok_or_else(|| format!("Couldn't parse suit {}", sco.unwrap_or('?')))?;

        let c = Card { value: v, suit: s };
        if !cards.insert(c) {
          // If this card is already in the set then error out.
          return Err(format!("This card has already been added {}", c));
        }
      }
    }

    if chars.next() != None {
      return Err(String::from("Extra un-used chars found."));
    }

    let mut cv: Vec<Card> = cards.into_iter().collect();

    cv.reserve(7);
    Ok(Self { cards: cv })
  }

  pub fn get_cards(&self) -> Vec<Card> {
    (*self.cards).to_owned()
  }

  /// Add card at to the hand.
  /// No verification is done at all.
  pub fn push(&mut self, c: Card) {
    self.cards.push(c);
  }
  /// Truncate the hand to the given number of cards.
  pub fn truncate(&mut self, len: usize) {
    self.cards.truncate(len)
  }
  /// How many cards are in this hand so far ?
  #[must_use]
  pub fn len(&self) -> usize {
    self.cards.len()
  }
  /// Are there any cards at all ?
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.cards.is_empty()
  }
  /// Create an iter on the cards.
  #[must_use]
  pub fn iter(&self) -> Iter<Card> {
    self.cards.iter()
  }
}

/// Allow indexing into the hand.
impl Index<usize> for Hand {
  type Output = Card;
  #[must_use]
  fn index(&self, index: usize) -> &Card {
    &self.cards[index]
  }
}

/// Allow the index to get refernce to every card.
impl Index<RangeFull> for Hand {
  type Output = [Card];
  #[must_use]
  fn index(&self, range: RangeFull) -> &[Card] {
    &self.cards[range]
  }
}

impl Index<RangeTo<usize>> for Hand {
  type Output = [Card];
  #[must_use]
  fn index(&self, index: RangeTo<usize>) -> &[Card] {
    &self.cards[index]
  }
}
impl Index<RangeFrom<usize>> for Hand {
  type Output = [Card];
  #[must_use]
  fn index(&self, index: RangeFrom<usize>) -> &[Card] {
    &self.cards[index]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::card::Card;

  #[test]
  fn test_add_card() {
    let mut h = Hand::default();
    let c = Card { value: Value::Three, suit: Suit::Spade };
    h.push(c);
    // Make sure that the card was added to the vec.
    //
    // This will also test that has len works
    assert_eq!(1, h.len());
  }

  #[test]
  fn test_index() {
    let mut h = Hand::default();
    h.push(Card { value: Value::Four, suit: Suit::Spade });
    // Make sure the card is there
    assert_eq!(Card { value: Value::Four, suit: Suit::Spade }, h[0]);
  }
  #[test]
  fn test_parse_error() {
    assert!(Hand::new_from_str("BAD").is_err());
    assert!(Hand::new_from_str("Adx").is_err());
  }

  #[test]
  fn test_parse_one_hand() {
    let h = Hand::new_from_str("Ad").unwrap();
    assert_eq!(1, h.len())
  }
  #[test]
  fn test_parse_empty() {
    let h = Hand::new_from_str("").unwrap();
    assert!(h.is_empty());
  }
}
