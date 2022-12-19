use crate::board::*;

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


pub fn try_solve(input_board:&Board,input_poss: &BoardPoss) -> Option<Board>{
    //print_board(&input_board);
    let mut board = *input_board;
    let mut poss = *input_poss;
    let mut exchanged;
    while has_none(&board){
        poss = check_rows(&board,&poss);
        poss = check_columns(&board,&poss);
        poss = check_squares(&board,&poss);
        match replace_known(&mut board, &poss){
            Some(a) => exchanged = a,
            None => return None,
        }
        if !exchanged {
            let guess_index = calc_guess(&board,&poss);
            let mut guess: u32 = 0;
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
            let mut new_board = board;
            new_board.fields[guess_index] = Some(guess);
            match try_solve(&new_board, &poss.clone()){
                Some(_board) => return Some(_board),
                None => poss.fields[guess_index].set(guess as usize,false),
            }
        }
    }
    
    return Some(board)
}

#[cfg(test)]
#[test]
fn poss_field() {
    let mut a = PossField::new();
    a.set(1, false);
    assert_eq!(a.get(1),Some(false));
}