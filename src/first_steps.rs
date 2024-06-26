use std::collections::{HashMap, HashSet};

fn main() {
    // Hello world
    println!("Hello World");

    // strings
    let my_string: &str = "Str";
    println!("Let: {my_string}");

    let my_string2: String = String::from("Hello World");
    println!("Let: {my_string2}");

    // numbers
    let mut my_number: i32 = 1;
    my_number = my_number + 1;
    println!("Let: {}", my_number - 10);

    //boolean
    let my_boolean: bool = false;
    println!("{my_boolean}");

    // conts
    const MY_CONST: &str = "hello";
    println!("{MY_CONST}");

    //control de flujo
    if true {
        println!("if")
    } else if true {
        println!("else if")
    } else {
        println!("else")
    }

    // lista
    let mut my_list: Vec<&str> = vec!["1", "2", "1", "asda", "new"];

    my_list.push("jeff");

    println!("{:?}", my_list);
    println!("{:#?}", my_list);
    println!("{}", my_list[0]);

    // sets
    let mut my_set: HashSet<&str> = vec!["1", "2", "1", "asda", "new"].into_iter().collect();
    my_set.insert("jeff");
    println!("{:?}", my_set);

    // Maps
    let mut my_map: HashMap<&str, i32> = vec![("jeff", 1)].into_iter().collect();
    my_map.insert("k", 1);
    println!("{:?}", my_map);

    // blucles
    for value in &my_list {
        println!("{}", value)
    }

    for (key, value) in &my_map {
        println!("{},{}", key, value)
    }

    let mut my_counter = 0;
    while my_counter < my_list.len() {
        println!("{} {}", my_counter, my_list[my_counter]);
        my_counter += 1
    }

    //fns
    my_fn();

    //strutures
    let my_struct = MyStruct::new("Jeff", 1);
    println!("{}, {}", my_struct.name, my_struct.age)
}

fn my_fn() {
    println!("my_fn");
}

struct MyStruct {
    name: String,
    age: i32,
}

impl MyStruct {
    fn new(name: &str, age: i32) -> MyStruct {
        return MyStruct {
            name: String::from(name),
            age,
        };
    }
}
