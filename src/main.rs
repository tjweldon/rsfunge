fn main() {
    println!("Hello, world!");
}

mod funge {
    type Stack = Vec<u8>;

    enum Direction {
        North, South, East, West
    }

    trait Movable {
        fn go(&mut self, direction: Direction);
    }

    trait Readable {
        fn read(&self, at: Location) -> u8;
    }

    trait Writable {
        fn write(&mut self, value: u8, at: Location);
    }

    struct Space {
        points: Vec<Vec<u8>>,
    }

    impl Readable for Space {
        fn read(&self, at: Location) -> u8 {
            self[at.0][at.1]
        }
    }

    impl Writable for Space {
        fn write(&mut self, value: u8, at: Location) {
            self[at.0][at.1] = value;
        }
    }

    struct Location(i64, i64);

    impl Movable for Location {
        fn go(&mut self, direction: Direction) {
            let delta: Location = match direction {
                Direction::North => Location(0, -1),
                Direction::East => Location(-1, 0),
                Direction::South => Location(0, 1),
                Direction::West => Location(1, 0),
            };

            self.0 += delta.0;
            self.1 += delta.1;
        }
    }

    #[derive(debug)]
    struct Vm {
        space: Space,
        stack: Stack,
        location: Location,
        delta: Direction,
    }
}