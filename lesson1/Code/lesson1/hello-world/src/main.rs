#[derive(Debug)]
struct User {
    username: String
}

fn main() {
    // Display Trait
    println!("Hello {}", "yuequan");
    println!("{:o}", 9);
    println!("{:x}", 11);
    println!("{:X}", 11);
    let str1 = "yuequan";
    println!("{:p}", &str1);
    println!("{:b}", 2);
    println!("{:?}", str1);
    let user = User {
        username: String::from("yuequan")
    };
    println!("{:#?}", user);

    println!("{username}", username=&user.username);
    println!("{0}, {0}", user.username);
    println!("{:>8} {0}", user.username);
}
