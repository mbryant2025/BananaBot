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
    //locations of sarting letter of words plus their direction
    //(0,0) is top left corner
    word_positions: Vec<(i32, i32, i32)>, //positions of words ex. [1,5,1] means word starts at position 1,5 and is vertical
}

//Number of branches to search
static DEEP_DEPTH: i32 = 7;
static SHALLOW_DEPTH: i32 = 3;

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
        word_positions: b.word_positions.clone(),
    };
    BOARDS.lock().unwrap().push(b_copy);
}

//recursively construct word and then find word with remaining letters
//paramaters: remaining letters, Board struct
//if no more words can be found that connect or no more remaining letters, add board to boards
fn construct(b: &Board, letters: &str, depth : i32) {

    //if not correct amout of letters, return
    //should never satisfy this if statement
    if b.tiles.len() != MASTER_LETTERS.lock().unwrap().len() {
        return;
    }

    //if no more letters, add board to boards
    if letters.len() == 0 {
        copy_and_push(&b);
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
                word_positions: b.word_positions.clone(),
            };
            b2.words.push(word.to_string());
            b2.word_positions.push((0,0,0));
            construct(&b2, &remaining, depth);
        }
        return;
    }

    //check if we can make words that link to current board -- if not add to boards
    //otherwise recurse
    let mut made_words = false;

    // //find longest words that can be made from remaining letters
    let l = find_longest_words(&letters, depth.try_into().unwrap());
    //see if these words can form two letter words with the current board
    for word in &l {

        //if word 4 letters or less, skip
        if word.len() <= 4 {
            continue;
        }

        //the below struture is slightly redundant in terms of line count
        //but it makes modifications easier in the future
        
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
                    word_positions: b.word_positions.clone(),
                };
                
                b2.remaining = remaining.clone();
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[b.word_positions.len()-1].0 as i32;
                let prev_y = b.word_positions[b.word_positions.len()-1].1 as i32;
                //length of word
                let len = word.len() as i32;
                b2.word_positions.push((prev_x + 1, prev_y + 1 - len , 1)); //up = less y ((0,0) is top left)
                b2.words.push(word.to_string());
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                
                b2.remaining = remaining.clone();
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[b.word_positions.len()-1].0 as i32;
                let prev_y = b.word_positions[b.word_positions.len()-1].1 as i32;
                //length of word
                let len = word.len() as i32;
                b2.word_positions.push((prev_x - 1, prev_y + 1 - len , 1)); //up = less y ((0,0) is top left)
                b2.words.push(word.to_string());
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                
                b2.remaining = remaining.clone();
                //length of word
                let len = b.words[b.words.len()-1].len() as i32;
                //adding to top end
                //this is a horizontal word so the third coordinate is 0
                let prev_x = b.word_positions[b.word_positions.len()-1].0 + len - 1 as i32;
                let prev_y = b.word_positions[b.word_positions.len()-1].1 as i32;
                
                b2.word_positions.push((prev_x, prev_y + 1 , 0)); //up = less y ((0,0) is top left)
                b2.words.push(word.to_string());
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                
                b2.remaining = remaining.clone();
                //length of word
                let len = b.words[b.words.len()-1].len() as i32;
                //adding to top end
                //this is a horizontal word so the third coordinate is 0
                let prev_x = b.word_positions[b.word_positions.len()-1].0 + len - 1 as i32;
                let prev_y = b.word_positions[b.word_positions.len()-1].1 as i32;
                
                b2.word_positions.push((prev_x, prev_y - 1 , 0)); //up = less y ((0,0) is top left)
                b2.words.push(word.to_string());

                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                //length of word
                let len = word.len() as i32;
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[0].0 as i32;
                let prev_y = b.word_positions[0].1 + len - 1 as i32;
                
                b2.word_positions.insert(0, (prev_x + 1, prev_y , 1)); //up = less y ((0,0) is top left)
                
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                //length of word
                let len = word.len() as i32;
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[0].0 as i32;
                let prev_y = b.word_positions[0].1 + len - 1 as i32;
                
                b2.word_positions.insert(0, (prev_x - 1, prev_y , 1)); //up = less y ((0,0) is top left)
                
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                //length of word
                let len = word.len() as i32;
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[0].0 as i32;
                let prev_y = b.word_positions[0].1 as i32;
                
                b2.word_positions.insert(0, (prev_x - len + 1, prev_y + 1, 0)); //up = less y ((0,0) is top left)
                
                construct(&b2, &remaining, depth);
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
                    word_positions: b.word_positions.clone(),
                };
                //add word to beginning of words
                b2.words.insert(0, word.to_string());
                b2.remaining = remaining.clone();
                //length of word
                let len = word.len() as i32;
                //adding to top end
                //this is a vertical word so the third coordinate is 1
                let prev_x = b.word_positions[0].0 as i32;
                let prev_y = b.word_positions[0].1 as i32;
                
                b2.word_positions.insert(0, (prev_x - len + 1, prev_y - 1, 0)); //up = less y ((0,0) is top left)
                 
                construct(&b2, &remaining, depth);
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
            word_positions: Vec::new(),
        };
        construct(&b, &get_master_letters(), DEEP_DEPTH);
    }
    else {
        let mut recurse_boards = Vec::new();

        //double for loop structure prevents concurrent modification
        for board in BOARDS.lock().unwrap().iter_mut() {
            let board_clone = Board {
                tiles: board.tiles.clone(),
                remaining: board.remaining.clone(),
                bottom_end: board.bottom_end,
                top_end: board.top_end,
                words: board.words.clone(),
                bottom_end_vert: board.bottom_end_vert,
                top_end_vert: board.top_end_vert,
                word_positions: board.word_positions.clone(),
            };
            recurse_boards.push(board_clone);
        }

        for mut board in recurse_boards {
            let diff = remove_from_tiles(&board.tiles, &MASTER_LETTERS.lock().unwrap());
            board.remaining.push_str(&diff);
            board.tiles = MASTER_LETTERS.lock().unwrap().clone();
            construct(&board, &board.remaining, DEEP_DEPTH);
        }

        //Check refactor condition
        //create empty board
        let b = Board {
            tiles: get_master_letters(),
            remaining: get_master_letters(),
            bottom_end: ' ',
            top_end: ' ',
            words: Vec::new(),
            bottom_end_vert: false,
            top_end_vert: false,
            word_positions: Vec::new(),
        };
        construct(&b, &get_master_letters(), SHALLOW_DEPTH);
    }

    purge_old_boards();
    //for boards with identical words in in slightly different configurations
    remove_duplicates();

    sort_boards();

    println!("-------------------------------------------------------");

    pretty_print_boards();
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

