use std::fs::read_to_string;

use chrono::Local;

fn main() {
    println!("{}", is_even(2));
    println!("{}", fib(4));
    let string = String::from("value");
    println!("{}", get_str_length("hello world".to_string()));
    println!("{}", get_str_length(string));
    let user = User {
        name: String::from("Ashutosh"),
        age: 21,
    };
    println!("{}", user.name);
    println!("{}", user.age);

    let rect = Rect {
        height: 12,
        width: 5,
    };
    println!("area is: {}", rect.area());
    println!("permiter is: {}", rect.perimeter());
    println!("{}", Rect::debug());

    let rect2 = Shape::Rectangle(1.0, 2.0);
    let circle = Shape::Circle(2.0);
    println!("Area of rectangle: {}", calculate_area(rect2));
    println!("Area of circle: {}", calculate_area(circle));
    let index = find_first_a("Preet".to_string());
    match index {
        Some(value) => println!("index is: {}", value),
        None => println!("index not found"),
    }
    let ans = read_from_file(String::from("a.txt"));
    match ans {
        Ok(data) => println!("{}", data),
        Err(err) => println!("{}", err),
    }

    let time = Local::now();
    println!("Current time is: {}", time);

    let mut s1 = String::from("Hello");
    // let s2 = do_somethng(&mut s1);
    println!("{}", s1);
    do_somethng(&mut s1);
    let s2 = &s1;
    println!("{}", s2);
}

fn do_somethng(s2: &mut String) -> &mut String {
    println!("printing inside fn: {}", s2);
    (*s2).push_str(" world");
    return s2;
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn fib(num: u32) -> u32 {
    // 0 1 1 2 3
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }
    let mut cnt: u32 = 0;
    for _ in 0..num - 1 {
        cnt = cnt + 1;
        let temp = second;
        second = second + first;
        first = temp;
    }

    println!("iterations: {}", cnt);

    return second;
}

fn get_str_length(str: String) -> usize {
    str.chars().count()
}

struct User {
    name: String,
    age: u32,
}

struct Rect {
    height: u32,
    width: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    fn debug() -> i32 {
        return 1;
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    return area;
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn read_from_file(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("File not read")),
    }
}
