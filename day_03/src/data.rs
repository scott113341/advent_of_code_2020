use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum Node {
    Tree,
    Open,
}

pub type Row = Vec<Node>;
pub type Rows = Vec<Row>;

pub struct Map {
    pub rows: Rows,
}

impl Map {
    pub fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut tree_count = 0;
        let mut x = 0;
        let mut y = 0;

        while y < self.rows.len() {
            if self.node_at(x, y) == &Node::Tree {
                tree_count += 1;
            }

            x += dx;
            y += dy;
        }

        tree_count
    }

    // Account for repeating in the x-direction by taking (x % width)
    fn node_at(&self, x: usize, y: usize) -> &Node {
        let row = &self.rows[y];
        let x_idx = x % row.len();
        &row[x_idx]
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Rows::new();

        for line in s.split("\n") {
            let mut row = Row::new();
            for char in line.chars() {
                let node = match char {
                    '.' => Node::Open,
                    '#' => Node::Tree,
                    _ => panic!("Unknown character: '{}'", char),
                };
                row.push(node);
            }
            rows.push(row);
        }

        Ok(Map { rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 10
    // x=12 => r1
    fn example() -> Map {
        "
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#"
            .trim()
            .replace(' ', "")
            .parse()
            .unwrap()
    }

    #[test]
    fn test_map_from_str() {
        let map = example();
        assert_eq!(map.rows[0][0], Node::Open);
        assert_eq!(map.rows[0][1], Node::Open);
        assert_eq!(map.rows[0][2], Node::Tree);
    }

    #[test]
    fn test_map_count_trees() {
        let map = example();
        assert_eq!(map.count_trees(1, 1), 2);
        assert_eq!(map.count_trees(3, 1), 7);
        assert_eq!(map.count_trees(5, 1), 3);
        assert_eq!(map.count_trees(7, 1), 4);
        assert_eq!(map.count_trees(1, 2), 2);
    }
}
