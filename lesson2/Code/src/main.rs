use std::vec::Vec;


#[derive(Debug)]
enum Gender {
    FEMALE,
    MALE
}



fn main() {
    let v = "Hi!";
    println!("{}", v);
    let mut v = "Hello";
    println!("{}", v);
    v = "World";
    println!("{}", v);
    let v = "Hi";


    // u8  0-2^8 -1 
    // u16 0-2^16 -1
    // i8 -2^7 to 2^7 -1
    let v:i32 = -2;
    println!("{}", v);
    // error
    // let v:u32 = -2;
    // println!("{}", v);
    let big_val = std::i8::MAX;
    println!("{}", big_val);

    let money = 1_000;
    println!("{}", money);
    let c = b'a';
    println!("{}", c);

    // + - * / % 
    println!("{}", 1 + 1);
    println!("{}", 1 - 1);
    println!("{}", 1 * 1);
    println!("{}", 1 / 1);
    println!("{}", 1 % 1);

    let big_val = std::f32::MAX;
    println!("{}", big_val);

    let boolean = false;
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
    let boolean = true;
    let index = '*' as i32;
    println!("{}", index);
    let index = '8' as i32;
    println!("{}", index);

    println!();
    println!();

    let mut tuples = (1, "aa", false);
    println!("{}", tuples.0);
    println!("{}", tuples.1);
    println!("{}", tuples.2);
    tuples.0 = 2;


    let (a, b, c) = tuples;
    println!("a {}", a);
    println!("b {}", b);
    println!("c {}", c);

    println!();
    println!();

    let arr = [1,2,3,4];
    println!("{:?}", arr);
    println!("{}", arr[3]);
    println!("{}", arr.len());
    let mut arr:[i32; 4] = [1,2,3,4];
    arr[0] = 0;


    println!();
    println!();

    let mut vec = vec![1,2,3,4];
    println!("{:?}", vec);    
    vec.push(5);
    println!("{:?}", vec);    
    vec.pop();
    println!("{:?}", vec);

    let mut vec:Vec<i32> = (0..5).collect();
    println!("{:?}", vec);
    vec.pop();
    vec.remove(1);
    println!("{:?}", vec);
    println!("length {}", vec.len());
    println!("capacity {}", vec.capacity());

    println!();
    println!();

    let vec = vec![1,2,3,4];   
    let arr = [1,2,3,4];
    let rvec: &[i32] = &vec;
    let rarr: &[i32] = &arr;

    println!("{:?}", &vec[0..2]);


    println!("{:?}", Gender::MALE);
    println!("{:?}", Gender::FEMALE);

    //str
    let string = "aaa";
    //String
    let string = String::from("");

    let method = b"POST";
    println!("{:?}", method);
    // String
    let mut hello = "hello".to_string();
    hello.push('!');
    let ell0 = &hello[1..];
    // str
    let world = "world";


    let string = String::new();
    println!("{}", string);
    let mut string = String::from("hello");
    println!("{}", string);

    let emojy = "●﹏●";
    string.push_str(&emojy);
    string.extend("w o r l d".split_whitespace());
    println!("{}", string);
    println!("{}", emojy.len());
    println!("{}", emojy.chars().count());

    println!("{}", world.to_uppercase());
}
