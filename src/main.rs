use lambda_runtime::{error::HandlerError, lambda, Context};
use rayon::prelude::*;
use rs_poker::core::{Deck, FlatDeck, Hand, Rank};
use rs_poker::holdem::MonteCarloGame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
  simple_logger::init_with_level(log::Level::Debug)?;
  lambda!(my_handler);
  Ok(())
}

fn my_handler(
  e: CustomEvent,
  c: Context,
) -> Result<Vec<Vec<(i32, HashMap<String, i32>, HashMap<String, i32>)>>, HandlerError> {
  let num_iters = e.iters;
  let num_players = 6;
  let board_size = 5;
  let mut iters: Vec<Vec<Vec<(i32, Vec<Hand>)>>> =
    vec![vec![vec![(0, vec![]); 10]; num_players]; num_iters];
  let hand = Arc::new(e.hand);
  let board = Arc::new(e.board);
  iters.par_iter_mut().for_each(|p| {
    let mut deck = Deck::default();
    let hand = Arc::clone(&hand);
    let board = Arc::clone(&board);
    let my_hand = Hand::new_from_str(hand.as_str()).unwrap();
    let mut board = Hand::new_from_str(board.as_str()).unwrap();
    for i in 0..my_hand.len() {
      deck.remove(my_hand[i]);
    }
    for i in 0..board.len() {
      deck.remove(board[i]);
    }
    //    if board.len() > 0 && board.len() < board_size {
    if board.len() < board_size {
      let temp_deck = FlatDeck::try_from(deck.clone()).unwrap();
      let add_cards = temp_deck.sample(board_size - board.len());
      for i in 0..add_cards.len() {
        board.push(add_cards[i]);
        deck.remove(add_cards[i]);
      }
    }
    let mut hands = vec![my_hand];
    let temp_deck = FlatDeck::try_from(deck.clone()).unwrap();
    let sample_cards = temp_deck.sample(num_players * 2 - 2);
    for i in 0..sample_cards.len() {
      if i % 2 == 1 {
        continue;
      }
      deck.remove(sample_cards[i]);
      deck.remove(sample_cards[i + 1]);
      hands.push(Hand::new_with_cards(vec![sample_cards[i], sample_cards[i + 1]]));
    }
    let mut game = MonteCarloGame::new_with_hands(hands.clone()).unwrap();
    game.set_board(board.get_cards());
    let result = game.simulate().unwrap();
    let win_rank = match result.1 {
      Rank::HighCard(_) => 1,
      Rank::OnePair(_) => 2,
      Rank::TwoPair(_) => 3,
      Rank::ThreeOfAKind(_) => 4,
      Rank::Straight(_) => 5,
      Rank::Flush(_) => 6,
      Rank::FullHouse(_) => 7,
      Rank::FourOfAKind(_) => 8,
      Rank::StraightFlush(_) => 9,
    };
    p[result.0][0].0 += 1;
    p[result.0][win_rank].0 += 1;
    p[result.0][win_rank].1.push(hands[result.0].clone());
    game.reset();
  });
  let mut result: Vec<Vec<(i32, HashMap<String, i32>, HashMap<String, i32>)>> =
    vec![vec![(0, HashMap::new(), HashMap::new()); 10]; num_players];
  for i in 0..iters.len() {
    for j in 0..result.len() {
      for k in 0..result[j].len() {
        result[j][k].0 += iters[i][j][k].0;
        for l in 0..iters[i][j][k].1.len() {
          let hand = iters[i][j][k].1[l].clone();
          let mut two_cards = vec![
            format!("{}{}", hand[0].value.to_char(), hand[0].suit.to_char()),
            format!("{}{}", hand[1].value.to_char(), hand[1].suit.to_char()),
          ];
          two_cards.sort();
          if !result[j][k].2.contains_key(&two_cards[0]) {
            result[j][k].2.insert(two_cards[0].clone(), 0);
          }
          if !result[j][k].2.contains_key(&two_cards[1]) {
            result[j][k].2.insert(two_cards[1].clone(), 0);
          }
          *result[j][k].2.get_mut(&two_cards[0]).unwrap() += 1;
          *result[j][k].2.get_mut(&two_cards[1]).unwrap() += 1;
          let two_cards_key = format!("{}{}", two_cards[0], two_cards[1]);
          if !result[j][k].1.contains_key(&two_cards_key) {
            result[j][k].1.insert(two_cards_key.clone(), 0);
          }
          *result[j][k].1.get_mut(&two_cards_key).unwrap() += 1;
        }
      }
    }
  }
  Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomEvent {
  iters: usize,
  hand: String,
  board: String,
}
