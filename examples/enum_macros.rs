use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<i32> = 42.into();
    println!("{:?}", up);
    println!("{:?}", left);
    // construct a Direction
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// impl<T> From<i32> for Direction<T> {
//     fn from(e: i32) -> Self {
//         Direction::Left(e as u32)
//     }
// }

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(e: DirectionUp<T>) -> Self {
//         Direction::Up(e)
//     }
// }

// impl From<DirectionUp> for Direction {
//     fn from(v: DirectionUp) -> Self {
//         Direction::Up(v)
//     }
// }
