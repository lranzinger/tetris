use crate::{
    game::{HEIGHT, WIDTH},
    state::Board,
    tetromino::Tetromino,
};
use macroquad::rand::gen_range;

pub struct DummyBoard {
    pub cells: Board,
}

impl DummyBoard {
    pub fn new() -> Self {
        let mut board = Self {
            cells: [[None; WIDTH as usize]; HEIGHT as usize],
        };
        board.generate_tetromino_pattern();
        board.add_floating_piece();
        board
    }

    fn generate_tetromino_pattern(&mut self) {
        self.cells = [[None; WIDTH as usize]; HEIGHT as usize];

        // Fill from bottom up
        for y in (0..HEIGHT).rev() {
            if y > HEIGHT - 4 {
                // Bottom rows almost full
                self.fill_row_with_gaps(y, 1);
            } else if y > HEIGHT - 8 {
                // Middle rows partially filled
                self.fill_row_with_gaps(y, 2);
            } else if y > HEIGHT - 12 {
                // Upper rows with tetromino shapes
                self.place_random_tetromino(y);
            }
        }
    }

    fn fill_row_with_gaps(&mut self, y: i32, gap_count: i32) {
        let mut gaps = Vec::new();
        for _ in 0..gap_count {
            gaps.push(gen_range(0, WIDTH));
        }

        for x in 0..WIDTH {
            if !gaps.contains(&x) {
                let color = Tetromino::random().color();
                self.cells[y as usize][x as usize] = Some(color);
            }
        }
    }

    fn place_random_tetromino(&mut self, base_y: i32) {
        let piece = Tetromino::random();

        let pos_x = gen_range(1, WIDTH - 3);

        for &(x, y) in &piece.shape() {
            let board_x = pos_x + x;
            let board_y = base_y + y;

            if (0..WIDTH).contains(&board_x) && (0..HEIGHT).contains(&board_y) {
                self.cells[board_y as usize][board_x as usize] = Some(piece.color());
            }
        }
    }

    fn add_floating_piece(&mut self) {
        let piece: Tetromino = Tetromino::random();

        // Place in upper third of board
        let x = gen_range(WIDTH / 2 - 2, WIDTH / 2 + 2);
        let y = gen_range(HEIGHT / 3 - 2, HEIGHT / 3 + 2); // Upper third of board

        // Add the piece using its shape
        for &(dx, dy) in &piece.shape() {
            let board_x = x + dx;
            let board_y = y + dy;
            if board_x < WIDTH && board_y < HEIGHT {
                self.cells[board_y as usize][board_x as usize] = Some(piece.color());
            }
        }
    }
}