//remove boards that have the same words
fn remove_duplicates() {
    //Since boards with the same words have the same remaining letters, we only need to check the words and can diregard the order
    
    //make copy of boards to avoid concurrent modification
    let mut boards_copy = Vec::new();
    for board in BOARDS.lock().unwrap().iter() {
        let clone_board = Board {
            tiles: board.tiles.clone(),
            remaining: board.remaining.clone(),
            bottom_end: board.bottom_end,
            top_end: board.top_end,
            words: board.words.clone(),
            bottom_end_vert: board.bottom_end_vert,
            top_end_vert: board.top_end_vert,
            word_positions: board.word_positions.clone(),
        };
        boards_copy.push(clone_board);
    }
    //mark which boards to remove
    let mut to_remove = Vec::new();
    
    let mut i = 0;
    while i < boards_copy.len() {
        let mut j = i + 1;
        while j < boards_copy.len() {
            if boards_copy[i].words == boards_copy[j].words {
                to_remove.push(j);
            }
            j += 1;
        }
        i += 1;
    }

    //sort to_remove from largest to smallest
    to_remove.sort_by(|a, b| b.cmp(a));

    //remove duplicates from to_remove
    if to_remove.len() == 0 {
        return;
    }
    let mut i = 0;
    while i < to_remove.len() - 1 {
        if to_remove[i] == to_remove[i + 1] {
            to_remove.remove(i);
        }
        else {
            i += 1;
        }
    }

    //remove marked boards
    for index in to_remove {
        BOARDS.lock().unwrap().remove(index);
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
    println!("Enter 'c' to place remainting tiles given the current boards.");
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
    println!("Enter 'c' to place remainting tiles given the current boards.");

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
            "c" => pretty_print_boards(),
            _ => add_letters(),
        }
    }
}

