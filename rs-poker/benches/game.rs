#[macro_use]
extern crate criterion;
extern crate rs_poker;

use criterion::Criterion;
use rs_poker::core::Hand;
use rs_poker::holdem::MonteCarloGame;

fn simulate_one_game(c: &mut Criterion) {
  let hands = ["AdAh", "2c2s"]
    .iter()
    .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
    .collect();
  let mut g = MonteCarloGame::new_with_hands(hands).expect("Should be able to create a game.");

  c.bench_function("Simulate AdAh vs 2c2s", move |b| {
    b.iter(|| {
      let r = g.simulate().expect("There should be one best rank.");
      g.reset();
      r
    })
  });
}

criterion_group!(benches, simulate_one_game);
criterion_main!(benches);
