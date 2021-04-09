use crate::common::*;
use std::collections::HashMap;

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

    pub fn translate(
        &self,
        pos: Point,
        d_theta: f64,
        direction: NormalVector,
        length: f64,
        radius: f64,
    ) -> crate::scene::ObjectContainer {
        let mut res = crate::scene::ObjectContainer::new();
        let mut pos = pos;
        let mut stack = Vec::new();
        let mut theta: f64 = 0.0;

        for v in self.value.iter() {
            match v {
                'X' => (),
                'F' => {
                    use crate::scene::texture::UniformTexture;
                    use crate::scene::Cylinder;

                    let Vector { x, y, z } = direction.vector();
                    let direction = Vector::new(
                        theta.cos() * x - theta.sin() * y,
                        theta.sin() * x + theta.cos() * y,
                        z,
                    );
                    let dst = (Vector::from(ORIGIN, pos) + length * direction).to_point();

                    res.push(Box::new(Cylinder::new(
                        pos,
                        dst,
                        radius,
                        UniformTexture::new(GREEN, 1.0, 1.0),
                    )));

                    pos = dst;
                }
                '+' => theta += d_theta,
                '-' => theta -= d_theta,
                '[' => stack.push((pos, theta)),
                ']' => {
                    let pair = stack.pop().unwrap();
                    pos = pair.0;
                    theta = pair.1;
                }
                c => panic!("Unallowed char {}", c),
            }
        }

        res
    }
}

impl ToString for LSystem {
    fn to_string(&self) -> String {
        self.value.iter().collect()
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
