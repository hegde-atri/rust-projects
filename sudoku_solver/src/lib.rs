// TODO write documentation
// Also implement from_line to grid

struct Position {
    x: u8,
    y: u8,
}

pub struct Board {
    board: [[u8; 9]; 9],
}

impl Board {
    pub fn from(board: [[u8; 9]; 9]) -> Result<Board, String> {
        // TODO
        return Ok(Board {
            board: [[0u8; 9]; 9],
        });
    }

    pub fn set_cell() {}

    pub fn with_cell() {}
}

pub struct Solver {
    solution: [[u8; 9]; 9],
    steps_taken: Option<u32>,
    time_taken: Option<u32>,
}

impl Solver {
    // first check if inputted board is solvable, it has no existing conflicts
    pub fn solve(board: &[[u8; 9]; 9]) -> Solver {
        // TODO
        return Solver {
            solution: [[0u8; 9]; 9],
            steps_taken: None,
            time_taken: None,
        };
    }

    pub fn verbose(board: [[u8; 9]; 9]) {
        // TODO: display each step taken?
    }

    pub fn benchmark(board: [[u8; 9]; 9]) -> Solver {
        // TODO: display time taken, and total steps taken?
        return Solver {
            solution: [[0u8; 9]; 9],
            steps_taken: None,
            time_taken: None,
        };
    }

    fn is_valid(board: [[u8; 9]; 9], num: u8, pos: Position) -> Result<bool, String> {
        // ignore 0's
        // TODO: Check x-axis
        // TODO: Check y-axis
        // TODO: Check box
        // Take modulus, then for in range from modulus * 3 to modulus * 3 + 3
        Err(String::from("Not implemented"))
    }
}

pub fn print_board(board: [[u8; 9]; 9]) {
    // TODO
}
