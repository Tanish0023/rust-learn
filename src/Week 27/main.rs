/* fn main(){
    let mut str:String = String::from("Hello");
    str.push_str(" World!");
    let (s,str) = len(str);

    println!("{}", s);
    println!("{}", str);
}

fn len(s: String) -> (usize, String) {
    return (s.len(), s);
}*/


/*fn main(){
    let mut str:String = String::from("Hello");
    str.push_str(" World!");
    let s = len(&str);

    println!("{}", s);
    println!("{}", str);
}

fn len(s: &String) -> usize {
    return s.len();
} */

/* fn main(){
    let mut s1:String = String::from("Hello");
    s1.push_str(" World!");
    
    let s2 = &mut s1;
    s2.push_str(" from Tanish");

    // println!("{}", s1);
    println!("{}", s2);
} */

/* enum Direction {
    North,
    South,
    East,
    West
}

fn main() {
    let my_dir = Direction::West;

    steer(my_dir);
}

fn steer(dir: Direction) {
    match dir{
        Direction::North => println!("North"),
        Direction::East => println!("East"),
        Direction::South => println!("South"),
        Direction::West => println!("West"),
        _ => println!("Default")
    }
} */

/* use std::f64::consts::PI;

enum Shapes {
    Square(f64),
    Circle(f64),
    Rectangle(f64, f64)
}

fn main() {
    let sq = Shapes::Square(10.0);
    let cir = Shapes::Circle(2.0);
    let rec = Shapes::Rectangle(10.0, 20.0);

    println!("Square: {}\nCircle: {}\nRectangle: {}", area(sq), area(cir), area(rec));

}

fn area(shape:Shapes) -> f64{
    match shape {
        Shapes::Square(side) => return side * side,
        Shapes::Circle(radius) => return PI * (radius * radius),
        Shapes::Rectangle(len,br ) => return len * br
    }
}*/

/* use std::fs;

fn main() {
    let content = fs::read_to_string("./src/a.txt");

    match content {
        Ok(content) => println!("CONTENT: {}", content),
        Err(e) => println!("Error Reading the file: {}", e)
    }
} */