fn get_word_coordinates(word: &str, position_x: i32, position_y: i32, position_vert: i32 , x_offset: i32, y_offset: i32) -> Vec<(i32, i32)> {
    let mut coordinates = Vec::new();
    //determine if the word is vertical or horizontal
    if position_vert == 1 {
        //word is vertical
        for i in 0..word.len() {
            coordinates.push((position_x + x_offset, position_y + y_offset + i as i32));
        }
    }
    else {
        //word is horizontal
        for i in 0..word.len() {
            coordinates.push((position_x + x_offset + i as i32, position_y + y_offset));
        }
    }
    coordinates
}

fn pretty_print_boards() {
    //make sure boards are up to date with current letters
    //run through boards and add any letters that are not in the board
    for mut board in BOARDS.lock().unwrap().iter_mut() {
        let diff = remove_from_tiles(&board.tiles, &MASTER_LETTERS.lock().unwrap());
        board.remaining.push_str(&diff);
        board.tiles = MASTER_LETTERS.lock().unwrap().clone();
    }
    
    let boards = BOARDS.lock().unwrap();
    for i in 0..boards.len() {
        pretty_print_board(&boards[i]);
    }
}

fn pretty_print_board(b: &Board) {

    //can easily find min and max x and y values from start of words
    let min_x = b.word_positions.iter().map(|x| x.0).min().unwrap();
    let min_y = b.word_positions.iter().map(|x| x.1).min().unwrap();

    //max x and y values are found by adding the length of the word to the start of the word
    //get max x
    let mut x_coordinates = Vec::new();
    for i in 0..b.word_positions.len() {
        //if word is horizontal
        if b.word_positions[i].2 == 0 {
            x_coordinates.push(b.word_positions[i].0 + b.words[i].len() as i32);
        }
        else {
            x_coordinates.push(b.word_positions[i].0);
        }
    }

    let max_x = x_coordinates.iter().max().unwrap();

    //get max y
    let mut y_coordinates = Vec::new();
    for i in 0..b.word_positions.len() {
        //if word is vertical
        if b.word_positions[i].2 == 1 {
            y_coordinates.push(b.word_positions[i].1 + b.words[i].len() as i32);
        }
        else {
            y_coordinates.push(b.word_positions[i].1);
        }
    }

    let max_y = y_coordinates.iter().max().unwrap();
    
    let x_offset = -1 * min_x;
    let y_offset = -1 * min_y;

    let x_size = max_x - min_x + 1;
    let y_size = max_y - min_y + 1;

    let mut board = vec![vec![' '; x_size as usize]; y_size as usize];

    for i in 0..b.word_positions.len() {
        let word = &b.words[i];
        let positions = &b.word_positions[i];
        let coordinates = get_word_coordinates(word, positions.0, positions.1, positions.2 , x_offset, y_offset);
        for j in 0..word.len() {
            board[coordinates[j].1 as usize][coordinates[j].0 as usize] = word.chars().nth(j).unwrap();
        }
    }

    //TODO place remaining letters here


    for i in 0..y_size {
        for j in 0..x_size {
            print!("{}", board[i as usize][j as usize]);
        }
        println!("");
    }

    println!("");
    println!("Remaining letters: {}", b.remaining);

    println!("--------------------");
   
}

fn main() {
    init_game();
}
