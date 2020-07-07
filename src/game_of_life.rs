use itertools::*;
use rand::Rng;
use std::convert::TryInto;
use std::fmt;
use std::vec::Vec;

pub struct GameOfLife {
    state: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(h: usize, w: usize) -> GameOfLife {
        let mut state = Vec::new();
        for _ in 0..h {
            state.push(vec![false; w]);
        }

        GameOfLife { state: state }
    }

    pub fn new_random(h: usize, w: usize, alive_probability: f64) -> GameOfLife {
        let mut game = GameOfLife::new(h, w);

        for (x, y) in iproduct!(0..w, 0..h) {
            let random_num: f64 = rand::thread_rng().gen_range(0.0, 1.0);
            if random_num < alive_probability {
                game.set(x, y, true);
            }
        }

        game
    }

    fn set_cell(state: &mut Vec<Vec<bool>>, x: usize, y: usize, value: bool) -> Option<()> {
        state.get_mut(y).and_then(|line| {
            if let Some(_) = line.get(x) {
                line[x] = value;
                Some(())
            } else {
                None
            }
        })
    }

    pub fn set_state(&mut self, state: Vec<Vec<bool>>) {
        self.state = state
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) -> Option<()> {
        GameOfLife::set_cell(&mut self.state, x, y, alive)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<bool> {
        let yy: usize = match y.try_into().ok() {
            Some(n) => n,
            None => return None,
        };

        let xx: usize = match x.try_into().ok() {
            Some(n) => n,
            None => return None,
        };

        self.state.get(yy).and_then(|line| line.get(xx)).copied()
    }

    pub fn step(&mut self) {
        let mut next_state = self.state.clone();

        for (y, row) in self.state.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let n_neighbours = self.count_neighbours(x, y);
                let alive = match (cell, n_neighbours) {
                    (true, n) if n < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, n) if n > 3 => false,
                    (false, 3) => true,
                    _ => false,
                };

                GameOfLife::set_cell(&mut next_state, x, y, alive);
            }
        }

        self.set_state(next_state);
    }

    fn count_neighbours(&self, x: usize, y: usize) -> i8 {
        let mut count = 0;
        let xx: i32 = x.try_into().unwrap();
        let yy: i32 = y.try_into().unwrap();

        for (i, j) in iproduct!(xx - 1..xx + 2, yy - 1..yy + 2) {
            if i == xx && j == yy {
                continue;
            }

            if let Some(true) = self.get(i, j) {
                count += 1;
            }
        }
        count
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.state {
            for (pos, cell) in line.iter().enumerate() {
                let marker = if *cell { "x" } else { " " };
                let maybe_sep = if pos == line.len() - 1 { "|" } else { "" };
                let result = write!(f, "|{}{}", marker, maybe_sep);
                if let Err(_) = result {
                    return result;
                }
            }

            let result = write!(f, "\n");
            if let Err(_) = result {
                return result;
            }
        }

        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_bounds() {
        let game = GameOfLife::new(10, 10);

        assert_eq!(Some(false), game.get(0, 0));
        assert_eq!(Some(false), game.get(9, 0));
        assert_eq!(Some(false), game.get(9, 9));
        assert_eq!(Some(false), game.get(0, 9));

        assert_eq!(None, game.get(10, 0));
        assert_eq!(None, game.get(10, 10));
        assert_eq!(None, game.get(0, 10));
    }

    #[test]
    fn set_cell() {
        let mut game = GameOfLife::new(10, 10);
        assert_eq!(Some(()), game.set(0, 9, true));
        assert_eq!(Some(true), game.get(0, 9));

        assert_eq!(None, game.set(0, 10, true));
    }
}
