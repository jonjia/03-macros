use anyhow::{Ok, Result};

// my_vec! = my_vec! { 1, 2, 3 };
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ( $elem:expr; $n:expr ) => {
        std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),+ $(,)? ) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        // temp_vec
        <[_]>::into_vec(Box::new([$( $x ),*]))
    }};
}

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3];
    let v1 = vec![1, 2, 3];
    let empty_v: Vec<i32> = my_vec![];
    let empty_v1: Vec<i32> = vec![];

    let n_v = my_vec![1; 3];
    let n_v1 = vec![1; 3];

    let vv: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
        "7".parse()?,
    ];

    println!("Hello, world!, {:?}", v);
    println!("Hello, world!, {:?}", v1);
    println!("Hello, world!, {:?}", empty_v);
    println!("Hello, world!, {:?}", empty_v1);

    println!("Hello, world!, {:?}", n_v);
    println!("Hello, world!, {:?}", n_v1);

    println!("Hello, world!, {:?}", vv);

    Ok(())
}
