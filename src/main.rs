fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    if let Some(number) = six {
        println!("Test {}", number);
    }
    println!("{:?}", six);
    println!("{:?}", none);
}

