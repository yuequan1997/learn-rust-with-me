struct Counter {
    count: usize
}


impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}

trait InIterator<T: Copy> {
    fn each<F: Fn(T) -> T>(&mut self, f: F);
}

impl<T: Copy> InIterator<T> for Vec<T>{
    fn each<F: Fn(T) -> T>(&mut self, f: F){
        let mut i = 0;
        while i < self.len(){
            self[i] = f(self[i]);
            i += 1;
        }
    }
}


fn main() {
    let v = if false {
        1
    } else{
        2
    };

    println!("{}", v);



    let mut i = 0;

    // loop 
    // break 
    // continue
    loop {
        if i == 5 {
            i += 1;
            continue;
        }
        if i >= 10 {
            break;
        }
        i = i + 1;
    }
    println!("The loop end. i = {}", i);


    let mut i = 0;
    while true {
        if i >= 20 {
            break;
        }
        i+=1;
    }
    println!("The while end. i = {}", i);

    let v = vec![1,2,3,4];
    for element in &v{
        println!("{}", element);
    }
    println!("The for end.");

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());


    let mut v = vec![1,23,4,5,6,7];
    v.each(|i| {
        println!("{}", i);
        i * 3
    });
    println!("{:?}", v);
}
