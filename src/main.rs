use rand::seq::SliceRandom;
use std::fmt;
// use regex::Regex;
use clap::Parser;
use std::thread::sleep;
use std::time::Duration;

 const NEW_DECK: [&str; 52] = [
    // Spades
    "🂡", "🂢", "🂣", "🂤", "🂥", "🂦", "🂧", "🂨", "🂩", "🂪", "🂫", "🂭", "🂮",
    // Hearts
    "🂱", "🂲", "🂳", "🂴", "🂵", "🂶", "🂷", "🂸", "🂹", "🂺", "🂻", "🂽", "🂾",
    // Clubs
    "🃑", "🃒", "🃓", "🃔", "🃕", "🃖", "🃗", "🃘", "🃙", "🃚", "🃛", "🃝", "🃞",
    // Diamonds
    "🃁", "🃂", "🃃", "🃄", "🃅", "🃆", "🃇", "🃈", "🃉", "🃊", "🃋", "🃍", "🃎",
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CardDeck {
    cards: Vec<&'static str>,
}

impl CardDeck {
    fn new() -> Self {
        let cards = NEW_DECK.to_vec();
        Self { cards }
    }

    fn new_deck(&mut self) {
        self.cards = NEW_DECK.to_vec();
    }

    fn shuffle(&mut self) {
        self.new_deck();

        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal_one_card(&mut self) -> Option<&'static str> {
        self.cards.pop()
    }

    fn card_rank(card: &str) -> usize {
        return match card {
            "🂡" | "🂱" | "🃑" | "🃁" => 15, // Ace
            "🂢" | "🂲" | "🃒" | "🃂" => 2,
            "🂣" | "🂳" | "🃓" | "🃃" => 3,
            "🂤" | "🂴" | "🃔" | "🃄" => 4,
            "🂥" | "🂵" | "🃕" | "🃅" => 5,
            "🂦" | "🂶" | "🃖" | "🃆" => 6,
            "🂧" | "🂷" | "🃗" | "🃇" => 7,
            "🂨" | "🂸" | "🃘" | "🃈" => 8,
            "🂩" | "🂹" | "🃙" | "🃉" => 9,
            "🂪" | "🂺" | "🃚" | "🃊" => 10,
            "🂫" | "🂻" | "🃛" | "🃋" => 11,
            "🂭" | "🂽" | "🃝" | "🃍" => 12,
            "🂮" | "🂾" | "🃞" | "🃎" => 13,
            _ => 0, // Invalid or unknown card
        }
    }

    #[allow(dead_code)]
    fn card_name(card: &str) -> &str {
        return match card {
            "🂡" | "🂱" | "🃑" | "🃁" => "Ace",
            "🂢" | "🂲" | "🃒" | "🃂" => "2",
            "🂣" | "🂳" | "🃓" | "🃃" => "3",
            "🂤" | "🂴" | "🃔" | "🃄" => "4",
            "🂥" | "🂵" | "🃕" | "🃅" => "5",
            "🂦" | "🂶" | "🃖" | "🃆" => "6",
            "🂧" | "🂷" | "🃗" | "🃇" => "7",
            "🂨" | "🂸" | "🃘" | "🃈" => "8",
            "🂩" | "🂹" | "🃙" | "🃉" => "9",
            "🂪" | "🂺" | "🃚" | "🃊" => "10",
            "🂫" | "🂻" | "🃛" | "🃋" => "Jack",
            "🂭" | "🂽" | "🃝" | "🃍" => "Queen",
            "🂮" | "🂾" | "🃞" | "🃎" => "King",
            _ => "", // Invalid or unknown card
        }
    }
    
    #[allow(dead_code)]
    fn card_suit(card: &str) -> &str {
        match card.chars().last().unwrap() {
            '🂡'..='🂮' => "Spades",
            '🂱'..='🂾' => "Hearts",
            '🃑'..='🃞' => "Clubs",
            '🃁'..='🃎' => "Diamonds",
            _ => "Unknown",
        }
    }

}




struct Hand {
    cards: Vec<&'static str>,
    rank: usize,
}

impl Hand {    
    const HAND_FLUSH_REGEX: &str = "([🂡-🂮]{5})|([🂱-🂾]{5})|([🃁-🃎]{5})|([🃑-🃞]{5})";
    const HAND_STRAIGHT_REGEX: &str = "([🂡🂱🃁🃑][🂮🂾🃎🃞][🂭🂽🃋🃛][🂫🂻🃊🃚][🂪🂺🃉🃙])|[🂮🂾🃎🃞][🂭🂽🃋🃛][🂫🂻🃊🃚][🂪🂺🃉🃙][🂭🂽🃋🃛]|[🂫🂻🃊🃚][🂪🂺🃉🃙][🂭🂽🃋🃛][🂡-🂻🃁-🃛][🂠-🂮]|[🂭🂽🃋🃛][🂫🂻🃊🃚][🂭🂽🃋🃛][🂠-🂮][🂡-🂻]|[🂪🂺🃉🃙][🂫🂻🃊🃚][🂭🂽🃋🃛][🂡-🂻🃁-🃛][🂠-🂮]";
    const HAND_OF_A_KIND_REGEX: &str = "(?:[🂡🂱🃁🃑]{2,4})|(?:[🂮🂾🃎🃞]{2,4})|(?:[🂭🂽🃋🃛]{2,4})|(?:[🂫🂻🃊🃚]{2,4})|(?:[🂪🂺🃉🃙]{2,4})|(?:[🂭🂽🃋🃛]{2,4})|(?:[🂡🂱🃁🃑🂮🂾🃎🃞]{3,4})|(?:[🂭🂽🃋🃛🂫🂻🃊🃚]{3,4})|(?:[🂪🂺🃉🃙🂭🂽🃋🃛]{3,4})|(?:[🂡🂱🃁🃑🂮🂾🃎🃞🂭🂽🃋🃛]{4})|(?:[🂫🂻🃊🃚🂭🂽🃋🃛]{4})|(?:[🂡🂱🃁🃑🂮🂾🃎🃞🂭🂽🃋🃛]{4})|(?:[🂪🂺🃉🃙🂡-🂻🃁-🃛]{4})|(?:[🂠-🂮]{5})";
    const HAND_ROYAL_FLUSH: usize = 0x900000;
    const HAND_STRAIGHT_FLUSH: usize = 0x800000;
    const HAND_FOUR_OF_A_KIND: usize = 0x700000;
    const HAND_FULL_HOUSE: usize = 0x600000;
    const HAND_FLUSH: usize = 0x500000;
    const HAND_STRAIGHT: usize = 0x400000;
    const HAND_THREE_OF_A_KIND: usize = 0x300000;
    const HAND_TWO_PAIR: usize = 0x200000;
    const HAND_PAIR: usize = 0x100000;
    fn new() -> Self {
        let cards = Vec::new();
        let rank = 0;
        Self { cards, rank }
    }

    fn clear(&mut self) {
        self.cards.clear();
        self.rank = 0;
    }

    fn take_card(&mut self, card: &'static str) {
        self.cards.push(card);
    }

    #[allow(dead_code)]
    fn rank(&self) -> usize {
        self.rank
    }

    fn to_string(&self) -> String {
        self.cards.join("")
    }

    fn score(&mut self) {
        // Sort highest rank to lowest
        self.cards.sort_by(|a, b| {
            CardDeck::card_rank(b).cmp(&CardDeck::card_rank(a))
        });


        let hand_string: String = self.to_string();//self.cards.iter().copied().collect();

        let flush_result = regex::Regex::new(Self::HAND_FLUSH_REGEX)
            .unwrap()
            .captures(&hand_string);
        let straight_result = regex::Regex::new(Self::HAND_STRAIGHT_REGEX)
            .unwrap()
            .captures(&hand_string);
        let of_a_kind_result = regex::Regex::new(Self::HAND_OF_A_KIND_REGEX)
            .unwrap()
            .captures(&hand_string);

        if let Some(_flush_result) = flush_result {
            if let Some(straight_result) = straight_result {
                if straight_result.get(1).is_some() {
                    self.rank = Hand::HAND_ROYAL_FLUSH;
                } else {
                    self.rank = Hand::HAND_STRAIGHT_FLUSH;
                }
            } else {
                self.rank = Hand::HAND_FLUSH;
            }

            self.rank |=
                CardDeck::card_rank(self.cards[0]) << 16 | CardDeck::card_rank(self.cards[1]) << 12;
        } else if let Some(_straight_result) = straight_result {
            self.rank = Hand::HAND_STRAIGHT | CardDeck::card_rank(self.cards[0]) << 16
                | CardDeck::card_rank(self.cards[1]) << 12;
        } else if let Some(of_a_kind_result) = of_a_kind_result {
            if of_a_kind_result[0].len() == 8 {
                self.rank = Hand::HAND_FOUR_OF_A_KIND | CardDeck::card_rank(self.cards[0]);
            } else {
                let first_of_a_kind = &of_a_kind_result[0];
                let remaining_cards_index =
                    hand_string.find(first_of_a_kind).unwrap() + first_of_a_kind.len();
                let mut second_of_a_kind_result = None;

                if remaining_cards_index <= 6 {
                    let remaining_cards = &hand_string[remaining_cards_index..];
                    second_of_a_kind_result = regex::Regex::new(Self::HAND_OF_A_KIND_REGEX)
                        .unwrap()
                        .captures(remaining_cards);
                }

                if let Some(second_of_a_kind_result) = second_of_a_kind_result {
                    if (first_of_a_kind.len() == 6 && second_of_a_kind_result[0].len() == 4)
                        || (first_of_a_kind.len() == 4 && second_of_a_kind_result[0].len() == 6)
                    {
                        let three_of_a_kind_card_rank = CardDeck::card_rank(&first_of_a_kind[0..2]);
                        let two_of_a_kind_card_rank = CardDeck::card_rank(&second_of_a_kind_result[0][0..2]);
                        self.rank = Hand::HAND_FULL_HOUSE
                            | (three_of_a_kind_card_rank << 16)
                            | (three_of_a_kind_card_rank << 12)
                            | (three_of_a_kind_card_rank << 8)
                            | (two_of_a_kind_card_rank << 4)
                            | two_of_a_kind_card_rank;
                    } else if first_of_a_kind.len() == 4 && second_of_a_kind_result[0].len() == 4 {
                        let first_pair_card_rank = CardDeck::card_rank(&first_of_a_kind[0..2]);
                        let second_pair_card_rank =
                            CardDeck::card_rank(&second_of_a_kind_result[0][0..2]);
                        let other_card_rank;

                        if first_of_a_kind.chars().next().unwrap() == hand_string.chars().next().unwrap() {
                            if second_of_a_kind_result[0].chars().next().unwrap()
                                == hand_string.chars().nth(4).unwrap()
                            {
                                other_card_rank =
                                    CardDeck::card_rank(&hand_string[8..10]);
                            } else {
                                other_card_rank =
                                    CardDeck::card_rank(&hand_string[4..6]);
                            }
                        } else {
                            other_card_rank =
                                CardDeck::card_rank(&hand_string[0..2]);
                        }

                        self.rank = Hand::HAND_TWO_PAIR
                            | (first_pair_card_rank << 16)
                            | (first_pair_card_rank << 12)
                            | (second_pair_card_rank << 8)
                            | (second_pair_card_rank << 4)
                            | other_card_rank;
                    }
                } else {
                    let of_a_kind_card_rank =
                        CardDeck::card_rank(&first_of_a_kind.chars().take(2).collect::<String>());
                    let mut other_cards_rank = 0;

                    for card in &self.cards {
                        let card_rank = CardDeck::card_rank(card);
                        if card_rank != of_a_kind_card_rank {
                            other_cards_rank = (other_cards_rank << 4) | card_rank;
                        }
                    }

                    if first_of_a_kind.len() == 6 {
                        self.rank = Hand::HAND_THREE_OF_A_KIND
                            | (of_a_kind_card_rank << 16)
                            | (of_a_kind_card_rank << 12)
                            | (of_a_kind_card_rank << 8)
                            | other_cards_rank;
                    } else {
                        self.rank = Hand::HAND_PAIR
                            | (of_a_kind_card_rank << 16)
                            | (of_a_kind_card_rank << 12)
                            | other_cards_rank;
                    }
                }
            }
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cards.join(""))
    }
}

struct Player {
    name: String,
    wins: usize,
    hand_type_counts: [usize; 10],
    hand: Hand,
}

impl Player {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let wins = 0;
        let hand_type_counts = [0; 10];
        let hand = Hand::new();
        Self {
            name,
            wins,
            hand_type_counts,
            hand,
        }
    }

    fn score_hand(&mut self) {
        self.hand.score();
        let hand_type = self.hand.rank >> 20;
        self.hand_type_counts[hand_type as usize] += 1;
    }

    fn describe_hand(&self) -> &'static str {
        match self.hand.rank {
            r if r & Hand::HAND_ROYAL_FLUSH == Hand::HAND_ROYAL_FLUSH => "Royal Flush",
            r if r & Hand::HAND_STRAIGHT_FLUSH == Hand::HAND_STRAIGHT_FLUSH => "Straight Flush",
            r if r & Hand::HAND_FOUR_OF_A_KIND == Hand::HAND_FOUR_OF_A_KIND => "Four of a Kind",
            r if r & Hand::HAND_FULL_HOUSE == Hand::HAND_FULL_HOUSE => "Full House",
            r if r & Hand::HAND_FLUSH == Hand::HAND_FLUSH => "Flush",
            r if r & Hand::HAND_STRAIGHT == Hand::HAND_STRAIGHT => "Straight",
            r if r & Hand::HAND_THREE_OF_A_KIND == Hand::HAND_THREE_OF_A_KIND => "Three of a Kind",
            r if r & Hand::HAND_TWO_PAIR == Hand::HAND_TWO_PAIR => "Two Pair",
            r if r & Hand::HAND_PAIR == Hand::HAND_PAIR => "One Pair",
            _ => "High Card", // Default case or handle it according to your requirements
        }        
    }

    fn won_hand(&mut self) {
        self.wins += 1;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn hand(&self) -> String {
        self.hand.to_string()
    }
}

