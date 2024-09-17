use rand::{rngs::ThreadRng, Rng};
use speedy::{Readable, Writable};

pub const SIZE: usize = 5;

#[derive(Debug, Readable, Writable, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    id: usize,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
}

pub type PuzzleRow = [Piece; SIZE];
pub type Puzzle = [PuzzleRow; SIZE];
pub type Pieces = Vec<Piece>;

#[derive(Debug, Writable, Readable, Clone)]
pub struct Solution {
    pub score: f32,
    pub tries: u128,
    pub treshold: f32,
    pub edge_count: u8,
    pub values: Vec<Puzzle>,
}

#[derive(Debug, Readable, Writable, PartialEq)]
pub struct PuzzleGen {
    pub pieces: Puzzle,
    pub solutions: Vec<Puzzle>,
}

/// Get a random number between 1 and max+1
fn get_rand_int_with_max(max: u8, rng: &mut ThreadRng) -> u8 {
    rng.gen_range(1..=max)
}

/// Validate a puzzle
fn validate_puzzle(puzzle: &Puzzle) -> bool {
    for row_idx in 0..SIZE {
        for col_idx in 0..SIZE {
            let piece = &puzzle[row_idx][col_idx];

            // Check edges
            if (row_idx == 0 && piece.top != 0)
                || (row_idx == SIZE - 1 && piece.bottom != 0)
                || (col_idx == 0 && piece.left != 0)
                || (col_idx == SIZE - 1 && piece.right != 0)
            {
                return false;
            }

            // Internal validation
            if row_idx > 0 && puzzle[row_idx - 1][col_idx].bottom != piece.top {
                return false;
            }
            if col_idx > 0 && puzzle[row_idx][col_idx - 1].right != piece.left {
                return false;
            }
        }
    }

    true
}

/// Generate a puzzle with a maximum edge type
fn generate_puzzle_with_max(max: u8) -> Puzzle {
    let mut rng = rand::thread_rng();
    let mut puzzle: Puzzle = [[Piece {
        id: 0,
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    }; SIZE]; SIZE];

    for row_idx in 0..SIZE {
        for col_idx in 0..SIZE {
            let top = if row_idx == 0 {
                0
            } else {
                puzzle[row_idx - 1][col_idx].bottom
            };
            let left = if col_idx == 0 {
                0
            } else {
                puzzle[row_idx][col_idx - 1].right
            };
            let bottom = if row_idx == SIZE - 1 {
                0
            } else {
                get_rand_int_with_max(max, &mut rng)
            };
            let right = if col_idx == SIZE - 1 {
                0
            } else {
                get_rand_int_with_max(max, &mut rng)
            };
            let id = row_idx * SIZE + col_idx;

            puzzle[row_idx][col_idx] = Piece {
                id,
                top,
                right,
                bottom,
                left,
            };
        }
    }

    puzzle
}

fn flatten_puzzle(puzzle: &Puzzle) -> Vec<Piece> {
    puzzle.iter().flat_map(|row| row.iter()).cloned().collect()
}

fn grow_puzzle(puzzle: Vec<Piece>) -> Puzzle {
    let mut new_puzzle: Puzzle = [[Piece {
        id: 0,
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    }; SIZE]; SIZE];
    for (idx, piece) in puzzle.into_iter().enumerate() {
        let row_idx = idx / SIZE;
        let col_idx = idx % SIZE;
        new_puzzle[row_idx][col_idx] = piece;
    }
    new_puzzle
}

pub fn get_similarity_score(puzzle: &Puzzle, other: &Puzzle) -> f32 {
    let matching_pieces = puzzle
        .iter()
        .zip(other.iter())
        .flat_map(|(row1, row2)| row1.iter().zip(row2.iter()))
        .filter(|&(p1, p2)| p1 == p2)
        .count();

    matching_pieces as f32 / (SIZE * SIZE) as f32
}

impl PuzzleGen {
    pub fn new(edge_types: u8) -> Self {
        Self {
            pieces: generate_puzzle_with_max(edge_types),
            solutions: Vec::new(),
        }
    }

    pub fn solve(&mut self) {
        self.recursive_solve(Vec::new(), flatten_puzzle(&self.pieces));
    }

    pub fn new_puzzle(&mut self) {
        self.pieces = generate_puzzle_with_max(SIZE as u8);
        self.solutions.clear();
    }

    pub fn is_valid(&self) -> bool {
        validate_puzzle(&self.pieces)
    }

    pub fn get_puzzle(&self) -> Puzzle {
        self.pieces
    }

    pub fn get_solutions(&self) -> Vec<Puzzle> {
        self.solutions.clone()
    }

    fn recursive_solve(&mut self, current: Pieces, remaining: Pieces) {
        if remaining.is_empty() {
            let potential_solution = grow_puzzle(current);
            if validate_puzzle(&potential_solution) {
                self.solutions.push(potential_solution);
            }
            return;
        }

        for (i, &check_piece) in remaining.iter().enumerate() {
            let mut new_current = current.clone();
            let mut new_remaining = remaining.clone();
            new_remaining.remove(i);

            let idx = new_current.len();
            let row_idx = idx / SIZE;
            let col_idx = idx % SIZE;

            // Check top and left constraints before placing the piece
            let fits_top = if row_idx == 0 {
                check_piece.top == 0
            } else {
                check_piece.top == new_current[idx - SIZE].bottom
            };

            let fits_left = if col_idx == 0 {
                check_piece.left == 0
            } else {
                check_piece.left == new_current[idx - 1].right
            };

            if fits_top && fits_left {
                new_current.push(check_piece);
                self.recursive_solve(new_current, new_remaining);
            }
        }
    }

    pub fn check_solutions(&self) -> bool {
        let mut seen_solutions = std::collections::HashSet::new();
        self.solutions
            .iter()
            .all(|sol| validate_puzzle(sol) && seen_solutions.insert(sol.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rand_int_with_max() {
        let rand_num = get_rand_int_with_max(5, &mut rand::thread_rng());
        assert!(rand_num >= 1 && rand_num <= 5);
    }

    #[test]
    fn flatten_puzzle_test() {
        let puzzle = generate_puzzle_with_max(5);
        let pieces = flatten_puzzle(&puzzle);
        assert_eq!(pieces.len(), SIZE * SIZE);
    }

    #[test]
    fn grow_puzzle_test() {
        let puzzle = generate_puzzle_with_max(5);
        let pieces = flatten_puzzle(&puzzle);
        let new_puzzle = grow_puzzle(pieces);
        assert_eq!(puzzle, new_puzzle);
    }

    #[test]
    fn generate_puzzle_generates_valid_puzzle() {
        let puzzle = generate_puzzle_with_max(5);
        assert!(validate_puzzle(&puzzle));
    }

    #[test]
    fn test_validate_puzzle() {
        let puzzle = generate_puzzle_with_max(5);
        assert!(validate_puzzle(&puzzle));
        let mut invalid_puzzle = puzzle;
        invalid_puzzle[0][0].right = 0;
        assert!(!validate_puzzle(&invalid_puzzle));
    }

    #[test]
    fn solution_contains_puzzle() {
        let mut pg = PuzzleGen::new(5);
        pg.new_puzzle();
        pg.solve();
        assert!(pg.solutions.contains(&pg.pieces));
    }
}
