use crate::common::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

type LSConstant = char;

type LSValues = Vec<LSConstant>;
type LSRules = HashMap<LSConstant, LSValues>;

#[derive(Debug, Clone)]
pub struct LSystem {
    value: LSValues,
    rules: LSRules,
}

impl LSystem {
    pub fn new(axioms: LSValues) -> LSystem {
        LSystem {
            value: axioms,
            rules: LSRules::new(),
        }
    }

    pub fn value(&self) -> &LSValues {
        &self.value
    }

    /// Return true if key has been modified
    pub fn add_rule(&mut self, symbol: LSConstant, rule: LSValues) -> bool {
        self.rules.insert(symbol, rule).is_some()
    }

    pub fn expand(&mut self) {
        self.value = self
            .value
            .iter()
            .map(|&v| {
                self.rules
                    .get(&v)
                    .map(|vec| vec.clone().to_owned())
                    .or(Some(vec![v]))
                    .unwrap()
            })
            .flatten()
            .collect::<Vec<char>>()
            .to_owned();
    }

    pub fn from_file(path: &str) -> std::io::Result<LSystem> {
        let file = File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();

        let axioms = lines.next().unwrap()?;

        let mut res = LSystem::new(axioms.chars().collect());

        for rule in lines {
            let mut chars: LSValues = rule?.chars().collect();

            let token = chars[0];

            chars.remove(0);
            chars.remove(0);

            res.add_rule(token, chars);
        }

        Ok(res)
    }
}

impl ToString for LSystem {
    fn to_string(&self) -> String {
        self.value.iter().collect()
    }
}

impl LSystem {
    pub fn translate(
        &self,
        pos: Point,
        d_turn: f64,
        d_roll: f64,
        d_pitch: f64,
        direction: NormalVector,
        length: f64,
        radius: f64,
    ) -> crate::scene::ObjectContainer {
        let state = LSTState {
            pos,
            turn: 0.0,
            roll: 0.0,
            pitch: 0.0,
        };
        LSTranslator::new(direction, d_turn, d_roll, d_pitch, length, radius)
            .run(state, &self.value)
    }
}

use crate::scene::ObjectContainer as LSTResult;

#[derive(Debug, Clone, Copy)]
struct LSTState {
    pos: Point,
    turn: f64,
    roll: f64,
    pitch: f64,
}

type LSTStack = Vec<LSTState>;

struct LSTranslator {
    direction: NormalVector,
    d_turn: f64,
    d_roll: f64,
    d_pitch: f64,
    length: f64,
    radius: f64,
    saved_states: LSTStack,
    res: LSTResult,
}

impl LSTranslator {
    fn new(
        direction: NormalVector,
        d_turn: f64,
        d_roll: f64,
        d_pitch: f64,
        length: f64,
        radius: f64,
    ) -> LSTranslator {
        LSTranslator {
            direction,
            d_turn,
            d_roll,
            d_pitch,
            length,
            radius,
            saved_states: LSTStack::new(),
            res: LSTResult::new(),
        }
    }

    fn add_edge(&mut self, state: &LSTState, dst: Point) {
        use crate::scene::texture::UniformTexture;
        use crate::scene::Cylinder;

        self.res.push(Box::new(Cylinder::new(
            state.pos,
            dst,
            self.radius,
            UniformTexture::new(GREEN, 1.0, 1.0),
        )));
    }

    fn rotate(x: f64, y: f64, z: f64, state: &LSTState) -> (f64, f64, f64) {
        let (x, y, z) = (
            state.turn.cos() * x - state.turn.sin() * y,
            state.turn.sin() * x + state.turn.cos() * y,
            z,
        );
        let (x, y, z) = (
            state.pitch.cos() * x + state.pitch.sin() * z,
            y,
            -state.pitch.sin() * x + state.pitch.cos() * z,
        );
        let (x, y, z) = (
            x,
            state.roll.cos() * y - state.roll.sin() * z,
            state.roll.sin() * y + state.roll.cos() * z,
        );
        (x, y, z)
    }

    fn compute_dst(&self, state: &LSTState) -> Point {
        let Vector { x, y, z } = self.direction.vector();
        let (x, y, z) = LSTranslator::rotate(x, y, z, &state);
        let direction = Vector::new(x, y, z);
        let dst = (Vector::from(ORIGIN, state.pos) + self.length * direction).to_point();
        dst
    }

    fn run(mut self, initial_state: LSTState, values: &LSValues) -> LSTResult {
        let mut state = initial_state;

        for val in values {
            match val {
                'x' | 'X' => (),
                'f' | 'F' => {
                    let dst = self.compute_dst(&state);
                    self.add_edge(&state, dst);
                    state.pos = dst;
                }
                '+' => state.turn += self.d_turn,
                '-' => state.turn -= self.d_turn,
                '&' => state.pitch += self.d_pitch,
                '^' => state.pitch -= self.d_pitch,
                '\\' => state.roll += self.d_roll,
                '/' => state.roll -= self.d_roll,
                '|' => state.turn += 180f64.to_radians(),
                '[' => self.saved_states.push(state.clone()),
                ']' => state = self.saved_states.pop().unwrap(),
                c => panic!("Unallowed char {}", c),
            }
        }

        self.res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_algae() {
        let mut lsystem = LSystem::new(vec!['a']);
        lsystem.add_rule('a', vec!['a', 'b']);
        lsystem.add_rule('b', vec!['a']);

        assert_eq!(lsystem.to_string(), "a");

        lsystem.expand();
        assert_eq!(lsystem.to_string(), "ab");

        lsystem.expand();
        assert_eq!(lsystem.to_string(), "aba");

        lsystem.expand();
        assert_eq!(lsystem.to_string(), "abaab");

        lsystem.expand();
        assert_eq!(lsystem.to_string(), "abaababa");
    }
}
