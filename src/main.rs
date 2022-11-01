#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

pub mod funge {
    use rand::{rngs::ThreadRng, Rng};
    pub mod code {
        use super::Direction;

        pub enum Instruction {
            // default
            ReadAndPush(usize), // read the value onto the stack

            // special instructions
            NoOp,       // no operation
            Stop,       // stop execution
            Skip,       // skip the next Instruction
            StringMode, // skip the next Instruction

            // Instruction Pointer Movement instructions
            Move(Direction),  // move in a specific direction
            MoveEastOrWest, // pop a value off the stack and move east if the value is 0, west otherwise
            MoveNorthOrSouth, // pop a value off the stack and move north if the value is 0, south otherwise
            MoveRandom,       // move in a random direction

            // Stack Manipulation instructions
            Duplicate, // duplicate the top value on the stack
            Swap,      // swap the top two values on the stack
            Pop,       // pop the top value off the stack

            // Arithmetic instructions
            Add, // pop a and b, push a + b
            Sub, // pop a and b, push a - b
            Mul, // pop a and b, push a * b
            Div, // pop a and b, push a / b
            Mod, // pop a and b, push a % b

            // Logical instructions
            GreaterThan, // pop a and b, push 1 if a > b, 0 otherwise
            Not,         // pop a push 1 if a == 0, 0 otherwise

            // I/O instructions
            PrintInt, // pop a, print a as an integer
            PrintChr, // pop a, print a as a character
            ReadInt,  // read an integer from stdin, push it
            ReadChr,  // read a character from stdin, push its ascii value

            // Put and Get instructions
            Put, // pop y, x and v, put v at (x, y)
            Get, // pop y and x, push the value at (x, y)
        }

        impl Instruction {
            pub fn from_raw(raw: u8, string_mode: &bool) -> Self {
                if *string_mode {
                    return match raw as char {
                        '"' => Self::StringMode,
                        x => Self::ReadAndPush(x as usize),
                    };
                }

                match raw as char {
                    ' ' => Self::NoOp,
                    '@' => Self::Stop,
                    '#' => Self::Skip,
                    '"' => Self::StringMode,
                    '^' => Self::Move(Direction::North),
                    '>' => Self::Move(Direction::East),
                    'v' => Self::Move(Direction::South),
                    '<' => Self::Move(Direction::West),
                    '_' => Self::MoveEastOrWest,
                    '|' => Self::MoveNorthOrSouth,
                    '?' => Self::MoveRandom,
                    ':' => Self::Duplicate,
                    '\\' => Self::Swap,
                    '$' => Self::Pop,
                    '+' => Self::Add,
                    '-' => Self::Sub,
                    '*' => Self::Mul,
                    '/' => Self::Div,
                    '%' => Self::Mod,
                    '`' => Self::GreaterThan,
                    '!' => Self::Not,
                    '.' => Self::PrintInt,
                    ',' => Self::PrintChr,
                    '&' => Self::ReadInt,
                    '~' => Self::ReadChr,
                    'p' => Self::Put,
                    'g' => Self::Get,
                    x if x as u8 >= '0' as u8 && x as u8 <= '9' as u8 => {
                        Self::ReadAndPush(x as usize)
                    }
                    _ => Self::ReadAndPush(raw as usize),
                }
            }
        }
    }

    pub mod ops {

        #[allow(dead_code)]
        pub struct NAry<'a, T: num_traits::PrimInt, const N: usize>(&'a dyn Fn([T; N]) -> T);

        impl<'a, T: num_traits::PrimInt, const N: usize> NAry<'a, T, N> {
            pub fn eval(&self, args: [T; N]) -> T {
                self.0(args)
            }

