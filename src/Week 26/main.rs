fn main() {
    // println!("Hello, world!");

    let s = sum(10,20);
    let name = String::from("Hello");
    let v = vec![1,2,3];
    let a = 10;
    
    assert_eq!(a, 10);
    // let ans = sum(1,2);
    println!("{}, {}, {:?}, {}",s,name,v,a);
}

fn sum(a: u32, b:u32) -> u32 {
    return a+b;
}

