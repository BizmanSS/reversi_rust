//Assignment 1 ECE1724H
//Bisman Sawhney
//Student number: 1005730755
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
fn main() {
    let mut board = [
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', 'W', 'B', '.', '.', '.'],
        ['.', '.', '.', 'B', 'W', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    let mut is_black_turn = true;

    loop {
        print_board(&board);
        if !valid_turn(&board, is_black_turn) {
            if !valid_turn(&board, !is_black_turn) {
                game_over_display(&board); //Game Over
                break;
            }
            is_black_turn = !is_black_turn;
            continue;
        }
        let player_color = if is_black_turn { "B" } else { "W" };
        print!("Enter move for colour {} (RowCol): ", player_color);
        stdout().flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        let (row_idx, col_idx) = match parse_input(input, &board, is_black_turn) {
            Ok((row_idx, col_idx)) => (row_idx, col_idx),
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
        make_move(&mut board, row_idx, col_idx, is_black_turn);
        is_black_turn = !is_black_turn;
    }
}
//Function to print the current state of the board
fn print_board(board: &[[char; 8]; 8]) {
    println!("  abcdefgh");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", (b'a' + i as u8) as char);
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
//Function to parse the move input given by user
fn parse_input(
    input: &str,
    board: &[[char; 8]; 8],
    is_black_turn: bool,
) -> Result<(usize, usize), &'static str> {
    if input.len() == 2 {
        let mut chars = input.chars();
        let row_char = chars.next().unwrap();
        let col_char = chars.next().unwrap();
        if ('a'..='h').contains(&row_char) && ('a'..='h').contains(&col_char) {
            let row_idx = row_char as usize - 'a' as usize;
            let col_idx = col_char as usize - 'a' as usize;

            if is_valid_move(board, row_idx, col_idx, is_black_turn) {
                return Ok((row_idx, col_idx));
            }
        }
    }
    Err("Invalid move. Try again.")
}
//Function to check if a given move is valid
fn is_valid_move(board: &[[char; 8]; 8], r: usize, c: usize, is_black_turn: bool) -> bool {
    if board[r][c] != '.' {
        return false;
    }
    let player = if is_black_turn { 'B' } else { 'W' };
    let opponent = if is_black_turn { 'W' } else { 'B' };
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for dir in directions.iter() {
        let mut temp_r = r as isize + dir.0; //can be negative
        let mut temp_c = c as isize + dir.1;
        let mut found_opponent = false;
        while in_bounds(temp_r, temp_c)
            && temp_c < 8
            && board[temp_r as usize][temp_c as usize] == opponent
        {
            found_opponent = true;
            temp_r += dir.0;
            temp_c += dir.1;
        }
        if found_opponent
            && in_bounds(temp_r, temp_c)
            && board[temp_r as usize][temp_c as usize] == player
        {
            return true;
        }
    }
    false
}
//Function to make a move and update the board
fn make_move(board: &mut [[char; 8]; 8], r: usize, c: usize, is_black_turn: bool) {
    let player = if is_black_turn { 'B' } else { 'W' };
    let opponent = if is_black_turn { 'W' } else { 'B' };

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    board[r][c] = player;

    for direction in directions.iter() {
        let mut flip_pieces = Vec::new();
        let mut temp_r = r as isize + direction.0;
        let mut temp_c = c as isize + direction.1;

        while in_bounds(temp_r, temp_c)
            && temp_c < 8
            && board[temp_r as usize][temp_c as usize] == opponent
        {
            flip_pieces.push((temp_r as usize, temp_c as usize));
            temp_r += direction.0;
            temp_c += direction.1;
        }

        if in_bounds(temp_r, temp_c) && board[temp_r as usize][temp_c as usize] == player {
            for (x, y) in flip_pieces {
                board[x][y] = player;
            }
        }
    }
}
//Function to check if the row and col are in-bounds
fn in_bounds(r: isize, c: isize) -> bool {
    (0..8).contains(&c) && (0..8).contains(&r)
}
//Function to check if a given turn is valid
fn valid_turn(board: &[[char; 8]; 8], is_black_turn: bool) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if is_valid_move(board, i, j, is_black_turn) {
                return true;
            }
        }
    }

    println!(
        "{} player has no valid move.",
        if is_black_turn { "B" } else { "W" }
    );
    false
}
//Function to assess who won the game and to display the final statement
fn game_over_display(board: &[[char; 8]; 8]) {
    let mut w_count = 0;
    let mut b_count = 0;
    for row in board {
        for cell in row {
            match cell {
                'W' => w_count += 1,
                'B' => b_count += 1,
                _ => {}
            }
        }
    }

    match w_count.cmp(&b_count) {
        Ordering::Greater => {
            let diff = w_count - b_count;
            println!("White wins by {} points!", diff);
        }
        Ordering::Less => {
            let diff = b_count - w_count;
            println!("Black wins by {} points!", diff);
        }
        Ordering::Equal => {
            println!("It's a draw!");
        }
    }
}
