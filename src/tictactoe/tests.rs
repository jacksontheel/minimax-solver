use crate::solver::gamestate::GameState;
use crate::tictactoe::TicTacToeGameState;
use crate::tictactoe::TicTacToeSpot;

#[allow(dead_code)]
struct Case {
    input: [TicTacToeSpot; 9],
    exp_x: i32,
    exp_o: i32,
}

#[allow(dead_code)]
fn case_to_game_state(input: [TicTacToeSpot; 9]) -> TicTacToeGameState {
    let mut state = TicTacToeGameState::new();
    for i in 0..9 {
        state.spots[i / 3][i % 3] = input[i];
    }
    state
}

#[test]
fn test_parse_int() {
    let test_cases = vec![
        Case {
            input: [
                TicTacToeSpot::X,
                TicTacToeSpot::X,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
            ],
            exp_x: 5,
            exp_o: 0,
        },
        Case {
            input: [
                TicTacToeSpot::X,
                TicTacToeSpot::X,
                TicTacToeSpot::O,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::O,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::O,
            ],
            exp_x: 2,
            exp_o: i32::MAX,
        },
        Case {
            input: [
                TicTacToeSpot::X,
                TicTacToeSpot::X,
                TicTacToeSpot::O,
                TicTacToeSpot::O,
                TicTacToeSpot::O,
                TicTacToeSpot::O,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::O,
            ],
            exp_x: 0,
            exp_o: i32::MAX,
        },
        Case {
            input: [
                TicTacToeSpot::X,
                TicTacToeSpot::Empty,
                TicTacToeSpot::O,
                TicTacToeSpot::Empty,
                TicTacToeSpot::X,
                TicTacToeSpot::O,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
            ],
            exp_x: 4,
            exp_o: 2,
        },
        Case {
            input: [
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::X,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
                TicTacToeSpot::Empty,
            ],
            exp_x: 4,
            exp_o: 0,
        },
    ];

    for case in test_cases {
        let gs = case_to_game_state(case.input);
        assert_eq!(gs.evaluate(true), case.exp_x);
        assert_eq!(gs.evaluate(false), case.exp_o);
    }
}
