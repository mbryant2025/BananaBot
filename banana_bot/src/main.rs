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
    top_end_vert: bool, //true if top end is vertical -- future use
    bottom_end_vert: bool, //true if bottom end is vertical -- future use
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

fn reset_boards() {
    BOARDS.lock().unwrap().clear();
}

fn copy_and_push(b: &Board) {
    let b_copy = Board {
        tiles: b.tiles.clone(),
        remaining: b.remaining.clone(),
        bottom_end: b.bottom_end,
        top_end: b.top_end,
        words: b.words.clone(),
        top_end_vert: b.top_end_vert,
        bottom_end_vert: b.bottom_end_vert,
    };
    BOARDS.lock().unwrap().push(b_copy);
}

//recursively construct word and then find word with remaining letters
//paramaters: remaining letters, Board struct
//if no more words can be found that connect or no more remaining letters, add board to boards
fn construct(b: &Board, letters: &str) {
    //if no more letters, add board to boards
    if letters.len() == 0 {
        copy_and_push(&b);
        return;
    }

    //if not correct amout of letters, return
    if b.tiles.len() != MASTER_LETTERS.lock().unwrap().len() {
        return;
    }

    //if no words on board, add first word
    if b.words.len() == 0 {
        let l = find_longest_words(&letters, 3);
        for word in &l {
            let remaining = remove_from_tiles(&word, &letters);
            let mut b2 = Board{
                tiles: b.tiles.clone(),
                remaining: remaining.clone(),
                bottom_end: word.chars().next().unwrap(),
                top_end: word.chars().last().unwrap(),
                words: b.words.clone(),
                bottom_end_vert: b.bottom_end_vert,
                top_end_vert: b.top_end_vert,
            };
            b2.words.push(word.to_string());
            construct(&b2, &remaining);
        }
        return;
    }

    //check if we can make words that link to current board -- if not add to boards
    //otherwise recurse
    let mut made_words = false;

    // //find longest words that can be made from remaining letters
    let l = find_longest_words(&letters, 7);
    //see if these words can form two letter words with the current board
    for word in &l {

        //if word 4 letters or less, skip
        if word.len() <= 4 {
            continue;
        }
        
        if b.top_end_vert {
            let mut two_letter_word = b.top_end.to_string();
            two_letter_word.push_str(&word.chars().last().unwrap().to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: b.bottom_end,
                    top_end: word.chars().next().unwrap(),
                    words: b.words.clone(),
                    bottom_end_vert: b.bottom_end_vert,
                    top_end_vert: true,
                };
                b2.words.push(word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }

            two_letter_word = word.chars().last().unwrap().to_string();
            two_letter_word.push_str(&b.top_end.to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: b.bottom_end,
                    top_end: word.chars().next().unwrap(),
                    words: b.words.clone(),
                    bottom_end_vert: b.bottom_end_vert,
                    top_end_vert: true,
                };
                b2.words.push(word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }
            
        }
        else {
            let mut two_letter_word = b.top_end.to_string();
            two_letter_word.push_str(&word.chars().next().unwrap().to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: b.bottom_end,
                    top_end: word.chars().last().unwrap(),
                    words: b.words.clone(),
                    bottom_end_vert: b.bottom_end_vert,
                    top_end_vert: false,
                };
                b2.words.push(word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }

            two_letter_word = word.chars().next().unwrap().to_string();
            two_letter_word.push_str(&b.top_end.to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: b.bottom_end,
                    top_end: word.chars().last().unwrap(),
                    words: b.words.clone(),
                    bottom_end_vert: b.bottom_end_vert,
                    top_end_vert: false,
                };
                b2.words.push(word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }
        }

        if b.bottom_end_vert {
            let mut two_letter_word = b.bottom_end.to_string();
            two_letter_word.push_str(&word.chars().next().unwrap().to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: word.chars().last().unwrap(),
                    top_end: b.top_end,
                    words: b.words.clone(),
                    bottom_end_vert: true,
                    top_end_vert: b.top_end_vert,
                };
                
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }

            two_letter_word = word.chars().next().unwrap().to_string();
            two_letter_word.push_str(&b.bottom_end.to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: word.chars().last().unwrap(),
                    top_end: b.top_end,
                    words: b.words.clone(),
                    bottom_end_vert: true,
                    top_end_vert: b.top_end_vert,
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }
            
        }
        else {
            let mut two_letter_word = b.bottom_end.to_string();
            two_letter_word.push_str(&word.chars().last().unwrap().to_string());
    
            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: word.chars().next().unwrap(),
                    top_end: b.top_end,
                    words: b.words.clone(),
                    bottom_end_vert: false,
                    top_end_vert: b.top_end_vert,
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }

            two_letter_word = word.chars().last().unwrap().to_string();
            two_letter_word.push_str(&b.bottom_end.to_string());

            //if two letter word is in dictionary, add to board
            if is_valid_two_letter(&two_letter_word) {
                made_words = true;
                let remaining = remove_from_tiles(&word, &letters);
                let mut b2 = Board{
                    tiles: b.tiles.clone(),
                    remaining: remaining.clone(),
                    bottom_end: word.chars().next().unwrap(),
                    top_end: b.top_end,
                    words: b.words.clone(),
                    bottom_end_vert: false,
                    top_end_vert: b.top_end_vert,
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                construct(&b2, &remaining);
            }
        }
    }

    //if no words were made, add board to boards
    if !made_words {
        copy_and_push(&b);
    }

}

