use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();
    for i in 1..10 {
        vec.push(i);
    }
    println!("{:?}", vec);
    // println!("{:?}", even_filter(vec));
    let x = even_filter(&mut vec);
    println!("{}", x);
    println!("{:?}", vec);

    let vec2 = vec![1, 2, 3];
    println!("{:?}", vec2);

    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert(String::from("Hello"), String::from("World"));
    let val = hm.get("Hello");
    match val {
        Some(v) => println!("{}", v),
        None => println!("Nothing found"),
    }

    let inp_vec = vec![(String::from("a"), 1), (String::from("b"), 2)];
    let hm2 = group_values_by_keys(inp_vec);
    println!("{:?}", hm2);

    let nums = vec![1, 2, 3, 4, 5];
    let mut itr = nums.iter();
    let itr2 = nums.iter();
    let itr3 = nums.iter();
    let itr4 = nums.iter();
    let itr5 = nums.iter();

    for value in itr2 {
        print!("{} ", value);
    }
    print!("\n");
    while let Some(val) = itr.next() {
        print!("{} ", val);
    }
    print!("\n");

    let sum: i32 = itr3.sum();
    println!("{}", sum);

    let even_vals_itr = itr4.filter(|x| *x % 2 == 0);
    print!("Even values: ");
    for val in even_vals_itr {
        print!("{} ", val);
    }

    let double_even_vals_itr = itr5.filter(|x| *x % 2 == 0).map(|x| x * 2);
    print!("Double of even values: ");
    for val in double_even_vals_itr {
        print!("{} ", val);
    }
    print!("\n");

    let hello_world = String::from("Hello world");
    let first_word = get_first_word(&hello_world);
    println!("{} ", first_word);
    let user = User {
        name: String::from("Ashutosh"),
        age: 21,
    };
    println!("{}", user.summary());
    println!("{}, {}", user.name, user.age);
    notify(user);

    let longest_str;
    let s1 = String::from("Helloo");
    let s2 = String::from("world");
    longest_str = longest(&s1, &s2);
    println!("Longest string is: {}", longest_str);

    let name = String::from("lifetimes with structs");
    let user = User2{
        name:&name
    };
    println!("{}",user.name);
    let user = String::from("wtf");
    println!("{}",user);
}

struct  User2<'a>{
    name:&'a str
}


fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

fn notify<T: Summerize + FixTrait>(u: T) {
    println!("{}", u.summary());
    u.fix();
}
pub trait FixTrait {
    fn fix(&self) -> String {
        return String::from("Hi from fix");
    }
}
pub trait Summerize {
    fn summary(&self) -> String {
        return String::from("Hi there");
    }
}
struct User {
    name: String,
    age: i32,
}
impl Summerize for User {}
impl FixTrait for User {}

fn get_first_word(word: &String) -> &str {
    let mut index = 0;
    for (_, c) in word.chars().enumerate() {
        if c == ' ' {
            break;
        }
        index = index + 1;
    }
    return &word[0..index];
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut new_hm: HashMap<String, i32> = HashMap::new();
    for (key, value) in vec {
        new_hm.insert(key, value);
    }
    return new_hm;
}

fn even_filter(vec: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut it = 0;

    while i < vec.len() {
        it += 1;
        if vec[i] % 2 != 0 {
            vec.remove(i);
        }
        i += 1;
    }
    return it;
}
