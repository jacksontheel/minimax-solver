mod tests;

use crate::solver::gamestate::GameState;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TicTacToeSpot {
    Empty,
    X,
    O,
}

#[derive(Clone, Copy)]
pub struct TicTacToeGameState {
    pub spots: [[TicTacToeSpot; 3]; 3],
}

impl GameState for TicTacToeGameState {
    fn new() -> TicTacToeGameState {
        Self {
            spots: [[TicTacToeSpot::Empty; 3]; 3],
        }
    }

    fn evaluate(&self, first_player: bool) -> i32 {
        self.evaluate_solo(first_player) - self.evaluate_solo(!first_player)
    }

    fn get_children(&self, first_player: bool) -> Vec<Self> {
        let mut children = Vec::new();

        let spot = if first_player {
            TicTacToeSpot::X
        } else {
            TicTacToeSpot::O
        };

        for i in 0..3 {
            for j in 0..3 {
                if self.spots[i][j] == TicTacToeSpot::Empty {
                    let mut new_state = self.clone();
                    new_state.spots[i][j] = spot;
                    children.push(new_state);
                }
            }
        }
        children
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..3 {
            for j in 0..3 {
                result.push(if self.spots[i][j] == TicTacToeSpot::X {
                    'X'
                } else if self.spots[i][j] == TicTacToeSpot::O {
                    'O'
                } else {
                    'E'
                });
            }
        }
        result
    }
}

impl TicTacToeGameState {
    fn evaluate_solo(&self, first_player: bool) -> i32 {
        let mut score = 0;
        let spot = if first_player {
            TicTacToeSpot::X
        } else {
            TicTacToeSpot::O
        };

        for i in 0..3 {
            let h_score = self.evaluate_horizontal(i, spot);
            if h_score == 3 {
                return i32::MAX;
            }
            score += h_score;
        }

        for i in 0..3 {
            let v_score = self.evaluate_vertical(i, spot);
            if v_score == 3 {
                return i32::MAX;
            }
            score += v_score;
        }

        let mut d1_score = 0;
        for i in 0..3 {
            if self.spots[i][i] == spot {
                d1_score += 1;
            } else if self.spots[i][i] != TicTacToeSpot::Empty {
                d1_score = 0;
                break;
            }
        }
        if d1_score == 3 {
            return i32::MAX;
        }
        score += d1_score;

        let mut d2_score = 0;
        for i in 0..3 {
            if self.spots[2 - i][i] == spot {
                d2_score += 1;
            } else if self.spots[i][i] != TicTacToeSpot::Empty {
                d2_score = 0;
                break;
            }
        }
        if d1_score == 3 {
            return i32::MAX;
        }
        score += d2_score;

        score
    }

    fn evaluate_horizontal(&self, index: usize, spot: TicTacToeSpot) -> i32 {
        let mut score = 0;
        for i in 0..3 {
            if self.spots[index][i] == spot {
                score += 1;
            } else if self.spots[index][i] != TicTacToeSpot::Empty {
                score = 0;
                break;
            }
        }
        score
    }

    fn evaluate_vertical(&self, index: usize, spot: TicTacToeSpot) -> i32 {
        let mut score = 0;
        for i in 0..3 {
            if self.spots[i][index] == spot {
                score += 1;
            } else if self.spots[i][index] != TicTacToeSpot::Empty {
                score = 0;
                break;
            }
        }
        score
    }
}
