#![allow(dead_code)]
extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    Game::start();
}

// TODO: IMPLEMENT FILE METHODS
// struct FileHandler {
//     file_name: &'static str,
// }
//
// impl FileHandler {
//     fn new(file_name: &'static str) -> FileHandler {
//         FileHandler {
//             file_name: file_name,
//         }
//     }
//
//     fn write_to_file(&self,) {
//         let line: String = format!("{} {} {}")
//     }
// }


struct GameBoard {
    board: [Option<&'static str>; 9],
}

struct Player {
    name: String,
    games_won: usize,
    games_lost: usize,
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard { board: [None; 9] }
    }

    fn display_board(&self) -> String {
        let mut board: String = String::new();
        board = board + " _   _   _\n";
        for i in 0..self.board.len() {
            if i % 3 == 0 && i != 0 {
                board = board + &format!("\n|{:?}|", self.board[i]);
            } else {
                board = board + &format!("|{:?}|", self.board[i]);
            }
        }
        board = board + "\n _   _   _";
        board
    }
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            games_won: 0,
            games_lost: 0,
        }
    }

    fn player_stats(&self) -> String {
        format!("Name: {}\nGames won: {}\nGames lost: {}",
                self.name.trim(),
                self.games_won,
                self.games_lost)
    }
}
struct Game;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
impl Game {
    fn start() {
        let mut board = GameBoard::new();
        board.board[2] = Some("x");
        println!("{}", board.display_board());
        println!("Enter a name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).ok().expect("Could not read input.");
        let player = Player::new(name);

        println!("Which icon would you like to use (x, o)? Enter x or o.");
        let mut icon = String::new();
        let player_icon;
        let comp_icon;


        loop {
            io::stdin().read_line(&mut icon).ok().expect("");
            icon = icon.trim().to_lowercase().to_owned();
            if icon == "x" {
                player_icon = "x";
                comp_icon = "o";
                break;
            } else if icon == "o" {
                player_icon = "o";
                comp_icon = "x";
                break;
            }
            icon = "".to_owned();
            println!("Incorrect icon entered.");
            println!("Which icon would you like to use (x, o)? Enter x or o.");
        }
        let instructions = "Enter the number corresponding to the block you would like to \
                            play.\n0 represents the first block, 1 the second, 2 the third and so \
                            on.\nNB: You can only make a play in a block that is unoccupied";
        println!("You choose icon: {}\n{}", player_icon, instructions);

        let play_first = rand::thread_rng().gen_range(0, 2);
        if play_first == 0 {
            println!("Player's turn: ");
            let mut index_str = String::new();
            io::stdin().read_line(&mut index_str).ok();
            let mut index = index_str.trim().parse::<usize>().unwrap();
            loop {
                if let Some(i) = board.board[index] {
                    println!("That space is occupied or doesnt exist. Enter another block.");
                    index_str = "".to_owned();
                    io::stdin().read_line(&mut index_str).ok();
                    index = index_str.trim().parse::<usize>().unwrap();
                } else {
                    player_play(index, player_icon, &mut board);
                    println!("{}", board.display_board());
                    break;
                }
            }
            // TODO: IMPLEMENT error handling
            // player_play(index.unwrap(), player_icon);
        } else {
            comp_play(comp_icon, &mut board);
        }
    }
}

fn player_play(i: usize, player_icon: &'static str, board: &mut GameBoard) {
    board.board[i] = Some(player_icon);
}

fn comp_play(comp_icon: &'static str, board: &mut GameBoard) {
    let mut index = rand::thread_rng().gen_range(0, 8);
    loop {
        if let Some(i) = board.board[index] {
            index = rand::thread_rng().gen_range(0, 8);
        } else {
            board.board[index] = Some(comp_icon);
        }
    }
}
