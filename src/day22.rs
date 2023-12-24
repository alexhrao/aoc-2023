use std::{collections::HashSet, fmt::Debug, fs, str::FromStr};

use super::Day;
use regex::Regex;
pub struct Day22;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

fn intersects(r1: (usize, usize), r2: (usize, usize)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

impl Brick {
    pub fn plane(z: usize) -> Brick {
        Brick {
            start: (usize::MIN, usize::MIN, z),
            end: (usize::MAX, usize::MAX, z),
        }
    }

    pub fn xy_intersects(&self, other: &Brick) -> bool {
        // For it NOT to rest, either the x range is completely different,
        // or the y range is.
        let (x11, y11, _) = self.start;
        let (x12, y12, _) = self.end;
        let (x21, y21, _) = other.start;
        let (x22, y22, _) = other.end;
        !(!intersects((x11, x12), (x21, x22)) || !intersects((y11, y12), (y21, y22)))
    }
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
        if let Some(caps) = re.captures(s) {
            let nums: Vec<_> = (1..=6)
                .map(|i| caps.get(i).map(|c| c.as_str()).unwrap().parse().unwrap())
                .collect();
            Ok(Brick {
                start: (nums[0], nums[1], nums[2]),
                end: (nums[3], nums[4], nums[5]),
            })
        } else {
            Err(())
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node<'a, T> {
    supporters: Vec<usize>,
    supporting: Vec<usize>,
    value: &'a T,
}

impl<'a, T> Node<'a, T> {
    pub fn is_redundant(&self, tree: &'a Tree<'a, T>) -> bool {
        self.supporting
            .iter()
            .all(|m| tree[*m].supporters.len() > 1)
    }
}

impl<'a> From<&'a Brick> for Node<'a, Brick> {
    fn from(value: &'a Brick) -> Self {
        Node {
            supporters: vec![],
            supporting: vec![],
            value,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Tree<'a, T> {
    nodes: Vec<Node<'a, T>>,
}

impl<'a> Tree<'a, Brick> {
    pub fn supported(&'a self) -> usize {
        let mut sum = 0;
        for n in 1..self.len() {
            // Make a copy for what-if analysis
            let mut tree = self.clone();
            let mut killed = vec![n];
            let mut checked = HashSet::new();
            loop {
                for k in killed {
                    // Clone so that way there's no WAY that we accidentally trample ourselves
                    for supported in tree[k].supporting.clone() {
                        tree[supported].supporters = tree[supported]
                            .supporters
                            .iter()
                            .filter(|&&i| i != k)
                            .copied()
                            .collect();
                    }
                    checked.insert(k);
                }
                // We've erased everyone in this round. Is there anyone new?
                // Skip the plane; otherwise it's turtles all the way down
                killed = tree
                    .iter()
                    .enumerate()
                    .skip(1)
                    .filter_map(|(n, node)| {
                        if !checked.contains(&n) && node.supporters.is_empty() {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .collect();
                if killed.is_empty() {
                    break;
                }
            }
            sum += tree
                .iter()
                .skip(1)
                .filter(|n| n.supporters.is_empty())
                .count();
        }
        sum
    }
}

impl<'a, T> std::ops::Deref for Tree<'a, T> {
    type Target = Vec<Node<'a, T>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl<'a, T> std::ops::DerefMut for Tree<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}

impl<'a> From<&'a mut Vec<Brick>> for Tree<'a, Brick> {
    fn from(bricks: &'a mut Vec<Brick>) -> Self {
        bricks.sort_unstable_by_key(|b| b.start.2);
        let mut bricks: Vec<&mut Brick> = bricks.iter_mut().collect();
        let root = Node::from(&*bricks.drain(0..1).next().unwrap());
        let mut placed = vec![root];
        for brick in bricks {
            // See where this rests. It will rest on a brick that
            // has an end z-index at (me.start.z - 1), and whose xy surface
            // occludes, at least a little bit, with my xy plane
            let mut node = None;
            // First check if anyone that's been placed has both matching z and xy inter
            for z in (1..=brick.start.2).rev() {
                if placed
                    .iter()
                    .any(|p| p.value.end.2 == (z - 1) && brick.xy_intersects(p.value))
                {
                    // At least one placed brick would support me. Update my z, append to places, and exit
                    // I'm guaranteed to hit this eventually, because we have the bottom plane
                    let diff = brick.start.2 - z;
                    brick.start.2 = z;
                    brick.end.2 -= diff;
                    node = Some(Node::from(&*brick));
                    break;
                }
            }
            placed.push(node.unwrap());
        }

        let mut nodes = placed;

        let supporters: Vec<_> = nodes
            .iter()
            .map(|node| {
                if node.value.end.2 == 0 {
                    vec![]
                } else {
                    nodes
                        .iter()
                        .enumerate()
                        .filter_map(|(p, brick)| {
                            if brick.value.end.2 == (node.value.start.2 - 1)
                                && node.value.xy_intersects(brick.value)
                            {
                                Some(p)
                            } else {
                                None
                            }
                        })
                        .collect()
                }
            })
            .collect();

        for (brick, supporters) in nodes.iter_mut().zip(supporters) {
            brick.supporters.extend(supporters);
        }
        let supporting: Vec<_> = nodes
            .iter()
            .map(|node| {
                let supporting: Vec<usize> = nodes
                    .iter()
                    .enumerate()
                    .filter_map(|(p, brick)| {
                        if brick.value.end.2 == 0 {
                            None
                        } else if node.value.end.2 == (brick.value.start.2 - 1)
                            && node.value.xy_intersects(brick.value)
                        {
                            Some(p)
                        } else {
                            None
                        }
                    })
                    .collect();
                supporting
            })
            .collect();
        for (brick, supporting) in nodes.iter_mut().zip(supporting) {
            brick.supporting.extend(supporting);
        }

        Tree { nodes }
    }
}

// fn settle(bricks: &mut Vec<Brick>) -> Vec<Node<'_, Brick>> {
//     // sort by their z-index

// }

impl Day for Day22 {
    fn task1(&self, file: &std::path::Path) {
        let mut bricks: Vec<Brick> = std::iter::once(Brick::plane(0))
            .chain(
                fs::read_to_string(file)
                    .unwrap()
                    .lines()
                    .map(|l| l.parse().unwrap()),
            )
            .collect();
        let tree = Tree::from(&mut bricks);
        let disintegrated = tree
            .iter()
            .skip(1)
            .filter(|n| n.is_redundant(&tree))
            .count();
        println!("{}", disintegrated);
    }
    fn task2(&self, file: &std::path::Path) {
        let mut bricks: Vec<Brick> = std::iter::once(Brick::plane(0))
            .chain(
                fs::read_to_string(file)
                    .unwrap()
                    .lines()
                    .map(|l| l.parse().unwrap()),
            )
            .collect();
        let tree = Tree::from(&mut bricks);
        println!("{}", tree.supported());
    }
}

#[cfg(test)]
mod test {
    use super::Brick;

    #[test]
    pub fn plane_checks() {
        let b1 = Brick {
            start: (0, 0, 1),
            end: (1, 1, 1),
        };
        let mut b2 = Brick {
            start: (2, 2, 0),
            end: (3, 2, 1),
        };
        assert!(!b1.xy_intersects(&b2));
        b2.start = (1, 1, 0);
        assert!(b1.xy_intersects(&b2));
    }

    #[test]
    pub fn test_bricks() {
        let a = Brick {
            start: (1, 0, 1),
            end: (1, 2, 1),
        };
        let b = Brick {
            start: (0, 0, 2),
            end: (2, 0, 2),
        };
        let c = Brick {
            start: (0, 2, 3),
            end: (2, 2, 3),
        };
        let f = Brick {
            start: (0, 1, 4),
            end: (2, 1, 4),
        };
        let g = Brick {
            start: (1, 1, 5),
            end: (1, 1, 6),
        };
        assert!(a.xy_intersects(&b));
        assert!(!b.xy_intersects(&c));
        assert!(g.xy_intersects(&f));
        assert_eq!(f.end.2, g.start.2 - 1);
    }
}
