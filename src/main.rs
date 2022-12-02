use std::fs;
use termion::{color,cursor};
use clap::{ArgGroup,Parser};
#[derive(Debug)]

#[derive(Copy,Clone)]
struct PossField{
    fields: [Option<bool>;9],
    amount: i32
}


impl PossField {
    fn get(self, idx: usize) -> Option<bool>{
        self.fields[idx-1]
    }
    fn set(&mut self, idx: usize, val: bool){
        if idx >9{
            print!("")
        }
        self.fields[idx-1] = Some(val);
        PossField::calc_amount(self);
    }
    fn new() -> PossField{
        PossField { fields: [None;9],amount: 0}
    }
    fn combine(&mut self, other: PossField){
        for i in 0..9{
            match (self.fields[i] ,other.fields[i]){
                (Some(_),Some(_)) => self.fields[i] = Some(self.fields[i].unwrap() && other.fields[i].unwrap()),
                (None,Some(_))=> self.fields[i] = other.fields[i],
                _ => (),
            }
        }
        PossField::calc_amount(self);
    }
    fn calc_amount(&mut self){
        self.amount = 0;
        for i in 0..9{
            match self.fields[i]{
                None => (),
                _ => self.amount += 1,
            }
        }
    }
}

#[derive(Copy,Clone)]
struct Board {
    fields: [Option<u32>;81]
}
impl Board {
    fn new() -> Board{
        Board{
            fields:[None;81],
        }
    }
}

#[derive(Copy,Clone)]
struct BoardPoss {
    fields: [PossField ;81]
}
    
impl BoardPoss {
    fn new() -> BoardPoss{
        BoardPoss{
            fields:[PossField::new() ;81]
        }
    }
}

#[derive(Debug)]
enum BoardReadError{
    WrongLength,
    UnexpectedCharacter
}


fn make_board(input: &str)-> Result<Board,BoardReadError>{
    let mut board = Board::new();
    
    if input.len() != 81{
        return Err(BoardReadError::WrongLength);
    }
    for (i,c) in input.chars().enumerate() {
        match c {
            '1'..='9' => board.fields[i] = Some(c.to_digit(10).unwrap()),
            '.' => board.fields[i] = None,
            _ => return Err(BoardReadError::UnexpectedCharacter),
        }
    }
    return Ok(board);
}

fn check_rows(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..9 {
        let mut not_possible = PossField::new();
        for j in 0..9 {
            let num = board.fields[i*9 + j];
            match num {
                Some(a)=> not_possible.set(a as usize,false),
                None => (),
            }
        }
        for j in 0..9 {
            new_poss.fields[i*9 + j].combine(not_possible);
        }
    }
    return new_poss;
}

fn check_columns(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..9 {
        let mut not_possible = PossField::new();
        for j in 0..9 {
            let num = board.fields[i + j*9];
            match num {
                Some(a)=> not_possible.set(a as usize,false),
                None => (),
            }
        }
        for j in 0..9 {
            new_poss.fields[i + j*9].combine(not_possible);
        }
    }
    return new_poss;
}

fn check_squares(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..3{
        for j in 0..3{
            let mut square = PossField::new();
            for l in 0..3{
                for k in 0..3{
                    let num = board.fields[3*j +k + 9*(3*i+l)];
                    match num {
                        Some(a)=> square.set(a as usize,false),
                        None => (),
                    }       
                }
            }
            for l in 0..3{
                for k in 0..3{
                    new_poss.fields[3*j +k + 9*(3*i+l)].combine(square);
                }
            }
        }
    }
    return new_poss;
}

fn print_board(board:&Board){
    for i in 0..9{
        if i%3==0{println!{""};}
        for j in 0..9{
            if j%3==0{print!{" "};}
            let val = board.fields[i*9 +j];
            match val{
                Some(_) => print!{"{}",val.unwrap()},
                None => print!{"~"},
            }
        }
        println!{""};
    }
}


fn print_checked(poss: &BoardPoss){
    for i in 0..9{
        if i%3==0{println!{""};} //Delim Rows Squares
        for j in 0..3{
            for k in 0..9{
                if k%3==0{print!{"  "};}//Delim Cols Squares
                for l in 0..3{
                    let val = poss.fields[9*i+k].get(l+3*j+1);
                    match val {
                       Some(false)=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Red)},
                       Some(true)=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Green)},
                       None=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Reset)},
                    }
                }
                print!{"{} ",color::Bg(color::Reset)} //Delim Cols Neigbour
            }
            println!{""} //Delim Rows Neigbour
        }
        //println!{""}
    }
}

fn replace_known(board: &mut Board, poss: &BoardPoss) -> Option<bool>{
    let mut replaced = false;
    for (i,f) in poss.fields.iter().enumerate(){
        match board.fields[i]{
            Some(_) => continue,
            None => (),
        }
        let mut amount = 0;
        for j in 1..10 {
            match f.get(j) {
                Some(false) => amount+=1,
                _ => (),
                }
        }
        if amount==8 {
            for j in 1..10 {
                match f.get(j) {
                    None => {
                        board.fields[i] = Some(j as u32);
                        replaced = true;
                        break
                    },
                    _ => (),
                    }
            }
        }
        else if amount==9 {
            return None
        }
    }
    return Some(replaced)
}

