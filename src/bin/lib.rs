use std::collections::HashMap;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn display(self) -> T {
        self.x
    }
}

fn main() {
    let mut hash = HashMap::new();
    hash.insert(String::from("car"), String::from("Gwagon"));
    hash.insert(String::from("car1"), String::from("BMW"));

    println!("{:?}", hash);
    hash.remove(&String::from("car"));
    println!("{:?}", hash);

    hash.insert(String::from("car"), String::from("Gwagon"));
    println!("{:?}", hash.get(&String::from("car1")));
    let p = Point { x: 5, y: 5 };
    println!("{}", p.display());

    let largest_num1 = largest_data(5, 1);
    let largest_num2 = largest_data("t", "d");
    println!("{}", largest_num1);
    println!("{}", largest_num2);

    let user = User {
        name: String::from("ABCDEFG"),
        age: 25,
    };
    notify(&user);
    println!("{}", user.summrise());

    let ans;

    let str1 = String::from("smagfgfgll");
    {
        let str2 = String::from("longeraaatryuiop");
        ans = longest(&str1, &str2);
        println!("{}", ans);
    }
    // println!("{}", ans);  --> Err : `str2` does not live long enough
}

fn largest_data<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub trait Summary {
    fn summrise(&self) -> String {
        return String::from("Default");
    }
}

pub trait Car {
    fn cars(&self);
}

struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summrise(&self) -> String {
        return format!("This is {} and {} year old", self.name, self.age);
    }
}

impl Car for User {
    fn cars(&self) {
        println!("This is Car");
    }
}

// fn notify(item: &impl Summary) {
//     println!("notify : {}", item.summrise());
// }

fn notify<T: Summary + Car>(item: &T) {
    println!("notify : {}", item.summrise());
    item.cars();
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}
