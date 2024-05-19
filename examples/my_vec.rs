use anyhow::{Ok, Result};
use macros::my_vec;

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
