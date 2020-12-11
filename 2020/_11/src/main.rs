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
        let n = y > 0 && self.is_occupied(x, y - 1);
        let ne = y > 0 && x < self.width - 1 && self.is_occupied(x + 1, y - 1);
        let e = x < self.width - 1 && self.is_occupied(x + 1, y);
        let se = x < self.width - 1 && y < self.height - 1 && self.is_occupied(x + 1, y + 1);
        let s = y < self.height - 1 && self.is_occupied(x, y + 1);
        let sw = y < self.height - 1 && x > 0 && self.is_occupied(x - 1, y + 1);
        let w = x > 0 && self.is_occupied(x - 1, y);
        let nw = x > 0 && y > 0 && self.is_occupied(x - 1, y - 1);
        n as usize
            + ne as usize
            + e as usize
            + se as usize
            + s as usize
            + sw as usize
            + w as usize
            + nw as usize
    }

    fn num_visible_occupied(&self, x: usize, y: usize) -> usize {
        let n = self.occupied_visible_n(x, y);
        let ne = self.occupied_visible_ne(x, y);
        let e = self.occupied_visible_e(x, y);
        let se = self.occupied_visible_se(x, y);
        let s = self.occupied_visible_s(x, y);
        let sw = self.occupied_visible_sw(x, y);
        let w = self.occupied_visible_w(x, y);
        let nw = self.occupied_visible_nw(x, y);
        n as usize
            + ne as usize
            + e as usize
            + se as usize
            + s as usize
            + sw as usize
            + w as usize
            + nw as usize
    }

    fn occupied_visible_nw(&self, x: usize, y: usize) -> bool {
        let mut y = y;
        let mut x = x;
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

    fn occupied_visible_w(&self, x: usize, y: usize) -> bool {
        let mut x = x;
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

    fn occupied_visible_sw(&self, x: usize, y: usize) -> bool {
        let mut y = y;
        let mut x = x;
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

    fn occupied_visible_s(&self, x: usize, y: usize) -> bool {
        let mut y = y;
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

    fn occupied_visible_se(&self, x: usize, y: usize) -> bool {
        let mut y = y;
        let mut x = x;
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

    fn occupied_visible_n(&self, x: usize, y: usize) -> bool {
        let mut y = y;
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

    fn occupied_visible_ne(&self, x: usize, y: usize) -> bool {
        let mut y = y;
        let mut x = x;
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

    fn occupied_visible_e(&self, x: usize, y: usize) -> bool {
        let mut x = x;
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

    let mut board = Board::new(input);
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

    let input = read_lines("_11/input.txt")?
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut board = Board::new(input);
    let mut to_remove = HashSet::<(usize, usize)>::default();
    let mut to_add = HashSet::<(usize, usize)>::default();

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
