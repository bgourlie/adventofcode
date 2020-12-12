use util::{read_lines, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    Right,
    Left,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

#[derive(Default)]
struct Ship {
    facing: Direction,
    x: i32,
    y: i32,
}

struct Waypoint {
    rel_x: i32,
    rel_y: i32,
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint {
            rel_x: 10,
            rel_y: 1,
        }
    }
}

#[derive(Default)]
struct World {
    ship: Ship,
    waypoint: Waypoint,
}

impl World {
    fn move_ship_toward_waypoint(&mut self, amount: u16) {
        self.ship.x += i32::from(amount) * self.waypoint.rel_x;
        self.ship.y += i32::from(amount) * self.waypoint.rel_y;
    }

    fn move_waypoint(&mut self, direction: Direction, amount: u16) {
        let amount = i32::from(amount);
        match direction {
            Direction::North => self.waypoint.rel_y += amount,
            Direction::East => self.waypoint.rel_x += amount,
            Direction::South => self.waypoint.rel_y -= amount,
            Direction::West => self.waypoint.rel_x -= amount,
        }
    }

    fn rotate_waypoint(&mut self, turn: Turn, degrees: u16) {
        let theta = (std::f64::consts::PI / 2.0)
            * f64::from(degrees / 90)
            * if turn == Turn::Left { 1.0 } else { -1.0 };
        let (ship_x, ship_y) = (f64::from(self.ship.x), f64::from(self.ship.y));
        let (my_x, my_y) = (
            f64::from(self.waypoint.rel_x) + ship_x,
            f64::from(self.waypoint.rel_y) + ship_y,
        );
        self.waypoint.rel_x =
            (theta.cos() * (my_x - ship_x) - theta.sin() * (my_y - ship_y)).round() as i32;
        self.waypoint.rel_y =
            (theta.sin() * (my_x - ship_x) + theta.cos() * (my_y - ship_y)).round() as i32;
    }
}

impl Ship {
    fn turn(&mut self, turn: Turn, degrees: u16) {
        let turns = degrees / 90;
        for _ in 0..turns {
            match turn {
                Turn::Left => match self.facing {
                    Direction::North => self.facing = Direction::West,
                    Direction::East => self.facing = Direction::North,
                    Direction::South => self.facing = Direction::East,
                    Direction::West => self.facing = Direction::South,
                },
                Turn::Right => match self.facing {
                    Direction::North => self.facing = Direction::East,
                    Direction::East => self.facing = Direction::South,
                    Direction::South => self.facing = Direction::West,
                    Direction::West => self.facing = Direction::North,
                },
            }
        }
    }

    fn forward(&mut self, amount: u16) {
        match self.facing {
            Direction::North => self.y += i32::from(amount),
            Direction::East => self.x += i32::from(amount),
            Direction::South => self.y -= i32::from(amount),
            Direction::West => self.x -= i32::from(amount),
        }
    }
}

fn main() -> Result<()> {
    problem1()?;
    problem2()
}

fn problem1() -> Result<()> {
    let mut ship = Ship::default();
    read_lines("_12/input.txt")?.for_each(|line| {
        let (instr, value) = line.split_at(1);
        let value = value.parse::<u16>().unwrap();
        match instr {
            "N" => ship.y += i32::from(value),
            "S" => ship.y -= i32::from(value),
            "E" => ship.x += i32::from(value),
            "W" => ship.x -= i32::from(value),
            "L" => ship.turn(Turn::Left, value),
            "R" => ship.turn(Turn::Right, value),
            "F" => ship.forward(value),
            _ => unimplemented!(),
        }
    });
    println!("{}", ship.x.abs() + ship.y.abs());
    Ok(())
}

fn problem2() -> Result<()> {
    let mut world = World::default();
    read_lines("_12/input.txt")?.for_each(|line| {
        let (instr, value) = line.split_at(1);
        let value = value.parse::<u16>().unwrap();
        match instr {
            "N" => world.move_waypoint(Direction::North, value),
            "S" => world.move_waypoint(Direction::South, value),
            "E" => world.move_waypoint(Direction::East, value),
            "W" => world.move_waypoint(Direction::West, value),
            "L" => world.rotate_waypoint(Turn::Left, value),
            "R" => world.rotate_waypoint(Turn::Right, value),
            "F" => world.move_ship_toward_waypoint(value),
            _ => unimplemented!(),
        }
    });
    println!("{}", world.ship.x.abs() + world.ship.y.abs());
    Ok(())
}
