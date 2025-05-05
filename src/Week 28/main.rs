/* 
// Using External Library for dates

use chrono::prelude::*;

fn main() {
    let utc = Local::now();
    println!("{}",utc);

} */

/* 
// USing Environment Variables

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("REDIS_ADDRESS");
    
    match var {
        Ok(str) => println!("{}", str),
        Err(e) => println!("Error while reading: {}", e)
    }
} */

/* 
// Generic and Traits Bounds
use std::ops::Add;

fn sum<T: Add<Output = T>>(a:T, b:T) -> T {
    return a+b;
}

fn main() {
    println!("{}", sum(1, 2));
}
*/

/* 

// Generic Over Structs

#[derive(Clone, Copy)]
struct Rect<T> {
    width: T,
    height: T
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self)-> T{
        return self.width * self.height;
    }
}
fn main() {
    let r = Rect {
        width: 10.0,
        height: 20.0
    };

    let r1 = Rect {
        width: 10,
        height: 20
    };

    print!("{}", r.area());
    print!("{}", r1.area());
}
    
*/