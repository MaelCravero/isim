use crate::common::*;
use std::{collections::HashMap, fs::File, io::BufRead};

use rand::seq::SliceRandom;

type LSConstant = char;

type LSValues = Vec<LSConstant>;
type LSRules = HashMap<LSConstant, Vec<LSValues>>;

#[derive(Debug, Clone)]
pub enum LSMaterial {
    Uniform(Color),
    Texture(String),
}
type LSColorTable = Vec<LSMaterial>;

#[derive(Debug, Clone)]
pub struct LSystem {
    value: LSValues,
    rules: LSRules,
    age: u64,
    delta: f64,
    trunk: u64,
    radius: f64,
    radius_decrease: f64,
    color_table: LSColorTable,
}

impl LSystem {
    pub fn new(
        axioms: LSValues,
        age: u64,
        delta: f64,
        trunk: u64,
        radius: f64,
        radius_decrease: f64,
    ) -> LSystem {
        LSystem {
            value: axioms,
            age,
            delta,
            trunk,
            radius,
            radius_decrease,
            rules: LSRules::new(),
            color_table: LSColorTable::new(),
        }
    }

    pub fn value(&self) -> &LSValues {
        &self.value
    }

    /// Return true if key has been modified
    pub fn add_rule(&mut self, symbol: LSConstant, rule: LSValues) {
        if !self.rules.contains_key(&symbol) {
            self.rules.insert(symbol, vec![rule]);
        } else {
            self.rules.get_mut(&symbol).unwrap().push(rule);
        }
    }

    pub fn add_material(&mut self, mat: LSMaterial) {
        self.color_table.push(mat)
    }

    pub fn with_colors(self, color_table: LSColorTable) -> LSystem {
        LSystem {
            color_table,
            ..self
        }
    }

    pub fn expand(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = self
            .value
            .iter()
            .map(|&v| {
                self.rules
                    .get(&v)
                    .map(|vec| vec.choose(&mut rng).unwrap().clone().to_owned())
                    .or(Some(vec![v]))
                    .unwrap()
            })
            .flatten()
            .collect::<Vec<char>>()
            .to_owned();
    }

    pub fn generate(mut self) -> LSystem {
        for i in 1..=self.age {
            println!("Expansion {}/{}", i, self.age);
            self.expand()
        }
        self
    }