fn play_hands(players: &mut Vec<Player>) {
    let mut card_deck = CardDeck::new();
    let mut hands_played = 0;
    let mut highest_rank;

    while hands_played < 2000 {
        card_deck.shuffle();

        for player in players.iter_mut() {
            player.hand.clear();
        }

        for _ in 0..5 {
            for player in players.iter_mut() {
                if let Some(card) = card_deck.deal_one_card() {
                    player.hand.take_card(card);
                }
            }
        }

        for player in players.iter_mut() {
            player.score_hand();
        }

        hands_played += 1;

        highest_rank = players
            .iter()
            .map(|player| player.hand.rank)
            .max()
            .unwrap_or_default();

        for player in players.iter_mut() {
            if player.hand.rank == highest_rank {
                player.won_hand();
            }
        }
    }
}


fn play_hands_verbose(players: &mut Vec<Player>, verbose: bool) {
    let mut card_deck = CardDeck::new();
    let mut hands_played = 0;
    let mut highest_rank;

    while hands_played < 2000 {
        card_deck.shuffle();

        if verbose {
            println!("Hands dealt:");
            for player in players.iter_mut() {
                println!("{}: {}", player.name(), player.hand());
            }
            println!("------------------------");
        }

        for player in players.iter_mut() {
            player.hand.clear();
        }

        for _ in 0..5 {
            for player in players.iter_mut() {
                if let Some(card) = card_deck.deal_one_card() {
                    player.hand.take_card(card);
                }
            }
        }

        if verbose {
            println!("Hands after dealing:");
            for player in players.iter_mut() {
                println!("{}: {}", player.name(), player.hand());
            }
            println!("------------------------");
        }

        for player in players.iter_mut() {
            player.score_hand();
            if verbose {
                println!("{}'s Hand: {}", player.name(), player.hand());
                println!("{}'s Hand Description: {}", player.name(), player.describe_hand());
            }
        }

        hands_played += 1;

        highest_rank = players
            .iter()
            .map(|player| player.hand.rank)
            .max()
            .unwrap_or_default();

        for player in players.iter_mut() {
            if player.hand.rank == highest_rank {
                player.won_hand();
                if player.hand.rank == Hand::HAND_ROYAL_FLUSH {
                    println!("Pausing for 3 seconds for a Royal Flush!");
                    sleep(Duration::from_secs(3));
                }
            }
        }

        if verbose {
            println!("Winner(s) of this hand:");
            for player in players.iter() {
                if player.hand.rank == highest_rank {
                    println!("{} won with a {}", player.name(), player.describe_hand());
                } else {
                    println!("{} lost.", player.name());
                }
            }
            println!("------------------------");
        }
    }
}
#[derive(Parser, Debug)]
struct Opts {
    /// Name of Player 1
    #[clap(short='1', long, default_value = "Player1")]
    player1: String,

