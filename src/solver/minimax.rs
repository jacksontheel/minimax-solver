use std::collections::HashMap;

use super::gamestate::GameState;

pub fn make_move<T: Copy + GameState>(state: &T, depth: i32, first_player: bool) -> T {
    let mut table = HashMap::new();
    let (result, _) = minimax(state, &mut table, depth, i32::MIN, i32::MAX, first_player);
    result
}

fn minimax<T: Copy + GameState>(
    state: &T,
    transposition_table: &mut HashMap<String, (T, i32)>,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    first_player: bool,
) -> (T, i32) {
    let children = state.get_children(first_player);
    if depth == 0 || children.len() == 0 {
        let value = state.evaluate(true);
        transposition_table.insert(state.to_string(), (state.clone(), value));
        (state.clone(), value)
    } else if let Some(v) = transposition_table.get(&state.to_string()) {
        *v
    } else if first_player {
        let mut maximum = i32::MIN;
        let mut ret_state = T::new();
        for child in children.iter() {
            let (_, value) = minimax(child, transposition_table, depth - 1, alpha, beta, false);
            if value > maximum {
                maximum = value;
                ret_state = child.clone();
            }
            if value > beta {
                break;
            }
            alpha = std::cmp::max(alpha, value);
        }
        transposition_table.insert(state.to_string(), (ret_state, maximum));
        (ret_state, maximum)
    } else {
        let mut minimum = i32::MAX;
        let mut ret_state = T::new();
        for child in children.iter() {
            let (_, value) = minimax(child, transposition_table, depth - 1, alpha, beta, true);
            if value < minimum {
                minimum = value;
                ret_state = child.clone();
            }
            if value < alpha {
                break;
            }
            beta = std::cmp::min(beta, value);
        }
        transposition_table.insert(state.to_string(), (ret_state, minimum));
        (ret_state, minimum)
    }
}
