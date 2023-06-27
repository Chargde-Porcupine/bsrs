use std::{fmt, io};
use std::process::Command;
use std::thread::sleep;
use crate::Rank::*;
use crate::Suit::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

/*
Cheat (also known as Bullshit or I Doubt It[3]) is a card game where the players aim to get rid of all of their cards.[4][5] It is a game of deception, with cards being played face-down and players being permitted to lie about the cards they have played. A challenge is usually made by players calling out the name of the game, and the loser of a challenge has to pick up every card played so far. Cheat is classed as a party game.[4] As with many card games, cheat has an oral tradition and so people are taught the game under different names.
 */
//by: Matthew Epshtein, 2023

fn main() {
    println!("Hello, world!");
    let mut game = Game::new(4);
    match game {
        Ok(mut game) => {
            println!("{:?}", game.hands);
            loop {
                let mut continuer = false;
                match game.display_state() {
                    Ok(true) => {},
                    Ok(false) => {break;},
                    Err(err) => panic!("{}", err)
                }
                game.current_play = match game.build_play() {
                    Ok(play) => play,
                    Err(err) => {
                        println!("Error: {}", err);
                        continuer = true;
                        game.current_play
                    }
                };
                if continuer { continue; }
                game.next_rank = game.next_rank.next();
                game.pile.append(&mut game.current_play.cards);
                match game.get_contest() {
                    Ok(_) => {},
                    Err(err) => { panic!("{}", err); }
                }
                game.next_player = if game.next_player == game.amount_of_players { 1 } else { game.next_player + 1 };
                println!("Press the ENTER key to continue to player {}'s turn!", game.next_player);
                io::stdin().read_line(&mut "".to_string()).expect("TODO: panic message");
            }

            },
        Err(error) => println!("{}", error)
    }
}

#[derive(Clone,Copy, PartialEq, Debug)]
struct Card{
    suit:Suit,
    rank: Rank
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Card{
    fn to_str(&self) -> String{
        format!("{} of {}", self.rank.to_str(), self.suit.to_str())
    }
}

struct Play{
    cards: Vec<Card>,
    attested_rank: Rank,
    attested_amount: i32,
    contested: bool
}

impl Play {
    //check validity of play
    fn check(& self) -> bool{
        if self.cards.len() != self.attested_amount as usize{
            return false;
        }
        for card in &self.cards {
            if card.rank != self.attested_rank {
                return false;
            }
        }
        return true;
    }
}

struct Game{
    pile: Vec<Card>,
    hands: Vec<Vec<Card>>,
    current_play: Play,
    next_rank: Rank,
    next_player: i32,
    amount_of_players: i32
}

impl Game {
    fn new(player_amount: i32) -> Result<Game, &'static str>{
        let mut first_up = 1;

        // This code is not my own
        // Attribution: https://github.com/locka99/deckofcards-rs/blob/master/src/card.rs
        // lines 150-201
        let mut cards: Vec<Card> = vec![
            Card { suit: Spades, rank: Two },
            Card { suit: Spades, rank: Three },
            Card { suit: Spades, rank: Four },
            Card { suit: Spades, rank: Five },
            Card { suit: Spades, rank: Six },
            Card { suit: Spades, rank: Seven },
            Card { suit: Spades, rank: Eight },
            Card { suit: Spades, rank: Nine },
            Card { suit: Spades, rank: Ten },
            Card { suit: Spades, rank: Jack },
            Card { suit: Spades, rank: Queen },
            Card { suit: Spades, rank: King },
            Card { suit: Spades, rank: Ace },
            Card { suit: Hearts, rank: Two },
            Card { suit: Hearts, rank: Three },
            Card { suit: Hearts, rank: Four },
            Card { suit: Hearts, rank: Five },
            Card { suit: Hearts, rank: Six },
            Card { suit: Hearts, rank: Seven },
            Card { suit: Hearts, rank: Eight },
            Card { suit: Hearts, rank: Nine },
            Card { suit: Hearts, rank: Ten },
            Card { suit: Hearts, rank: Jack },
            Card { suit: Hearts, rank: Queen },
            Card { suit: Hearts, rank: King },
            Card { suit: Hearts, rank: Ace },
            Card { suit: Diamonds, rank: Two },
            Card { suit: Diamonds, rank: Three },
            Card { suit: Diamonds, rank: Four },
            Card { suit: Diamonds, rank: Five },
            Card { suit: Diamonds, rank: Six },
            Card { suit: Diamonds, rank: Seven },
            Card { suit: Diamonds, rank: Eight },
            Card { suit: Diamonds, rank: Nine },
            Card { suit: Diamonds, rank: Ten },
            Card { suit: Diamonds, rank: Jack },
            Card { suit: Diamonds, rank: Queen },
            Card { suit: Diamonds, rank: King },
            Card { suit: Diamonds, rank: Ace },
            Card { suit: Clubs, rank: Two },
            Card { suit: Clubs, rank: Three },
            Card { suit: Clubs, rank: Four },
            Card { suit: Clubs, rank: Five },
            Card { suit: Clubs, rank: Six },
            Card { suit: Clubs, rank: Seven },
            Card { suit: Clubs, rank: Eight },
            Card { suit: Clubs, rank: Nine },
            Card { suit: Clubs, rank: Ten },
            Card { suit: Clubs, rank: Jack },
            Card { suit: Clubs, rank: Queen },
            Card { suit: Clubs, rank: King },
            Card { suit: Clubs, rank: Ace }
        ];
        // End of "stolen" code

