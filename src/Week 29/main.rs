/* trait Shape {
    fn area(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.width * self.height
    }
}

fn main() {
    print!("{}\n",String::from("Start Learning"));
    let r = Rect {
        width: 10.0,
        height: 10.0
    };
    let res = get_area(r);
    print!("{}", res);
}

fn get_area<T: Shape>(s:T) -> f32 {
    return s.area();
} */

/* 
// run: cargo expand
fn main() {
    print!("{}\n", String::from("Hello World"));
}
*/

/* 
// Declarative Macros
macro_rules! say_hello {
    () => {
        println!("Hello World");
    };
}

fn main() {
    say_hello!();
}
*/

/* 
// Procedural Macro 

use std::fmt::{Debug, Formatter};
// #[derive(Debug)]
struct User {
    name: String,
    password: String,
    age: u8
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Password: {}, age: {}", self.name, self.password, self.age)
    }
}

fn main() {
    let u = User {
        name: String::from("Tanish"),
        password: String::from("Tanish"),
        age: 18,
    };

    print!("{:?}\n", u); //Debug
    // print!("{}\n", u); //Display


}
*/