use std::collections::HashSet;
use util::{read_lines, Result};

struct Board {
    tiles: Vec<char>,
    height: usize,
    width: usize,
}

impl Board {
    fn new(tiles: Vec<Vec<char>>) -> Self {
        let height = tiles.len();
        let width = tiles[0].len();
        let tiles = tiles.into_iter().flatten().collect::<Vec<_>>();
        Board {
            tiles,
            height,
            width,
        }
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        if x >= self.width {
            panic!("x too high");
        }

        if y >= self.height {
            panic!("y too high");
        }

        (x % self.width) + (y * self.width)
    }

    fn remove_person(&mut self, x: usize, y: usize) {
        let index = self.index_for(x, y);
        let tile = self.tiles[index];
        if tile != '#' {
            panic!(
                "can only remove person from occupied seat. tile at ({},{}) is {}",
                x, y, tile
            );
        }
        self.tiles[index] = 'L';
    }

    fn place_person(&mut self, x: usize, y: usize) {
        let index = self.index_for(x, y);
        let tile = self.tiles[index];
        if tile != 'L' {
            panic!(
                "can only place person in empty seat. tile at ({},{}) is {}",
                x, y, tile
            );
        }
        self.tiles[index] = '#';
    }

    fn tile_at(&self, x: usize, y: usize) -> char {
        self.tiles[self.index_for(x, y)]
    }

    fn is_occupied_seat(&self, x: usize, y: usize) -> bool {
        self.tile_at(x, y) == '#'
    }

    fn is_empty_seat(&self, x: usize, y: usize) -> bool {
        self.tile_at(x, y) == 'L'
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        match self.tile_at(x, y) {
            'L' | '.' => false,
            '#' => true,
            _ => unimplemented!(),
        }
    }

    fn num_adjacent_occupied(&self, x: usize, y: usize) -> usize {
        (y > 0 && self.is_occupied(x, y - 1)) as usize
            + (y > 0 && x < self.width - 1 && self.is_occupied(x + 1, y - 1)) as usize
            + (x < self.width - 1 && self.is_occupied(x + 1, y)) as usize
            + (x < self.width - 1 && y < self.height - 1 && self.is_occupied(x + 1, y + 1)) as usize
            + (y < self.height - 1 && self.is_occupied(x, y + 1)) as usize
            + (y < self.height - 1 && x > 0 && self.is_occupied(x - 1, y + 1)) as usize
            + (x > 0 && self.is_occupied(x - 1, y)) as usize
            + (x > 0 && y > 0 && self.is_occupied(x - 1, y - 1)) as usize
    }

    fn num_visible_occupied(&self, x: usize, y: usize) -> usize {
        self.occupied_visible_n(x, y) as usize
            + self.occupied_visible_ne(x, y) as usize
            + self.occupied_visible_e(x, y) as usize
            + self.occupied_visible_se(x, y) as usize
            + self.occupied_visible_s(x, y) as usize
            + self.occupied_visible_sw(x, y) as usize
            + self.occupied_visible_w(x, y) as usize
            + self.occupied_visible_nw(x, y) as usize
    }

    fn occupied_visible_nw(&self, mut x: usize, mut y: usize) -> bool {
        loop {
            if x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_w(&self, mut x: usize, y: usize) -> bool {
        loop {
            if x > 0 {
                x -= 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_sw(&self, mut x: usize, mut y: usize) -> bool {
        loop {
            if x > 0 && y < self.height - 1 {
                x -= 1;
                y += 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_s(&self, x: usize, mut y: usize) -> bool {
        loop {
            if y < self.height - 1 {
                y += 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_se(&self, mut x: usize, mut y: usize) -> bool {
        loop {
            if x < self.width - 1 && y < self.height - 1 {
                x += 1;
                y += 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_n(&self, x: usize, mut y: usize) -> bool {
        loop {
            if y > 0 {
                y -= 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_ne(&self, mut x: usize, mut y: usize) -> bool {
        loop {
            if y > 0 && x < self.width - 1 {
                y -= 1;
                x += 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn occupied_visible_e(&self, mut x: usize, y: usize) -> bool {
        loop {
            if x < self.width - 1 {
                x += 1;
                if self.is_empty_seat(x, y) {
                    break false;
                } else if self.is_occupied_seat(x, y) {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    fn num_occupied(&self) -> usize {
        self.tiles
            .iter()
            .copied()
            .filter(|tile| tile == &'#')
            .count()
    }
}

fn main() -> Result<()> {
    let input = read_lines("_11/input.txt")?
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut board = Board::new(input.clone());
    let mut to_remove = HashSet::<(usize, usize)>::default();
    let mut to_add = HashSet::<(usize, usize)>::default();

    loop {
        for y in 0..board.height {
            for x in 0..board.width {
                if board.is_empty_seat(x, y) && board.num_adjacent_occupied(x, y) == 0 {
                    to_add.insert((x, y));
                } else if board.is_occupied_seat(x, y) && board.num_adjacent_occupied(x, y) >= 4 {
                    to_remove.insert((x, y));
                }
            }
        }

        if to_remove.is_empty() && to_add.is_empty() {
            break;
        }

        to_add
            .iter()
            .copied()
            .for_each(|(x, y)| board.place_person(x, y));
        to_remove
            .iter()
            .copied()
            .for_each(|(x, y)| board.remove_person(x, y));

        to_remove.clear();
        to_add.clear();
    }

    println!("solution 1: {}", board.num_occupied());

    let mut board = Board::new(input);

    loop {
        for y in 0..board.height {
            for x in 0..board.width {
                if board.is_empty_seat(x, y) && board.num_visible_occupied(x, y) == 0 {
                    to_add.insert((x, y));
                } else if board.is_occupied_seat(x, y) && board.num_visible_occupied(x, y) >= 5 {
                    to_remove.insert((x, y));
                }
            }
        }

        if to_remove.is_empty() && to_add.is_empty() {
            break;
        }

        to_add
            .iter()
            .copied()
            .for_each(|(x, y)| board.place_person(x, y));
        to_remove
            .iter()
            .copied()
            .for_each(|(x, y)| board.remove_person(x, y));

        to_remove.clear();
        to_add.clear();
    }
    println!("solution 2: {}", board.num_occupied());
    Ok(())
}