        //shuffle the deck
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        //deal out them cards
        let mut hands: Vec<Vec<Card>> = vec![];
        for _i in 0..player_amount {
            hands.push(vec![])
        }
        let mut index:usize = 0;
        for card in cards{
            let hand = hands.get_mut(index);
            match hand{
                Some(hand) => {
                    if card.rank == Ace && card.suit == Spades {
                        first_up = if index == (player_amount - 1) as usize {1} else {index + 1};
                        continue;
                    }
                    hand.push(card);
                },
                None => {
                    return Err("Hand failed");
                }
            }
            if index < (player_amount - 1) as usize {index += 1;} else {index = 0}
        }

        //the first play cannot be challenged
        return Ok(Game{
            pile: vec![],
            hands,
            current_play: Play{
                cards: vec![Card{rank:Ace,suit:Spades}],
                attested_rank: Ace,
                attested_amount: 1,
                contested: false
            },
            next_rank: Two,
            next_player: first_up as i32,
            amount_of_players: player_amount
        })
    }

    fn build_play(&mut self) -> Result<Play, &'static str>{
        let hand = self.hands.get_mut((self.next_player - 1) as usize);
        let mut cards = vec![];
        match hand{
            Some(hand) => {
                let mut temp_input = String::new();
                while  cards.len() < 1 || temp_input.trim() != "q" && hand.len() > 0{
                    if cards.len() >= 1 {
                        println!("Hand:");
                        let mut index = 0;
                        for card in &mut *hand{
                            print!("{}:", index);
                            println!("{}", card);
                            index+=1;
                        }
                    }
                    println!("Rank: {}", self.next_rank.to_str());
                    temp_input = String::new();
                    println!("Enter the index of a card you want to play, [r]emove the last-picked card, or [q]uit to stop picking cards");
                    println!("You have picked {} cards so far", cards.len());
                    println!("You have picked the following cards so far: ");
                    for card in &mut *cards{
                        println!("{} ", card)
                    }
                    io::stdin().read_line(&mut temp_input)?;
                    if temp_input.trim() == "q" {
                        continue;
                    }

                    if temp_input.trim() == "r"{
                        match cards.pop() {
                            Some(card) => {println!("Removed card {} from list!", card); continue;}
                            None => {println!("Cannot remove from empty list!"); continue;}
                        }
                    }
                    //now we parse
                    let index:u32 = match temp_input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid Input; NaN");
                            continue;
                        }
                    };
                    if index < 0 || index > (hand.len() -1 )as u32{
                        println!("Invalid Input: Index out of range");
                        continue;
                    }
                    let card = hand.get(index as usize);
                    match card {
                        Some(card) => {cards.push(*card); hand.remove(index as usize);},
                        None => {return Err("Cannot find card");}
                    }
                }
            }
            None => {
                return Err("Cannot find hand");
            }
        }
        let attested_amount = cards.len() as i32;
        println!("Cards to play: ");
        for card in cards.to_owned(){
            println!("{}", card);
        }
        //TODO: Maybe allow for an undo?
        return Ok(Play{
            cards,
            attested_rank: self.next_rank,
            attested_amount,
            contested: false,
        })
    }

    fn get_contest(&mut self) -> Result<bool, &'static str> {
        let output = Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        print!("{}", String::from_utf8_lossy(&output.stdout));
        println!("Player {} attests that they played {} cards, all of them {}s", self.next_player,self.current_play.attested_amount, self.current_play.attested_rank.to_str());
        let mut temp_input = String::new();
        let mut contesting = false;
        let mut contester = 0;
        for player in 0..self.amount_of_players{
            if player == self.next_player - 1 {continue;}
            println!("Does player {} wish to contest? [y/n]", player+1);
            io::stdin().read_line(&mut temp_input)?;
            if temp_input.trim() == "y"{
                contester = player;
                contesting = true;
            }
            //this may be improvable, but rn it doesnt work if i put it in the above for loop
            if contesting {break;}
        }
        if !contesting {
            return Ok(true);
        }
        if self.current_play.check() {
            println!("A false accusation has been made, and player {} is responsible!", contester + 1);
            //maybe reveal the offender's cards!
            let hand = self.hands.get_mut(contester as usize);
            return match hand {
                Some(hand) => {
                    hand.append(&mut self.pile);
                    self.current_play.contested = true;
                    return Ok(true);
                },
                //me when the police
                None => Err("Failed to locate hand")
            };
        }
        println!("The lie has been revealed, all thanks to player {}!", contester + 1);
        let hand = self.hands.get_mut((self.next_player -  1) as usize);
        match hand{
            Some(hand) => {
                hand.append(&mut self.pile);
                self.current_play.contested = true;
                Ok(false)
            }
            None => return Err("Failed to locate hand")
        }
    }

    fn check_win(&self) -> Result<i32, &'static str>{
        for index in 0..self.amount_of_players{
            let hand = self.hands.get((index as usize));
            match hand {
                Some(hand) => {
                    if hand.len() == 0 {
                        return Ok(index + 1);
                    }
                },
                None => {return Err("Fail to get hand");}
            };
        }
        return Err("No win yet!");
    }

    fn display_state(&self) -> Result<bool, &'static str>{
        let output = Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        print!("{}", String::from_utf8_lossy(&output.stdout));

        match self.check_win() {
            Ok(player) => {
                println!("Player {} has disposed of all of their cards, therefore winning! Whether they did it fairly shall stay a mystery for the ages...", player);
                println!("Press ENTER to continue.");
                //TODO: Panic message!
                io::stdin().read_line(&mut "".to_string()).expect("TODO: panic message");
                return Ok(false);
            },
            Err(err) => if err == "Fail to get hand" {
                return Err(err)
            }
        }

        println!("Player {}'s turn", self.next_player);
        println!("Rank: {}", self.next_rank.to_str());
        let hand = self.hands.get((self.next_player - 1) as usize);
        match hand {
            Some(hand) => {
                println!("Hand:");
                let mut index = 0;
                for card in hand{
                    print!("{}:", index);
                    println!("{}", card);
                    index+=1;
                }
            }
            None => return Err("Unable to retrieve hand")
        }
        Ok(true)
    }


}

