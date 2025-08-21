use std::io;

use rand;
use rand::seq::SliceRandom;

// The deck is unlimited in size.
// There are no jokers.
// The Jack/Queen/King all count as 10.
// The the Ace can count as 11 or 1.
// The cards in the list have equal probability of being drawn.
// Cards are not removed from the deck as they are drawn.
const CARDS: [i8; 12] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10];
const MAX: i8 = 21;

fn get_random_card() -> i8 {
    CARDS.choose(&mut rand::thread_rng()).unwrap().clone()
}

fn take_user_input() -> bool {
    let mut g_continue = String::new();
    io::stdin()
        .read_line(&mut g_continue)
        .expect("Failed to read line");
    match g_continue.trim().to_lowercase().as_ref() {
        "y" => true,
        _ => false,
    }
}

#[derive(PartialEq, Eq)]
enum GAME {
    WIN,
    LOOSE,
    DRAW,
    CONTINUE,
}

fn get_sum(arr: &Vec<i8>) -> i8 {
    arr.iter().map(|&i| i).sum()
}

fn decide_game(dealer: &Vec<i8>, user: &Vec<i8>) -> GAME {
    let dealer_sum: i8 = get_sum(dealer);
    let user_sum: i8 = get_sum(user);

    match user_sum {
        _ if user_sum > MAX => GAME::LOOSE,
        _ if dealer_sum > MAX => GAME::WIN,
        _ if user_sum == dealer_sum => GAME::DRAW,
        _ if user_sum > dealer_sum => GAME::WIN,
        _ if dealer_sum > user_sum => GAME::LOOSE,
        _ => GAME::CONTINUE,
    }
}

fn main() {
    let mut dealer: Vec<i8> = vec![];
    let mut user: Vec<i8> = vec![];

    dealer.push(get_random_card());
    dealer.push(get_random_card());
    user.push(get_random_card());
    user.push(get_random_card());
    println!("Dealer got {} & ?", dealer[0]);
    println!(
        "You got {} & {} sum = {}",
        user[0],
        user[1],
        user[0] + user[1]
    );

    let mut decision = decide_game(&dealer, &user);
    while decision == GAME::CONTINUE {}
    if get_sum(&dealer) < 17 {
        dealer.push(get_random_card());
    }
}
