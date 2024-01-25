use crate::board::*;
#[cfg(not(windows))]
use termion::color;
#[cfg(windows)]
use termion_win::color;

#[derive(Debug)]
pub enum BoardReadError {
    WrongLength,
    UnexpectedCharacter,
}

pub fn make_board(input: &str, del: &char, end: &str) -> Result<Board, BoardReadError> {
    let mut board = Board::new();

    if input.len() != 81 + end.len() {
        return Err(BoardReadError::WrongLength);
    }
    for (i, c) in input[..81].chars().enumerate() {
        match c {
            '1'..='9' => board.fields[i] = Some(c.to_digit(10).unwrap()),
            c if c == *del => board.fields[i] = None,
            _ => return Err(BoardReadError::UnexpectedCharacter),
        }
    }
    Ok(board)
}

pub fn get_formatted_board(board: &Board, del: &char) -> String {
    let mut out = String::new();
    for i in 0..9 {
        // cols
        if i == 3 || i == 6 {
            out.push('\n')
        }
        for j in 0..9 {
            //rows
            if j == 3 || j == 6 {
                out.push(' ')
            }
            let val = board.fields[i * 9 + j];
            match val {
                Some(v) => out.push_str(format! {" {} ",v}.as_str()),
                None => out.push_str(&format!(" {} ", del)),
            }
        }
        out.push('\n');
    }
    out
}

#[allow(dead_code)]
pub fn print_checked(poss: &BoardPoss) {
    for i in 0..9 {
        if i % 3 == 0 {
            println!();
        } //Delim Rows Squares
        for j in 0..3 {
            for k in 0..9 {
                if k % 3 == 0 {
                    print! {"  "};
                } //Delim Cols Squares
                for l in 0..3 {
                    let val = poss.fields[9 * i + k].get(l + 3 * j + 1);
                    match val {
                        Some(false) => print! {" {1}{0}",l+3*j +1,color::Bg(color::Red)},
                        Some(true) => print! {" {1}{0}",l+3*j +1,color::Bg(color::Green)},
                        None => print! {" {1}{0}",l+3*j +1,color::Bg(color::Reset)},
                    }
                }
                print! {"{} ",color::Bg(color::Reset)} //Delim Cols Neigbour
            }
            println!() //Delim Rows Neigbour
        }
        //println!{""}
    }
}

#[allow(dead_code)]
pub fn print_formatted_board(board: &Board) {
    println!("{}", get_formatted_board(board, &'.'))
}

pub fn get_encoded_board(board: &Board, del: &char, end: &str) -> String {
    let mut out = String::new();
    for field in board.fields {
        match field {
            Some(n) => out.push_str(format!("{}", n).as_str()),
            None => out.push_str(&del.to_string()),
        }
    }
    out.push_str(&format!("{}\n", end));
    out
}
