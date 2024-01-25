use crate::{board::*, print_checked, print_formatted_board};

fn check_rows(board: &Board, poss: &BoardPoss) -> Option<BoardPoss> {
    let mut new_poss = *poss;
    for i in 0..9 {
        // invalid board detector array
        let mut num_count: [usize; 9] = [0; 9];
        // rows;
        let mut not_possible = PossField::new();
        // write existing numbers in row in PossField
        for j in 0..9 {
            // collumns
            let num = board.fields[i * 9 + j];
            if let Some(a) = num {
                not_possible.set(a as usize, false);
                num_count[(a - 1) as usize] += 1;
            }
        }
        // combine the impossible fields
        for j in 0..9 {
            new_poss.fields[i * 9 + j].combine(not_possible);
        }
        if num_count.iter().max()? > &1 {
            return None;
        }
    }
    Some(new_poss)
}

fn check_columns(board: &Board, poss: &BoardPoss) -> Option<BoardPoss> {
    let mut new_poss = *poss;
    for i in 0..9 {
        // invalid board detector array
        let mut num_count: [usize; 9] = [0; 9];
        // collumns
        let mut not_possible = PossField::new();
        // write existing numbers in collumn in PossField
        for j in 0..9 {
            // rows
            let num = board.fields[i + j * 9];
            if let Some(a) = num {
                not_possible.set(a as usize, false);
                num_count[(a - 1) as usize] += 1;
            }
        }
        // combine the impossible fields
        for j in 0..9 {
            new_poss.fields[i + j * 9].combine(not_possible);
        }
        if num_count.iter().max()? > &1 {
            return None;
        }
    }
    Some(new_poss)
}

fn check_squares(board: &Board, poss: &BoardPoss) -> Option<BoardPoss> {
    let mut new_poss = *poss;
    for i in 0..3 {
        // squares row
        for j in 0..3 {
            // invalid board detector array
            let mut num_count: [usize; 9] = [0; 9];
            // squares collumn
            let mut square = PossField::new();
            // write existing numbers in square in PossField
            for l in 0..3 {
                // current square row
                for k in 0..3 {
                    // current square collumn
                    let num = board.fields[3 * j + k + 9 * (3 * i + l)];
                    if let Some(a) = num {
                        square.set(a as usize, false);
                        num_count[(a - 1) as usize] += 1;
                    }
                }
            }
            // combine the impossible fields
            for l in 0..3 {
                for k in 0..3 {
                    new_poss.fields[3 * j + k + 9 * (3 * i + l)].combine(square);
                }
            }
            if num_count.iter().max()? > &1 {
                return None;
            }
        }
    }
    Some(new_poss)
}

fn replace_known(board: &mut Board, poss: &BoardPoss) -> Option<bool> {
    // BUG: can replace two fields with the same number in the same pass, even if an invalid board is created
    // probably because the amount == 9 check is skipped when a number is written to the field

    let mut replaced = false;
    for (i, f) in poss.fields.iter().enumerate() {
        if board.fields[i].is_some() {
            continue;
        }
        let mut amount = 0;
        // count amount of impossible fields
        for j in 1..10 {
            if f.get(j) == Some(false) {
                amount += 1
            }
        }
        if amount == 8 {
            // one remaining? --> set remaining in board
            for j in 1..10 {
                if f.get(j).is_none() {
                    board.fields[i] = Some(j as u32);
                    replaced = true;
                    break;
                }
            }
        } else if amount == 9 {
            // solved wrong or impossible
            return None;
        }
    }
    Some(replaced)
}

fn has_none(board: &Board) -> bool {
    // check if board has unsolved fields
    for f in board.fields.iter() {
        if f.is_none() {
            return true;
        }
    }
    false
}

fn calc_dependence(board: &Board) -> [i32; 81] {
    let mut dependence = [0; 81]; // measure how many unsolved fields are causally connected

    let mut row_amount = [0; 9];
    let mut column_amount = [0; 9];
    // rows and columns
    for i in 0..9 {
        for j in 0..9 {
            if board.fields[9 * i + j].is_some() {
                row_amount[i] += 1
            }
            if board.fields[i + j * 9].is_some() {
                column_amount[j] += 1
            }
        }
    }
    // TODO: squares, don't count double

    // calculate final dependence
    for i in 0..9 {
        for j in 0..9 {
            dependence[9 * i + j] = row_amount[i] + column_amount[j];
        }
    }

    dependence
}

fn calc_guess(board: &Board, poss: &BoardPoss) -> usize {
    let dependence = calc_dependence(board);
    let mut max_i = 0;
    let mut max = -1;
    let mut max_depend = -1;
    for (i, f) in board.fields.iter().enumerate() {
        if f.is_some() {
            continue; // if already set, don't guess it
        }

        let amount = poss.fields[i].amount;
        let depend = dependence[i];
        // most impossible values and most dependence
        if (amount > max) || (amount == max && depend > max_depend) {
            max = amount;
            max_i = i;
            max_depend = depend;
        }
    }
    if max == -1 {
        print!("");
    }
    max_i
}

pub fn try_solve(input_board: &Board, input_poss: &BoardPoss) -> Option<Board> {
    let mut board = *input_board;
    let mut poss = *input_poss;
    let mut exchanged;
    print_formatted_board(&board);
    while has_none(&board) {
        poss = check_rows(&board, &poss)?;
        poss = check_columns(&board, &poss)?;
        poss = check_squares(&board, &poss)?;
        print_checked(&poss);
        exchanged = replace_known(&mut board, &poss)?;
        print_formatted_board(&board);
        if !exchanged {
            // no trivial step left
            let guess_index = calc_guess(&board, &poss);
            let mut guess: u32 = 0;
            // guess first possible value
            for i in 0..9 {
                let field_poss = poss.fields[guess_index].fields[i];
                if field_poss.is_none() {
                    guess = i as u32 + 1;
                    break;
                }
            }
            // construct new board with guessed value
            let mut new_board = board;
            new_board.fields[guess_index] = Some(guess);
            // solve new board
            match try_solve(&new_board, &poss.clone()) {
                Some(board) => return Some(board), // solved
                None => poss.fields[guess_index].set(guess as usize, false), // guessed wrong, remove possibility
            }
        }
    }

    Some(board)
}

#[cfg(test)]
#[test]
fn poss_field() {
    let mut a = PossField::new();
    a.set(1, false);
    assert_eq!(a.get(1), Some(false));
}
