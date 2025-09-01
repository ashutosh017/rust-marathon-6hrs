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
}

fn even_filter(vec: &mut Vec<i32>) -> i32 {
    // for i in vec {
    // let mut new_vec = Vec::new();
    //     if i % 2 == 0 {
    //         new_vec.push(i);
    //     }
    // }
    // return new_vec;
    let mut i = 0;
    let mut it = 0;

    while i < vec.len() {
        it += 1;
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
    return it;
}