#[derive(Clone,Copy,PartialEq, Debug)]
enum Rank {
    Ace,One,Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten,Jack,Queen,King
}

impl  Rank {
    fn next(&self) -> Rank{
        match self {
            Ace => One,
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => Jack,
            Jack => Queen,
            Queen => King,
            King => Ace
        }
    }
    fn to_str(&self) -> String{
        format!(
            "{}", match self {
                Ace => "Ace",
                One => "One",
                Two => "Two",
                Three => "Three",
                Four => "Four",
                Five => "Five",
                Six => "Six",
                Seven => "Seven",
                Eight => "Eight",
                Nine => "Nine",
                Ten => "Ten",
                Jack => "Jack",
                Queen => "Queen",
                King => "King"
            }
        )
    }

    fn from_str(from:&'static str) -> Result<Rank, &'static str>{
        match from.to_lowercase().trim() {
            "ace" |"a"  => Ok(Ace),
            "two" | "2" => Ok(Two),
            "three"| "3" => Ok(Three),
            "four"| "4" => Ok(Four),
            "five"| "5" => Ok(Five),
            "six"| "6" => Ok(Six),
            "seven"| "7" => Ok(Seven),
            "eight"| "8" => Ok(Eight),
            "nine"| "9" => Ok(Nine),
            "ten"| "10" => Ok(Ten),
            "jack"| "j" => Ok(Jack),
            "queen"| "q" => Ok(Queen),
            "king"| "k" => Ok(King),

            _ => Err("Invalid Input")
        }
    }

}


#[derive(Clone,Copy,PartialEq, Debug)]
enum Suit{
    Spades, Diamonds, Hearts, Clubs
}

impl Suit {
    fn to_str(&self) -> String{
        format!("{}", match self{
            Spades => "Spades",
            Diamonds => "Diamonds",
            Hearts => "Hearts",
            Clubs => "Clubs"
        })
    }
}