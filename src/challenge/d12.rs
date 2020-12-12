use crate::*;

pub struct Challenge12;

#[derive(Debug)]
enum Dir {
    North, East, South, West
}

impl Dir {
    fn rotate_left(&mut self) {
        *self = match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South
        }
    }
    
    fn rotate_right(&mut self) {
        *self = match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North
        }
    }
    
    fn rotate_left_by(&mut self, mut degrees: isize) {
        while degrees > 0 {
            self.rotate_left();
            degrees -= 90;
        }
    }
    
    fn rotate_right_by(&mut self, mut degrees: isize) {
        while degrees > 0 {
            self.rotate_right();
            degrees -= 90;
        }
    }
    
    fn to_char(&self) -> char {
        match self {
            Dir::North => 'N',
            Dir::East => 'E',
            Dir::South => 'S',
            Dir::West => 'W'
        }
    }
}

#[derive(Debug)]
struct Ship {
    north_south: isize,
    east_west: isize,
    dir: Dir
}

#[derive(Debug)]
struct Waypoint { //or "Vec2i", if you will
    north_south: isize,
    east_west: isize
}

impl Waypoint {
    fn rotate_left(&mut self) {
        std::mem::swap(&mut self.north_south, &mut self.east_west);
        self.east_west = -self.east_west;
    }
    
    fn rotate_right(&mut self) {
        std::mem::swap(&mut self.north_south, &mut self.east_west);
        self.north_south = -self.north_south;
    }
    
    fn rotate_left_by(&mut self, mut degrees: isize) {
        while degrees > 0 {
            self.rotate_left();
            degrees -= 90;
        }
    }
    
    fn rotate_right_by(&mut self, mut degrees: isize) {
        while degrees > 0 {
            self.rotate_right();
            degrees -= 90;
        }
    }
}

impl Ship {
    fn new() -> Ship {
        Ship { north_south: 0, east_west: 0, dir: Dir::East}
    }
    
    fn process_commands_1(&mut self, commands: String) {
        for cmd in commands.lines() {
            let mut chars = cmd.chars();
            
            let mut kind = chars.next().unwrap();
            let num = chars.collect::<String>().parse::<isize>().unwrap();
            
            //a bit cheeky
            if kind == 'F' {
                kind = self.dir.to_char()
            }
            
            match kind {
                'N' => self.north_south += num,
                'S' => self.north_south -= num,
                'E' => self.east_west += num,
                'W' => self.east_west -= num,
                'L' => self.dir.rotate_left_by(num),
                'R' => self.dir.rotate_right_by(num),
                _ => panic!()
            }
        }
    }
    
    fn process_commands_2(&mut self, commands: String) {
        let mut way = Waypoint {
            north_south: 1,
            east_west: 10,
        };
        
        for cmd in commands.lines() {
            let mut chars = cmd.chars();
            
            let kind = chars.next().unwrap();
            let num = chars.collect::<String>().parse::<isize>().unwrap();
            
            match kind {
                'N' => way.north_south += num,
                'S' => way.north_south -= num,
                'E' => way.east_west += num,
                'W' => way.east_west -= num,
                'L' => way.rotate_left_by(num),
                'R' => way.rotate_right_by(num),
                'F' => self.move_by(&way, num),
                _ => panic!()
            }
        }
    }
    
    fn move_by(&mut self, way: &Waypoint, count: isize) {
        self.north_south += way.north_south * count;
        self.east_west += way.east_west * count;
    }
    
    fn manhattan_dist_to_origin(&self) -> usize {
        (self.north_south.abs() + self.east_west.abs()) as usize
    }
}

impl Challenge for Challenge12 {
    fn filename(&self) -> &'static str {
        "12a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut ship = Ship::new();
        ship.process_commands_1(input);
        Ok(ship.manhattan_dist_to_origin().to_string())
    }

    //wrong: 13046
    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut ship = Ship::new();
        ship.process_commands_2(input);
        Ok(ship.manhattan_dist_to_origin().to_string())
    }
}