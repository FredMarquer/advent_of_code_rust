use crate::solvers::prelude::*;

const PLAYER_COUNT: usize = 2;
const BOARD_SIZE: usize = 10;

static DIRAC_ROLLS: [DiracRoll; 7] = [
    DiracRoll::new(3, 1),
    DiracRoll::new(4, 3),
    DiracRoll::new(5, 6),
    DiracRoll::new(6, 7),
    DiracRoll::new(7, 6),
    DiracRoll::new(8, 3),
    DiracRoll::new(9, 1),
];

pub struct Day21 {
    starting_positions: [usize; PLAYER_COUNT],
}

impl FromStr for Day21 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day21 {
            starting_positions: get_starting_positions(s)
        })
    }
}

impl Solver for Day21 {
    const INPUT_PATH: &'static str = "inputs/2021/21.txt";

    fn run_part1(&self) -> SolverResult {
        let mut game_state = GameState::new(&self.starting_positions, 1000);
        let mut dice = DeterministicDice::new();

        let mut player_index = 0;
        loop {
            let rolls_result = dice.roll() + dice.roll() + dice.roll();
            if game_state.move_player(player_index, rolls_result) {
                break;
            }
            player_index = (player_index + 1) % PLAYER_COUNT;
        };

        (game_state.get_losing_player_score() * dice.roll_count).into()
    }

    fn run_part2(&self) -> SolverResult {
        let game_state = GameState::new(&self.starting_positions, 21);
        let mut player_win_counts = [0; PLAYER_COUNT];

        dirac_turn(game_state, 0, 1, &mut player_win_counts);

        (*player_win_counts.iter().max().unwrap()).into()
    }
}

fn get_starting_positions(input: &str) -> [usize; PLAYER_COUNT] {
    let mut starting_positions = [0; PLAYER_COUNT];

    let mut index = 0;
    for line in input.lines() {
        starting_positions[index] = get_player_starting_position(line);
        index += 1;
    }

    debug_assert!(index == PLAYER_COUNT, "The input player count (= {index}) doesn't match the expected player count (= {PLAYER_COUNT}).");

    starting_positions
}

fn get_player_starting_position(line: &str) -> usize {
    debug_assert!(line.len() > 28);
    let value: usize = line[28..line.len()].parse().unwrap();
    value - 1
}

fn dirac_turn(game_state: GameState, player_index: usize, universe_count: usize, player_win_counts: &mut [usize; 2]) {
    for dirac_roll in DIRAC_ROLLS.iter() {
        let mut game_state = game_state; // Copy the game state
        let universe_count = universe_count * dirac_roll.count;

        if game_state.move_player(player_index, dirac_roll.value) {
            player_win_counts[player_index] += universe_count;
        } else {
            let player_index = (player_index + 1) % PLAYER_COUNT;
            dirac_turn(game_state, player_index, universe_count, player_win_counts);
        }
    }
}

#[derive(Clone, Copy)]
struct GameState {
    player_positions: [usize; PLAYER_COUNT],
    player_scores: [usize; PLAYER_COUNT],
    score_limit: usize,
}

impl GameState {
    fn new(starting_positions: &[usize; PLAYER_COUNT], score_limit: usize) -> GameState {
        GameState {
            player_positions: *starting_positions,
            player_scores: [0; PLAYER_COUNT],
            score_limit,
        }
    }

    fn move_player(&mut self, player_index: usize, distance: usize) -> bool {
        self.player_positions[player_index] = (self.player_positions[player_index] + distance) % BOARD_SIZE;
        self.player_scores[player_index] += self.player_positions[player_index] + 1;

        self.player_scores[player_index] >= self.score_limit
    }

    fn get_losing_player_score(&self) -> usize {
        *self.player_scores.iter().min().unwrap()
    }
}

struct DeterministicDice {
    roll_count: usize,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> usize {
        let value = (self.roll_count % 100) + 1;
        self.roll_count += 1;
        value
    }
}

struct DiracRoll {
    value: usize,
    count: usize,
}

impl DiracRoll {
    const fn new(value: usize, count: usize) -> DiracRoll {
        DiracRoll { value, count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        static TEST_INPUT: &str = indoc!{"
            Player 1 starting position: 4
            Player 2 starting position: 8
        "};

        let day = Day21::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 739785.into(), "Part1");
        assert_eq!(day.run_part2(), 444356092776315_i64.into(), "Part2");
    }
}