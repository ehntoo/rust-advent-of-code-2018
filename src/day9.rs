use std::boxed::Box;

#[derive(PartialEq, Debug, Clone)]
pub struct GameResult {
    players: usize,
    last_marble: usize,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Box<GameResult> {
    let mut parts = input.trim().split(" ");
    let players = parts.next().unwrap().parse().unwrap();
    let last_marble = parts.skip(5).next().unwrap().parse().unwrap();
    Box::new(GameResult{players, last_marble})
}

pub struct GameState {
    players: usize,
    current_player: usize,
    current_marble_idx: usize,
    placed_marbles: Vec<u32>,
    player_scores: Vec<u32>,
}

fn perform_turn(state: &mut GameState, marble: u32) -> () {
    if marble % 23 == 0 {
        let mut bonus_idx = state.current_marble_idx as i32 - 7;
        while bonus_idx < 0 { bonus_idx += state.placed_marbles.len() as i32; }
        let bonus_marble = state.placed_marbles.remove(bonus_idx as usize);
        let points_scored = bonus_marble + marble;
        state.player_scores[state.current_player] += points_scored;
        state.current_marble_idx = bonus_idx as usize;
    } else {
        let mut placing_idx = state.current_marble_idx as i32 + 2;
        while placing_idx > state.placed_marbles.len() as i32 { placing_idx -= state.placed_marbles.len() as i32; }
        // println!("Placing marble at index: {:?}", placing_idx);
        state.placed_marbles.insert(placing_idx as usize, marble);
        state.current_marble_idx = placing_idx as usize;
    }

    state.current_player = (state.current_player + 1) % state.players
}

#[aoc(day9, part1)]
pub fn solve_part1(result: &GameResult) -> u32 {
    let mut state = GameState {
        players: result.players,
        current_player: 0,
        current_marble_idx: 0,
        placed_marbles: vec![0],
        player_scores: vec![0; result.players],
    };
    // println!("Game state was: {:?}", state.placed_marbles);
    for i in 1..=result.last_marble {
        perform_turn(&mut state, i as u32);
        // println!("Game state was: next player {:?} next_idx {:?} {:?}", state.current_player + 1, state.current_marble_idx, state.placed_marbles);
    }
    *state.player_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&GameResult{players: 9, last_marble: 25}), 32);
        assert_eq!(solve_part1(&GameResult{players: 17, last_marble: 1104}), 2764);
        assert_eq!(solve_part1(&GameResult{players: 10, last_marble: 1618}), 8317);
        assert_eq!(solve_part1(&GameResult{players: 13, last_marble: 7999}), 146373);
    }
}
