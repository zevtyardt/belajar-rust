struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku { board }
    }

    fn show(&self) {
        for i in 0..9 {
            println!("{:?}", self.board[i])
        }
        println!()
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] == 0 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn check(&self, x: usize, y: usize, v: u8) -> bool {
        for i in 0..9 {
            if self.board[x][i] == v || self.board[i][y] == v {
                return false;
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                if self.board[(x - x % 3) + i][(y - y % 3) + j] == v {
                    return false;
                }
            }
        }
        true
    }

    fn solve(&mut self) -> bool {
        if let Some((x, y)) = self.find_empty_cell() {
            for v in 1..10 {
                if self.check(x, y, v) {
                    self.board[x][y] = v;
                    if self.solve() {
                        return true;
                    }
                    self.board[x][y] = 0;
                }
            }
        } else {
            return true;
        }
        false
    }
}

fn main() {
    let board = [
        [3, 0, 6, 5, 0, 8, 4, 0, 0],
        [5, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [0, 0, 3, 0, 1, 0, 0, 8, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [1, 3, 0, 0, 0, 0, 2, 5, 0],
        [0, 0, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    let board_with_no_solution = [
        [5, 0, 6, 5, 0, 8, 4, 0, 3],
        [5, 2, 0, 0, 0, 0, 0, 0, 2],
        [1, 8, 7, 0, 0, 0, 0, 3, 1],
        [0, 0, 3, 0, 1, 0, 0, 8, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [1, 3, 0, 0, 0, 0, 2, 5, 0],
        [0, 0, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    for (i, b) in [board, board_with_no_solution].iter().enumerate() {
        let mut sudoku = Sudoku::new(*b);

        println!("papan sudoku ke-{}:\n", i + 1);
        sudoku.show();
        if !sudoku.solve() {
            println!("tidak dapat menyelesaikan sudoku..")
        } else {
            println!("sudoku berhasil diselesaikan..\n");
            sudoku.show();
        }
    }
}