fn solve() {

    //check if there are any boards
    if BOARDS.lock().unwrap().len() == 0 {
        //create empty board
        let b = Board {
            tiles: get_master_letters(),
            remaining: get_master_letters(),
            bottom_end: ' ',
            top_end: ' ',
            words: Vec::new(),
            bottom_end_vert: false,
            top_end_vert: false,
        };
        construct(&b, &get_master_letters());
    }
    else {
        let mut recurse_boards = Vec::new();

        for board in BOARDS.lock().unwrap().iter_mut() {
            let board_clone = Board {
                tiles: board.tiles.clone(),
                remaining: board.remaining.clone(),
                bottom_end: board.bottom_end,
                top_end: board.top_end,
                words: board.words.clone(),
                bottom_end_vert: board.bottom_end_vert,
                top_end_vert: board.top_end_vert,
            };
            recurse_boards.push(board_clone);
        }

        for mut board in recurse_boards {
            let diff = remove_from_tiles(&board.tiles, &MASTER_LETTERS.lock().unwrap());
            board.remaining.push_str(&diff);
            board.tiles = MASTER_LETTERS.lock().unwrap().clone();
            construct(&board, &board.remaining);
        }
    }

    purge_old_boards();

    sort_boards();

    println!("-------------------------");

    for board in BOARDS.lock().unwrap().iter_mut() {
        print_board(board);
        println!(" ");
    }
}

//sort BOARDS by remaining letters
fn sort_boards() {
    BOARDS.lock().unwrap().sort_by(|a, b| b.remaining.len().cmp(&a.remaining.len()));
    BOARDS.lock().unwrap().truncate(15);
}

fn purge_old_boards() {
    //remove boards that have less letters than MASTER_LETTERS
    let mut i = 0;
    while i < BOARDS.lock().unwrap().len() {
        if BOARDS.lock().unwrap()[i].tiles.len() < MASTER_LETTERS.lock().unwrap().len() {
            BOARDS.lock().unwrap().remove(i);
        }
        else {
            i += 1;
        }
    }
}

fn help() {
    println!("Welcome to banana_bot!");
    println!("Enter 'q' to quit.");
    println!("Enter 'p' to print the current letters.");
    println!("Enter 'r' to reset the current letters.");
    println!("Enter 'b' to reset the current boards.");
    println!("Enter 's' to solve the current letters.");
    println!("Enter 'h' to print this help message.");
    println!("Enter any other key to add letters to the current letters.");
}

fn is_valid_two_letter(word: &str) -> bool {
    let file = File::open("two_letter_words.txt").expect("Unable to open file");
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


fn init_game() {
    println!("Welcome to banana_bot!");
    println!("Enter 'q' to quit.");
    println!("Enter 'p' to print the current letters.");
    println!("Enter 'r' to reset the current letters.");
    println!("Enter 'b' to reset the current boards.");
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
            "b" => reset_boards(),
            _ => add_letters(),
        }
    }
}

fn print_board(b: &Board) {
    println!("Current letters: {}", b.tiles);
    println!("Remaining letters: {}", b.remaining);
    println!("Words: {:?}", b.words);
}

fn main() {
    init_game();
}