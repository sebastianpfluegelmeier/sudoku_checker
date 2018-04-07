extern crate rpds;

use rpds::HashTrieMap;
use rpds::HashTrieSet;

type Sudoku = HashTrieMap<(u8, u8), Option<u8>>;

fn main() {
    let sudoku = HashTrieMap::new();
    println!("{}", is_valid(&sudoku));
}

fn is_valid(sudoku: &HashTrieMap<(u8, u8), Option<u8>>) -> bool {
    let rows_valid = (1..10).map(|x| check_row(&sudoku, x))
                            .all(|x| x);
    let cols_valid = (1..10).map(|x| check_col(&sudoku, x))
                            .all(|x| x);
    let blocks_valid = 
        [(0, 0), (0, 3), (0, 6), 
         (3, 0), (3, 3), (3, 6), 
         (6, 0), (6, 3), (6, 6)].iter()
                                .map(|(x, y)| check_block(&sudoku, *x as u8, *y as u8))
                                .all(|x| x);
    
    return rows_valid && cols_valid && blocks_valid;
}

fn check_row(sudoku: &Sudoku, row: u8) -> bool {

    // all entries in the current row
    let entries: HashTrieSet<u8> = 
                    // take the entries in the current row
            (1..10).map(|x| *sudoku.get(&(x, row)).unwrap())
                    // remove the Nothings
                   .filter(|x| x.is_some())
                    // unwrap them so Some(x) becomes x
                   .map(|x| x.unwrap())
                    // collect them so the Iterator becomes a HashTrieSet
                   .collect();

    let has_duplicate = entries.iter().map(|x| entries.remove(x).contains(x)).all(|x| !x);

    return has_duplicate;
}

fn check_col(sudoku: &Sudoku, col: u8) -> bool {
    // all entries in the current column
    let entries: HashTrieSet<u8> = 
                    // take the entries in the current column
            (1..10).map(|x| *sudoku.get(&(col, x)).unwrap())
                    // remove the Nothings
                   .filter(|x| x.is_some())
                    // unwrap them so Some(x) becomes x
                   .map(|x| x.unwrap())
                    // collect them so the Iterator becomes a HashTrieSet
                   .collect();

    let has_duplicate = entries.iter().map(|x| entries.remove(x).contains(x)).all(|x| !x);

    return has_duplicate;
}

fn check_block(sudoku: &Sudoku, row: u8, col: u8) -> bool {
    // all entries in the current block
    let entries: HashTrieSet<u8> = 
                    // take the entries in the current row
            [(0, 0), (1, 0), (2, 0),
             (0, 1), (1, 1), (2, 1),
             (0, 2), (1, 2), (2, 2)]
                    .iter()
                    .map(|(x, y)| *sudoku.get(&(col + x, row + y)).unwrap())
                    // remove the Nothings
                   .filter(|x| x.is_some())
                    // unwrap them so Some(x) becomes x
                   .map(|x| x.unwrap())
                    // collect them so the Iterator becomes a HashTrieSet
                   .collect();

    let has_duplicate = entries.iter().map(|x| entries.remove(x).contains(x)).all(|x| !x);

    return has_duplicate;
}