fn has_none(board: &Board) -> bool{

    for f in board.fields.iter(){
        match f {
            None => return true,
            _ => (),
        }
    }
    return false
}

fn calc_dependence(board: &Board) -> [i32;81]{
    let mut dependence = [0;81];

    let mut row_amount = [0;9];
    let mut column_amount = [0;9];
    // Rows and Columns
    for i in 0..9{
        for j in 0..9{
            match board.fields[9*i+j] {
                Some(_) => row_amount[i]+=1,
                _ => (),
            }
            match board.fields[i+j*9] {
                Some(_) => column_amount[j]+=1,
                _ => (),
            }
        }
    }
    // Squares, don't count double, 
    //TODO
    for i in 0..9{
        for j in 0..9{
            dependence[9*i+j] = row_amount[i]+column_amount[j];
        }
    }

    return dependence
}

fn calc_guess(board: &Board, poss: &BoardPoss) -> usize{
    let dependence = calc_dependence(&board);
    let mut max_i = 0;
    let mut max = -1;
    let mut max_depend = -1;
    for (i,f) in board.fields.iter().enumerate(){
        match f {
            Some(_) => continue,
            _ => (),
        }
        
        let amount = poss.fields[i].amount;
        let depend = dependence[i];
        // most impossible vals and most dependence
        if (amount > max) || (amount == max && depend>max_depend) {
            max = amount;
            max_i = i;
            max_depend = depend;
        }
    }
    if max ==-1{
        print!("");
    }
    return max_i
}


fn try_solve(input_board:&Board,input_poss: &BoardPoss) -> Option<Board>{
    //print_board(&input_board);
    let mut board = *input_board;
    let mut poss = *input_poss;
    let mut exchanged;
    while has_none(&board){
        //print_board(&board);
        poss = check_rows(&board,&poss);
        //print_checked(&poss);
        poss = check_columns(&board,&poss);
        //print_checked(&poss);
        poss = check_squares(&board,&poss);
        //print_checked(&poss);
        // for i in 0..81 {
        //     match board.fields[i]{
        //         Some(_) => continue,
        //         None => (),
        //     }
        //     let poss_field = poss.fields[i];
        //     let mut amount = 0;
        //     for j in 0..9{
        //         match poss_field.fields[j]{
        //             Some(_) => amount +=1,
        //             _ => (),
        //         }
        //     }
        //     if amount==9{
        //         print_board(&board);
        //         print_checked(&poss);
        //         print!("");
        //     }
        // }
        match replace_known(&mut board, &poss){
            Some(a) => exchanged = a,
            None => return None,
        }
        if !exchanged {
            let guess_index = calc_guess(&board,&poss);
            let mut guess: u32 = 0;
            // if guess_index ==44{
            //     print_board(&board);
            //     print_checked(&poss);
            //     print!("");
            // }
            for i in 0..9{
                let field_poss = poss.fields[guess_index].fields[i];
                match field_poss{
                    None => {
                        guess= i as u32 +1;
                        break;
                    },
                    _ => (),
                }
            }
            //println!("{}",guess_index);
            //println!{"Guessing {} is {}",guess_index,guess};
            //print_checked(&poss);
            let mut new_board = board;
            new_board.fields[guess_index] = Some(guess);
            match try_solve(&new_board, &poss.clone()){
                Some(_board) => return Some(_board),
                None => poss.fields[guess_index].set(guess as usize,false),
            }
            //println!{"Guessed Wrong! {} is NOT {}",guess_index,guess};
        }
    }
    
    return Some(board)
}

#[derive(Parser)]
struct Cli{
    input_file: std::path::PathBuf,

    output_file: std::path::PathBuf
}


fn main() {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.input_file)
        .expect("Should have been able to read the file");
    let mut lines = contents.split('\n').collect::<Vec<&str>>();
    //lines.pop();
    //println!{"{}",lines[1]};
    //let test = csv::Reader.from_path(file_path);
    let mut good = 0;
    let mut len = lines.len();
    let len_digits = 4 as usize;
    for (i,line) in lines.iter().enumerate(){
        let board = match make_board(line){
            Ok(b) => b,
            Err(e) => {
                eprintln!{"{:?}",e};
                len -= 1;
                continue
            },
        };
        let solved_board = try_solve(&board, &BoardPoss::new());
        match solved_board{
            Some(_)=> {println!("{:>len_digits$}/{:>len_digits$} solved{}",i,len,cursor::Up(1)); good+=1;},
            _ => println!{"Something went wrong."}
        }
    }
    println!("passed {}/{}",good,len);

    // let board = make_board(lines[1527]).ok().expect("couldn't make board");
    // let solved_board = try_solve(&board, &BoardPoss::new());
    // match solved_board{
    //     Some(_)=> print_board(&solved_board.unwrap()),
    //     _ => println!{"Something went wrong."}
    // }
}

#[cfg(test)]
#[test]
fn poss_field() {
    let mut a = PossField::new();
    a.set(1, false);
    assert_eq!(a.get(1),Some(false));
}