    /// Name of Player 2
    #[clap(short='2', long, default_value = "Player2")]
    player2: String,

    /// Print hands as they are dealt and display winner/loser information
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    // Parse command-line arguments using clap
    let opts: Opts = Opts::parse();

    // Create and initialize players
    let mut players = vec![Player::new(&opts.player1), Player::new(&opts.player2)];

    // Play hands
    if opts.verbose {
        play_hands_verbose(&mut players, opts.verbose);
    } else {
        play_hands(&mut players);
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_royal_flush() {
        let mut player = Player::new("TestPlayer");
        //player.hand.take_card("🂡");
        //player.hand.take_card("🂱");
        player.hand.cards = vec!["🂺", "🂻", "🂽", "🂾", "🂱"]; // Royal Flush in Hearts
        // Describe each card in the hand
        for card in player.hand.cards.iter() {
            std::println!("{} - {} {}", card, CardDeck::card_name(card), CardDeck::card_suit(card));
        }
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_ROYAL_FLUSH, Hand::HAND_ROYAL_FLUSH);
        

    }
    
    #[test]
    fn test_straight_flush() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂢", "🂣", "🂤", "🂥","🂦"]; // Straight Flush in Spades
        // Describe each card in the hand
        for card in player.hand.cards.iter() {
            std::println!("{} - {} {}", card, CardDeck::card_name(card), CardDeck::card_suit(card));
        }
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_STRAIGHT_FLUSH, Hand::HAND_STRAIGHT_FLUSH);
    }
    
    #[test]
    fn test_four_of_a_kind() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🂻", "🃛", "🂮"]; // Four of a Kind with Aces
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_FOUR_OF_A_KIND, Hand::HAND_FOUR_OF_A_KIND);
    }
    
    #[test]
    fn test_full_house() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🂻", "🃛", "🃛"]; // Full House with Aces over Kings
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_FULL_HOUSE, Hand::HAND_FULL_HOUSE);
    }
    
    #[test]
    fn test_flush() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🂻", "🃛", "🃛"]; // Flush in Hearts
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_FLUSH, Hand::HAND_FLUSH);
    }
    
    #[test]
    fn test_straight() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂢", "🂣", "🂤", "🂥"]; // Straight in Spades
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_STRAIGHT, Hand::HAND_STRAIGHT);
    }
    
    #[test]
    fn test_three_of_a_kind() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🂻", "🃛", "🂮"]; // Three of a Kind with Aces
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_THREE_OF_A_KIND, Hand::HAND_THREE_OF_A_KIND);
    }
    
    #[test]
    fn test_two_pair() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🃛", "🃛", "🂮"]; // Two Pair with Aces and Kings
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_TWO_PAIR, Hand::HAND_TWO_PAIR);
    }
    
    #[test]
    fn test_pair() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂱", "🃛", "🂮", "🂮"]; // Pair with Aces
        player.score_hand();
        assert_eq!(player.hand.rank & Hand::HAND_PAIR, Hand::HAND_PAIR);
    }
    
    #[test]
    fn test_high_card() {
        let mut player = Player::new("TestPlayer");
        player.hand.cards = vec!["🂡", "🂢", "🃛", "🂤", "🂥"]; // High Card
        player.score_hand();
        assert_eq!(player.hand.rank, 0);
    }
}
