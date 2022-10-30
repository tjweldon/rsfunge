#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

pub mod funge {
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

            pub fn add() -> NAry<'a, T, 2> {
                NAry::<'a, T, 2>::new(&_add)
            }
        }

        fn _add<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] + terms[1]
        }
        fn _sub<T: num_traits::PrimInt>(terms: [T; 2]) -> T {
            terms[0] - terms[1]
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

        fn apply<const N: usize>(&mut self, op: ops::NAry<T, N>) {
            let mut args: [T; N] = [T::zero(); N];
            for i in 0..N {
                args[i] = self.pop();
            }

            let result = op.eval(args);
            self.push(result)
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
    enum Direction {
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
        fn get(&self, at: Location) -> T {
            self.points[at.0 as usize][at.1 as usize]
        }

        fn set(&mut self, value: T, at: Location) {
            self.points[at.0 as usize][at.1 as usize] = value;
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
        fn go(&mut self, direction: Direction);
    }

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

    #[derive(Debug)]
    struct Vm {
        space: Space<u8>,
        stack: Stack<u8>,
        location: Location,
        delta: Direction,
    }

    impl Vm {
        fn new(code: String) -> Vm {
            Vm {
                space: Space::new(code),
                stack: Stack::new(),
                location: Location(0, 0),
                delta: Direction::West,
            }
        }
    }
}
