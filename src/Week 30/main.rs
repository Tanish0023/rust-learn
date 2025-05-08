/*
// Serde - help in serializing and deserializing

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

struct User {
    username: String,
    password: String
}

fn main() {
    let u = User {
        username: String::from("Tanish"),
        password: String::from("Tanish")
    };

    let string_u = serde_json::to_string(&u);
    match string_u {
        Ok(str) => print!("{}\n", str),
        Err(e) => print!("Error: {}", e)
    }

    let s = String::from("{\"username\": \"ABC\", \"password\": \"123\"}");
    let s1: Result<User, serde_json::Error> = serde_json::from_str(&s);

    match s1 {
        Ok(user) => println!("Printing the string: {:?}", user),
        Err(e) => print!("Error: {}", e)
    }

}
*/

/*
// BORSH

use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]

struct User {
    username: String,
    password: String
}


fn main() {
    let u = User {
        username: String::from("Tanish"),
        password: String::from("Tanish"),
    };

    let mut v: Vec<u8> = Vec::new();
    // print!("{:?}\n",v);
    let ans = u.serialize(&mut v);
    // print!("{:?}\n",v);

    match ans {
        Ok(res) => print!("{:?}: {:?}\n", res,v),
        Err(e) => print!("Error During Serializing BORSH: {}",e)
    }

    let user = User::try_from_slice(&v).unwrap();
    print!("{}", user.username);
}
*/

/*
// LIFETIMES


fn main(){
    let s1 = String::from("Tanish");
    let ans;
    {
        let s2 = String::from("abc");
        ans = longest_string(&s1, &s2);
    }
    // ans will be a dangling pointer if s2>s1
    print!("{}", ans);
}

fn longest_string<'a, 'b>(s1:&'a String, s2:&'b String) -> &'a String {
    return s1;
}

*/
