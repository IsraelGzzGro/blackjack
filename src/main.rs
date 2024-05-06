use rand::seq::SliceRandom;
use std::io;

//This is a basic framework for Blackjack using vecs and the rand crate's shuffle function.

#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Copy, Clone)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    fn value(&self) -> i32 {
        match self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();

    for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        for &rank in &[
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ] {
            deck.push(Card::new(rank, suit));
        }
    }

    // Shuffle the deck
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut player_hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    // Deal two cards to the player and dealer
    for _ in 0..2 {
        player_hand.push(deck.pop().unwrap());
        dealer_hand.push(deck.pop().unwrap());
    }

    println!("Player's hand: {:?}", player_hand);
    println!("Dealer's hand: {:?}", dealer_hand);

    let player_score: i32 = player_hand.iter().map(|card| card.value()).sum();
    let dealer_score: i32 = dealer_hand.iter().map(|card| card.value()).sum();

    println!("Player's score: {}", player_score);
    println!("Dealer's score: {}", dealer_score);

    // further game logic can go here, such as hitting, standing, etc.
}

