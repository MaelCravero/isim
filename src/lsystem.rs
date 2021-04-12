use crate::common::*;
use std::{collections::HashMap, fs::File, io::BufRead};

type LSConstant = char;

type LSValues = Vec<LSConstant>;
type LSRules = HashMap<LSConstant, LSValues>;
type LSColorTable = Vec<Color>;

#[derive(Debug, Clone)]
pub struct LSystem {
    value: LSValues,
    rules: LSRules,
    age: u64,
    delta: f64,
    color_table: LSColorTable,
}

impl LSystem {
    pub fn new(axioms: LSValues, age: u64, delta: f64) -> LSystem {
        LSystem {
            value: axioms,
            age,
            delta,
            rules: LSRules::new(),
            color_table: LSColorTable::new(),
        }
    }

    pub fn value(&self) -> &LSValues {
        &self.value
    }

    /// Return true if key has been modified
    pub fn add_rule(&mut self, symbol: LSConstant, rule: LSValues) -> bool {
        self.rules.insert(symbol, rule).is_some()
    }

    pub fn add_color(&mut self, color: Color) {
        self.color_table.push(color)
    }

    pub fn with_colors(self, color_table: LSColorTable) -> LSystem {
        LSystem {
            color_table,
            ..self
        }
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

    pub fn generate(mut self) -> LSystem {
        for _ in 0..self.age {
            self.expand()
        }
        self
    }

    pub fn from_file(path: &str) -> std::io::Result<LSystem> {
        let file = File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();

        // Prelude
        let age = lines.next().unwrap()?.parse::<u64>().unwrap();
        let delta = lines.next().unwrap()?.parse::<f64>().unwrap().to_radians();

        let mut colors = LSColorTable::new();

        for val in lines.next().unwrap()?.split_whitespace() {
            let r = u8::from_str_radix(&val[0..2], 16).unwrap();
            let g = u8::from_str_radix(&val[2..4], 16).unwrap();
            let b = u8::from_str_radix(&val[4..6], 16).unwrap();

            colors.push(Color(r, g, b))
        }

        // End of prelude
        let axioms = lines.next().unwrap()?;

        let mut res = LSystem::new(axioms.chars().collect(), age, delta).with_colors(colors);

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
        direction: NormalVector,
        length: f64,
        radius: f64,
    ) -> crate::scene::ObjectContainer {
        let state = LSTState { pos, direction };
        LSTranslator::new(self.delta, length, radius).run(state, &self.value)
    }
}

use crate::scene::ObjectContainer as LSTResult;

#[derive(Debug, Clone, Copy)]
struct LSTState {
    pos: Point,
    direction: NormalVector,
}

impl LSTState {
    fn rotate_turn(&mut self, turn: f64) {
        let Vector { x, y, z } = self.direction.vector();
        let (x, y, z) = (
            turn.cos() * x - turn.sin() * y,
            turn.sin() * x + turn.cos() * y,
            z,
        );
        self.direction = Vector::new(x, y, z).normalize();
    }

    fn rotate_pitch(&mut self, pitch: f64) {
        let Vector { x, y, z } = self.direction.vector();
        let (x, y, z) = (
            pitch.cos() * x + pitch.sin() * z,
            y,
            -pitch.sin() * x + pitch.cos() * z,
        );
        self.direction = Vector::new(x, y, z).normalize();
    }

    fn rotate_roll(&mut self, roll: f64) {
        let Vector { x, y, z } = self.direction.vector();
        let (x, y, z) = (
            x,
            roll.cos() * y - roll.sin() * z,
            roll.sin() * y + roll.cos() * z,
        );
        self.direction = Vector::new(x, y, z).normalize();
    }
}

type LSTStack = Vec<LSTState>;
type LSTLeave = Vec<Point>;

struct LSTranslator {
    delta: f64,
    length: f64,
    radius: f64,
    saved_states: LSTStack,
    res: LSTResult,
}

impl LSTranslator {
    fn new(delta: f64, length: f64, radius: f64) -> LSTranslator {
        LSTranslator {
            delta,
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

    fn generate_leaf(&mut self, leaf: &mut LSTLeave) {
        assert!(leaf.len() >= 3);

        use crate::scene::texture::UniformTexture;
        use crate::scene::Triangle;

        let v0 = leaf.pop().unwrap();
        let mut prev = leaf.pop().unwrap();

        while !leaf.is_empty() {
            let next = leaf.pop().unwrap();
            let triangle = Triangle::new((v0, prev, next), UniformTexture::new(RED, 1.0, 1.0));
            self.res.push(Box::new(triangle));
            prev = next;
        }
    }

    fn compute_dst(&self, state: &LSTState) -> Point {
        let dst =
            (Vector::from(ORIGIN, state.pos) + self.length * state.direction.vector()).to_point();
        dst
    }

    fn run(mut self, initial_state: LSTState, values: &LSValues) -> LSTResult {
        let mut state = initial_state;
        let mut leaf = LSTLeave::new();
        let in_leaf = false;

        for val in values {
            match val {
                'f' | 'F' => {
                    let dst = self.compute_dst(&state);
                    if !in_leaf {
                        self.add_edge(&state, dst)
                    } else {
                        leaf.push(dst)
                    }
                    state.pos = dst;
                }
                '+' => state.rotate_turn(self.delta),
                '-' => state.rotate_turn(-self.delta),
                '&' => state.rotate_pitch(self.delta),
                '^' => state.rotate_pitch(-self.delta),
                '\\' => state.rotate_roll(self.delta),
                '/' => state.rotate_roll(-self.delta),
                '|' => state.rotate_turn(180f64.to_radians()),
                '[' => self.saved_states.push(state.clone()),
                ']' => state = self.saved_states.pop().unwrap(),
                '{' => {
                    assert!(leaf.is_empty());
                    self.saved_states.push(state.clone());
                }
                '}' => {
                    self.generate_leaf(&mut leaf);
                    state = self.saved_states.pop().unwrap();
                }
                _ => (),
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
        let mut lsystem = LSystem::new(vec!['a'], 0, 0.0);
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
