struct Cat {
    name: String,
    age: i64
}

impl Cat {
    fn say(&self){
        println!("我的名字: {}, 我的年龄: {}", self.name, self.age);
    }
}

struct RGBA(i32, i32, i32, f32);

impl RGBA {
    fn print_color(&self){
        println!("Red {} Green {} Blue {} Alpha {}", self.0, self.1, self.2, self.3);
    }
}


// Unit Struct
struct UnitStruct;


trait Hash {
    fn to_hash(&self) -> u64;
}


impl Hash for bool {
    fn to_hash(&self) -> u64{
        if *self{
            1
        }else{
            0
        }
    }
}

impl Hash for i64{
    fn to_hash(self: &Self) -> u64 {
        *self as u64
    }
}

fn print_to_hash<T>(t: &T) where T:Hash{
    println!("hash {}", t.to_hash());
}

fn print_to_hash_bool(t: &bool){
    println!("hash {}", t.to_hash());
}

fn print_to_hash_i64(t: &i64){
    println!("hash {}", t.to_hash());
}

trait Gun {
    fn shoot(&self);
    fn clear();
}

struct Gatlin;

impl Gun for Gatlin{
    fn shoot(&self){
        println!("da da da da da!!!!");
        Self::clear();
    }

    fn clear(){
        println!("cleaning.....");
    }
}


trait Base {

}

trait Child : Base {

}

struct Person {

}

impl Child for Person {

}

impl Base for Person {

}

trait HTTP{
    type Request;
    type Response;
}


fn main() {
    trait HTTP = Http<Request = http::Request, Response = http::Response>;
    let mut cat = Cat {
        name: String::from("MiMi"),
        age: 1
    };

    cat.name = String::from("Tom");

    cat.say();
    Cat::say(&cat);

    let color = RGBA(255,255,255, 0.1);
    color.print_color();
    RGBA::print_color(&color);


    print_to_hash(&true);
    // print_to_hash_bool(&true)
    print_to_hash(&false);
    // print_to_hash_bool(&false)
    print_to_hash(&64);
    // print_to_hash_i64(&64)

    let gatlin = Gatlin;
    gatlin.shoot();
    Gatlin::clear();
}