    pub fn from_file(path: &str) -> std::io::Result<LSystem> {
        println!("Generating L-System from {}", path);

        let file = File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();

        // Prelude
        let age = lines.next().unwrap()?.parse::<u64>().unwrap();
        let delta = lines.next().unwrap()?.parse::<f64>().unwrap().to_radians();
        let trunk = lines.next().unwrap()?.parse::<u64>().unwrap();
        let radius = lines.next().unwrap()?.parse::<f64>().unwrap();
        let radius_decrease = lines.next().unwrap()?.parse::<f64>().unwrap();

        let mut colors = LSColorTable::new();

        for val in lines.next().unwrap()?.split_whitespace() {
            colors.push(match u32::from_str_radix(&val[0..6], 16) {
                Ok(_) => {
                    let r = u8::from_str_radix(&val[0..2], 16).unwrap();
                    let g = u8::from_str_radix(&val[2..4], 16).unwrap();
                    let b = u8::from_str_radix(&val[4..6], 16).unwrap();

                    LSMaterial::Uniform(Color(r, g, b))
                }
                Err(_) => LSMaterial::Texture(val.to_string()),
            })
        }

        // End of prelude
        let axioms = lines.next().unwrap()?;

        let mut res = LSystem::new(
            axioms.chars().collect(),
            age,
            delta,
            trunk,
            radius,
            radius_decrease,
        )
        .with_colors(colors);

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

use crate::scene::ObjectContainer;

type LSTResult = Vec<ObjectContainer>;

impl LSystem {
    pub fn translate(
        self,
        pos: Point,
        direction: NormalVector,
        right: NormalVector,
        length: f64,
    ) -> LSTResult {
        let state = LSTState {
            pos,
            direction,
            right,
            color: 0,
            radius: self.radius,
            obj_index: 0,
        };
        LSTranslator::new(
            self.delta,
            self.trunk,
            length,
            self.radius_decrease,
            self.color_table,
        )
        .run(state, &self.value)
    }
}

#[derive(Debug, Clone, Copy)]
struct LSTState {
    pos: Point,
    direction: NormalVector,
    right: NormalVector,
    color: usize,
    radius: f64,
    obj_index: usize,
}

impl LSTState {
    fn rotate_turn(&mut self, turn: f64) {
        let direction = self.direction.vector();
        let right = self.right.vector();
        let axis = Vector::cross_product(&direction, &right).normalize();

        self.direction = direction.rotate(&axis, turn).normalize();
        self.right = right.rotate(&axis, turn).normalize();
    }

    fn rotate_pitch(&mut self, pitch: f64) {
        self.direction = self
            .direction
            .vector()
            .rotate(&self.right, pitch)
            .normalize();
    }

    fn rotate_roll(&mut self, roll: f64) {
        self.right = self
            .right
            .vector()
            .rotate(&self.direction, roll)
            .normalize();
    }

    fn increase_color(&mut self, nb_color: usize) {
        self.color = (self.color + 1) % nb_color;
    }
}

type LSTStack = Vec<LSTState>;
type LSTLeave = Vec<Point>;

struct LSTranslator {
    delta: f64,
    trunk: u64,
    length: f64,
    radius_decrease: f64,
    saved_states: LSTStack,
    color_table: LSColorTable,
    res: LSTResult,
}

impl LSTranslator {
    fn new(
        delta: f64,
        trunk: u64,
        length: f64,
        radius_decrease: f64,
        color_table: LSColorTable,
    ) -> LSTranslator {
        LSTranslator {
            delta,
            trunk,
            length,
            radius_decrease,
            saved_states: LSTStack::new(),
            color_table,
            res: LSTResult::new(),
        }
    }

    fn get_material(&self, state: &LSTState) -> LSMaterial {
        self.color_table[state.color].clone()
    }

    fn add_edge(&mut self, state: &LSTState, dst: Point) {
        use crate::scene::texture::UVMapTexture;
        use crate::scene::texture::UniformTexture;
        use crate::scene::Cylinder;

        for i in state.obj_index..self.res.len() {
            match self.get_material(&state) {
                LSMaterial::Uniform(c) => {
                    let cylinder = Cylinder::new(
                        state.pos,
                        dst,
                        state.radius,
                        UniformTexture::new(c, 1.0, 0.3),
                    );
                    self.res[i].push(Box::new(cylinder));
                }
                LSMaterial::Texture(t) => {
                    let cylinder = Cylinder::new(
                        state.pos,
                        dst,
                        state.radius,
                        UVMapTexture::new(t, 1.0, 0.3),
                    );
                    self.res[i].push(Box::new(cylinder));
                }
            };
        }
    }

    fn generate_leaf(&mut self, state: &LSTState, leaf: &mut LSTLeave) {
        assert!(leaf.len() >= 3);

        use crate::scene::texture::UVMapTexture;
        use crate::scene::texture::UniformTexture;
        use crate::scene::Triangle;

        let v0 = leaf.pop().unwrap();
        let mut prev = leaf.pop().unwrap();

        while !leaf.is_empty() {
            let next = leaf.pop().unwrap();
            for i in state.obj_index..self.res.len() {
                match self.get_material(&state) {
                    LSMaterial::Uniform(c) => {
                        let triangle =
                            Triangle::new((v0, prev, next), UniformTexture::new(c, 1.0, 0.4));
                        self.res[i].push(Box::new(triangle));
                    }
                    LSMaterial::Texture(t) => {
                        let triangle =
                            Triangle::new((v0, prev, next), UVMapTexture::new(t, 1.0, 0.4));
                        self.res[i].push(Box::new(triangle));
                    }
                };
            }
            prev = next;
        }
    }

    fn compute_dst(&self, state: &LSTState) -> Point {
        let dst =
            (Vector::from(ORIGIN, state.pos) + self.length * state.direction.vector()).to_point();
        dst
    }

    fn compute_res_size(&mut self, values: &LSValues) {
        let mut size = 0;
        self.res.push(Vec::new());

        for val in values {
            match val {
                '[' => {
                    size += 1;
                    if size == self.res.len() {
                        self.res.push(Vec::new());
                        println!("LSystem steps increased to {}", size);
                    }
                }
                ']' => size -= 1,
                _ => (),
            }
        }
    }

    fn run(mut self, initial_state: LSTState, values: &LSValues) -> LSTResult {
        let mut state = initial_state;
        let mut leaf = LSTLeave::new();
        let mut in_leaf = false;

        self.compute_res_size(values);

        for val in values {
            match val {
                'f' | 'F' => {
                    let dst = self.compute_dst(&state);
                    if !in_leaf {
                        self.add_edge(&state, dst)
                    } else {
                        if state.obj_index >= self.trunk as usize {
                            leaf.push(dst)
                        }
                    }
                    state.pos = dst;
                }
                '!' => state.radius *= self.radius_decrease,
                '\'' => state.increase_color(self.color_table.len()),
                '+' => state.rotate_turn(self.delta),
                '-' => state.rotate_turn(-self.delta),
                '&' => state.rotate_pitch(self.delta),
                '^' => state.rotate_pitch(-self.delta),
                '\\' => state.rotate_roll(self.delta),
                '/' => state.rotate_roll(-self.delta),
                '|' => state.rotate_turn(180f64.to_radians()),
                '[' => {
                    self.saved_states.push(state.clone());
                    state.obj_index += 1;
                }
                ']' => state = self.saved_states.pop().unwrap(),
                '{' => {
                    assert!(leaf.is_empty());
                    //self.saved_states.push(state.clone());
                    in_leaf = true;
                }
                '}' => {
                    if state.obj_index >= self.trunk as usize {
                        self.generate_leaf(&state, &mut leaf);
                    }
                    //state = self.saved_states.pop().unwrap();
                    in_leaf = false;
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
        let mut lsystem = LSystem::new(vec!['a'], 0, 0.0, 0, 0.0, 0.0);
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
