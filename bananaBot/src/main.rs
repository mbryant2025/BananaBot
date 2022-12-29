use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use lazy_static::lazy_static;
use std::sync::Mutex;

struct Board {
    tiles: String, //all tiles currently in board
    remaining: String, //not linked
    bottom_end: char, //can be last letter of next word
    top_end: char, //can be first letter of next word
    words: Vec<String>, //linked words
}

//All letters user has logged so far
lazy_static! {
    static ref MASTER_LETTERS: Mutex<String> = Mutex::new(String::new());
}

//Currently solved boards
lazy_static! {
    static ref BOARDS: Mutex<Vec<Board>> = Mutex::new(Vec::new());
}

fn add_letters() {
    //read user input
    let mut letters = String::new();
    println!("Enter letters: ");
    io::stdin().read_line(&mut letters).expect("Failed to read line");
    let letters = letters.trim();
    MASTER_LETTERS.lock().unwrap().push_str(letters);
}

fn print_master_letters() {
    println!("Master Letters: {}", MASTER_LETTERS.lock().unwrap());
}

fn get_master_letters() -> String {
    MASTER_LETTERS.lock().unwrap().to_string()
}

fn reset_master_letters() {
    MASTER_LETTERS.lock().unwrap().clear();
    BOARDS.lock().unwrap().clear();
}

fn solve() {
    //check if there are any boards
    if BOARDS.lock().unwrap().len() == 0 {
        //if not, create some with two foundational words
        let l = find_longest_words(&MASTER_LETTERS.lock().unwrap(), 5);

        for word in &l {
            let remaining_base = remove_from_tiles(&word, &MASTER_LETTERS.lock().unwrap());
            //since we want words that connect to the base word,
            //we need to find words that start with the first letter of the base word or end with the same letter
            let l2 = find_longest_words_starting_with(&remaining_base, word.chars().next().unwrap(), 3);
            let l3 = find_longest_words_ending_with(&remaining_base, word.chars().last().unwrap(), 3);

            for word2 in &l2 {
                let remaining = remove_from_tiles(&word2, &remaining_base);
                let mut b = Board {
                    tiles: get_master_letters(),
                    remaining: remaining,
                    bottom_end: word2.chars().last().unwrap(),//first letter of word
                    top_end: word.chars().last().unwrap(),//last letter of word
                    words: Vec::new(),
                };
                b.words.push(word.to_string());
                b.words.push(word2.to_string());
                BOARDS.lock().unwrap().push(b);
            }

            for word3 in &l3 {
                let remaining = remove_from_tiles(&word3, &remaining_base);
                let mut b = Board {
                    tiles: get_master_letters(),
                    remaining: remaining,
                    bottom_end: word.chars().next().unwrap(),//first letter of word
                    top_end: word3.chars().next().unwrap(),//last letter of word
                    words: Vec::new(),
                };
                b.words.push(word.to_string());
                b.words.push(word3.to_string());
                BOARDS.lock().unwrap().push(b);
            }
            
        }

        //If no boards found yet, create some with longest words
        if BOARDS.lock().unwrap().len() == 0 {
            for word in &l {
                let remaining = remove_from_tiles(&word, &MASTER_LETTERS.lock().unwrap());
                let mut b = Board {
                    tiles: get_master_letters(),
                    remaining: remaining,
                    bottom_end: word.chars().next().unwrap(),//first letter of word
                    top_end: word.chars().last().unwrap(),//last letter of word
                    words: Vec::new(),
                };
                b.words.push(word.to_string());
                BOARDS.lock().unwrap().push(b);
                
            }
        }

        //If still no boards found, create one with remaining letters
        if BOARDS.lock().unwrap().len() == 0 {
            let b = Board {
                tiles: get_master_letters(),
                remaining: get_master_letters(),
                bottom_end: ' ',
                top_end: ' ',
                words: Vec::new(),
            };
            BOARDS.lock().unwrap().push(b);
        }
    }
    else {
        //go through boards and add update letters

    }

    for board in BOARDS.lock().unwrap().iter_mut() {
        //find longest words that can be added to the board
        print_board(board);
        println!(" ");
    }

}

