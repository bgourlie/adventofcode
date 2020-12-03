use util::{read_lines, Result};

struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(board: Vec<Vec<char>>) -> Self {
        let (width, height) = (board.first().unwrap().len(), board.len());
        Board {
            board,
            width,
            height,
        }
    }

    fn square_at(&self, x: usize, y: usize) -> char {
        let x = x % self.width;
        self.board[y][x]
    }

    fn check_path(&self, x_step: usize, y_step: usize) -> usize {
        (y_step..self.height)
            .step_by(y_step)
            .into_iter()
            .map(|height| ((height / y_step) * x_step, height))
            .filter(|(x, y)| self.square_at(*x, *y) == '#')
            .count()
    }
}

fn main() -> Result<()> {
    let board = read_lines("_03/input.txt")?
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let board = Board::new(board);

    let problem_2_solution = &[
        board.check_path(1, 1),
        board.check_path(3, 1),
        board.check_path(5, 1),
        board.check_path(7, 1),
        board.check_path(1, 2),
    ]
    .iter()
    .product::<usize>();

    println!("Problem 1 solution: {}", board.check_path(3, 1));
    println!("Problem 2 solution: {}", problem_2_solution);
    Ok(())
}
