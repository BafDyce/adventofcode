use super::*;

pub(crate) fn solve(start_state: char, steps: usize, states: &HashMap<char, State>) -> usize {
    let mut tape: HashMap<i32, usize> = HashMap::new();

    let mut position = 0;
    let mut state = start_state;

    for __ in 1..=steps {
        let field = tape.entry(position).or_insert(0);
        let action = &states.get(&state).unwrap().actions[*field];
        *field = action.val;
        position += action.direction;
        state = action.next_state;
    }

    tape.values().sum()
}