fn help() {
    println!("Welcome to BananaBot!");
    println!("Enter 'q' to quit.");
    println!("Enter 'p' to print the current letters.");
    println!("Enter 'r' to reset the current letters.");
    println!("Enter 's' to solve the current letters.");
    println!("Enter 'h' to print this help message.");
    println!("Enter any other key to add letters to the current letters.");
}

fn is_valid(word: &str) -> bool {
    let file = File::open("words.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line == word {
                return true;
            }
        }
    }
    false
}

fn is_in_tiles(word: &str, tiles: &str) -> bool {
    let mut tiles = tiles.to_string();
    for c in word.chars() {
        if let Some(i) = tiles.find(c) {
            tiles.remove(i);
        } else {
            return false;
        }
    }
    true
}

fn remove_from_tiles(word: &str, tiles: &str) -> String {
    let mut tiles = tiles.to_string();
    for c in word.chars() {
        if let Some(i) = tiles.find(c) {
            tiles.remove(i);
        }
    }
    tiles
}

fn find_longest_words(tiles: &str, num: usize) -> Vec<String> {
    let mut longest = Vec::new();
    let file = File::open("words.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if is_in_tiles(&line, tiles) {
                if longest.len() < num {
                    longest.push(line);
                } else {
                    for i in 0..longest.len() {
                        if line.len() > longest[i].len() {
                            longest[i] = line;
                            break;
                        }
                    }
                }
            }
        }
    }
    longest
}

//Note: char not in tiles
fn find_longest_words_starting_with(tiles: &str, letter: char, num: usize) -> Vec<String> {
    let tiles = tiles.to_string() + &letter.to_string();
    let mut longest = Vec::new();
    let file = File::open("words.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if is_in_tiles(&line, &tiles) && line.chars().next().unwrap() == letter {
                if longest.len() < num {
                    longest.push(line);
                } else {
                    for i in 0..longest.len() {
                        if line.len() > longest[i].len() {
                            longest[i] = line;
                            break;
                        }
                    }
                }
            }
        }
    }
    longest
}

//Note: char not in tiles
fn find_longest_words_ending_with(tiles: &str, letter: char, num: usize) -> Vec<String> {
    let tiles = tiles.to_string() + &letter.to_string();
    let mut longest = Vec::new();
    let file = File::open("words.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if is_in_tiles(&line, &tiles) && line.chars().last().unwrap() == letter {
                if longest.len() < num {
                    longest.push(line);
                } else {
                    for i in 0..longest.len() {
                        if line.len() > longest[i].len() {
                            longest[i] = line;
                            break;
                        }
                    }
                }
            }
        }
    }
    longest
}


fn init_game() {
    println!("Welcome to BananaBot!");
    println!("Enter 'q' to quit.");
    println!("Enter 'p' to print the current letters.");
    println!("Enter 'r' to reset the current letters.");
    println!("Enter 's' to solve the current letters.");
    println!("Enter 'h' to print this help message.");

    println!("Enter any other key to add letters to the current letters.");
    println!("");
    println!("Enter the letters you have to start the game.");
    println!("");
    add_letters();

    loop {
        println!("");
        let mut input = String::new();
        println!("Enter command: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "q" => break,
            "p" => print_master_letters(),
            "r" => reset_master_letters(),
            "s" => solve(),
            "h" => help(),
            _ => add_letters(),
        }
    }
}

fn print_board(b: &Board) {
    println!("Current letters: {}", b.tiles);
    println!("Remaining letters: {}", b.remaining);
    println!("Bottom end: {}", b.bottom_end);
    println!("Top end: {}", b.top_end);
    println!("Words: {:?}", b.words);
}

fn main() {


    init_game();

}