            pub fn new(op: &'a dyn Fn([T; N]) -> T) -> NAry<'a, T, N> {
                NAry(op)
            }

            // binary operator contstructors
            pub fn add() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_add)
            }
            pub fn sub() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_sub)
            }
            pub fn mul() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_times)
            }
            pub fn div() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_divide)
            }
            pub fn gt() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_gt)
            }
            pub fn rem() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_mod)
            }

            // unary operator contstructors
            pub fn not() -> NAry<'a, T, 1> {
                NAry::<'a, T, 1>::new(&_not)
            }
        }

        fn _add<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] + terms[1]
        }
        fn _sub<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] - terms[1]
        }
        fn _times<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] * terms[1]
        }
        fn _divide<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] / terms[1]
        }
        fn _mod<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] % terms[1]
        }
        fn _gt<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            match terms[0] > terms[1] {
                true => T::one(),
                false => T::zero(),
            }
        }
        fn _not<T: num_traits::PrimInt>(terms: [T; 1]) -> T {
            if !terms[0].is_zero() {
                T::zero()
            } else {
                T::one()
            }
        }
    }

    #[derive(Debug)]
    struct Stack<T: num_traits::PrimInt>(Vec<T>);

    impl<T: num_traits::PrimInt> Stack<T> {
        fn new() -> Stack<T> {
            Stack(Vec::<T>::new())
        }

        fn apply<const N: usize>(&mut self, op: ops::NAry<T, N>) -> () {
            let mut args: [T; N] = [T::zero(); N];
            for i in 0..N {
                args[i] = self.pop();
            }

            let result = op.eval(args);
            self.push(result);
            ()
        }

        fn dupe(&mut self) -> () {
            let item = self.pop();
            let duplicate = item;
            self.push(item);
            self.push(duplicate);
            ()
        }

        fn swap(&mut self) -> () {
            let item1 = self.pop();
            let item2 = self.pop();
            self.push(item2);
            self.push(item1);
            ()
        }
    }

    trait Lifo<T> {
        fn push(&mut self, item: T);
        fn pop(&mut self) -> T;
    }

    impl<T: num_traits::PrimInt> Lifo<T> for Stack<T> {
        fn push(&mut self, item: T) {
            self.0.push(item);
        }

        fn pop(&mut self) -> T {
            match self.0.pop() {
                Some(x) => x,
                _ => T::zero(),
            }
        }
    }

    #[derive(Debug)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    #[derive(Debug)]
    struct Space<T: num_traits::PrimInt> {
        points: Vec<Vec<T>>,
    }

    // Space trait implementations
    impl<T: num_traits::PrimInt> Space<T> {
        fn get(&self, at: &Location) -> T {
            self.points[at.0 as usize][at.1 as usize]
        }

        fn set(&mut self, value: T, at: Location) -> () {
            self.points[at.0 as usize][at.1 as usize] = value;
            ()
        }

        fn new(code: String) -> Space<T> {
            let mut max_len: usize = 0;
            let mut max_height: usize = 0;

            // establish size
            for line in code.lines() {
                if max_len < line.len() {
                    max_len = line.len();
                }
                max_height += 1;
            }

            // initialise the whitespace filled funge space
            let mut points: Vec<Vec<T>> =
                vec![vec![T::from(' ' as u8).unwrap(); max_len]; max_height];

            let mut y: usize = 0;

            // write the code to the funge space
            for line in code.lines() {
                let mut x: usize = 0;
                for chr in line.chars() {
                    points[y][x] = match T::from(chr as usize) {
                        Some(t) => t,
                        None => T::from(' ' as u8).unwrap(),
                    };
                    x += 1;
                }
                y += 1;
            }

            Space { points }
        }
    }

    #[derive(Debug)]
    struct Location(i64, i64);

    trait Movable {
        fn go(&mut self, direction: &Direction);
    }

    impl Movable for Location {
        fn go(&mut self, direction: &Direction) {
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

    #[derive(Debug)]
    struct Vm {
        space: Space<u8>,
        stack: Stack<u8>,
        location: Location,
        delta: Direction,
        string_mode: bool,
        stopped: bool,
        rng: ThreadRng,
    }

    impl Vm {
        pub fn new(code: String) -> Vm {
            Vm {
                space: Space::new(code),
                stack: Stack::new(),
                location: Location(0, 0),
                delta: Direction::West,
                string_mode: false,
                stopped: false,
                rng: rand::thread_rng(),
            }
        }

        pub fn consume(&mut self, instruction: code::Instruction) -> () {
            match instruction {
                code::Instruction::NoOp => (),
                code::Instruction::Stop => {
                    self.stopped = true;
                    ()
                }
                code::Instruction::Skip => {
                    self.location.go(&self.delta);
                    ()
                }
                code::Instruction::StringMode => {
                    self.string_mode = !self.string_mode;
                    ()
                }
                code::Instruction::Move(dir) => {
                    self.delta = dir;
                    ()
                }
                code::Instruction::MoveEastOrWest => {
                    self.delta = match self.stack.pop() {
                        0 => Direction::East,
                        _ => Direction::West,
                    };
                    ()
                }
                code::Instruction::MoveNorthOrSouth => {
                    self.delta = match self.stack.pop() {
                        0 => Direction::North,
                        _ => Direction::South,
                    };
                    ()
                }
                code::Instruction::MoveRandom => {
                    self.delta = match self.rng.gen_range(0..4) {
                        0 => Direction::North,
                        1 => Direction::East,
                        2 => Direction::South,
                        _ => Direction::West,
                    }
                }
                code::Instruction::Duplicate => self.stack.dupe(),
                code::Instruction::Swap => self.stack.swap(),
                code::Instruction::Pop => match self.stack.pop() {
                    _ => (),
                },
                code::Instruction::Add => self.stack.apply(ops::NAry::<u8, 2>::add()),
                code::Instruction::Sub => self.stack.apply(ops::NAry::<u8, 2>::sub()),
                code::Instruction::Mul => self.stack.apply(ops::NAry::<u8, 2>::mul()),
                code::Instruction::Div => self.stack.apply(ops::NAry::<u8, 2>::div()),
                code::Instruction::Mod => self.stack.apply(ops::NAry::<u8, 2>::rem()),
                code::Instruction::GreaterThan => self.stack.apply(ops::NAry::<u8, 2>::gt()),
                code::Instruction::Not => self.stack.apply(ops::NAry::<u8, 1>::not()),
                code::Instruction::PrintInt => {
                    print!("{}", self.stack.pop() as usize);
                    ()
                }
                code::Instruction::PrintChr => {
                    let mut output = String::from("");
                    output.push(self.stack.pop() as char);
                    print!("{}", output);
                    ()
                }
                code::Instruction::Put => {
                    let (y, x, v) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                    self.space.set(v, Location(x as i64, y as i64))
                }
                code::Instruction::Get => {
                    let (y, x) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(self.space.get(&Location(x as i64, y as i64)))
                }
                _ => {
                    let val = self.space.get(&self.location);
                    self.stack.push(val);
                    ()
                }
            }
        }
    }
}
