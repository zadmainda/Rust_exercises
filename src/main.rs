#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::structures::Employee;

mod closures;
mod forloop;
mod functions;
mod hof;
mod player;
mod slicers;
mod strings;
mod structures;
mod whileloop;
mod enums;
pub mod vectors;


fn main() {
    let last_name: &str = "Mainda";
    let salam = player::greeter::hello_name(last_name);
    // println!("{}", salam);
    // player::play_movie("Rambo: First Blood");
    // player::play_audio("Ariana Grande's: Girls");

    let mut array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice: &[i32] = &array[2..5];

    // println!("{:?}", slice);

    // STRING METHODS
    // let name_3 = "Zadok Mainda kinara".to_string();
    // println!("{}", name_3);

    // let mut name_3 = "Zadok Mainda kinara".to_string();
    // name_3 = name_3.replace("Zadok", "Example");

    // println!("{}", name_3);

    let mut name_3 = "Zadok Mainda kinara".to_string();
    name_3.push_str("Zzzzzz");

    // println!("{}", name_3);

    let name_4 = "Zadok Mainda kinara".to_string();

    // for word in name_4.split_whitespace() {
    //     println!("{}", word);
    // }

    let number: i32 = 10;
    // functions::add_one( number);

    slicers::use_slice(&mut array[1..5]);

    let mut empl_1: Employee = structures::Employee {
        name: String::from("John"),
        gender: String::from("Male"),
        age: 32,
        nums: [6, 7, 8, 9, 10, 11],
    };

    empl_1.age = 55;

    let empl_2: Employee = structures::Employee {
        name: String::from("Anna"),
        gender: String::from("Female"),
        age: 22,
        nums: [6, 7, 8, 9, 10, 11],
    };

    let  empl_3: Employee = structures::Employee {
        name: String::from("Tom"),
        gender: String::from("Female"),
        age: 33,
        nums: [1, 2, 3, 4, 5, 6],
    };

    // print!("{}",   empl_1.fn_details());
    // print!("{:?}", empl_2.fn_details());
    // print!("{:?}", empl_3.fn_details());


    structures::Employee::display_employee(empl_3);

    // forloop::loop_int();
    // forloop::loop_str();
    // forloop::loop_str_2();
    // forloop::loop_str_3();
    // forloop::loop_array();
    // forloop::loop_enum();
    // whileloop::get_squares(100);
    // whileloop::get_cubes(300);
    // println!("{}", functions::function(10));
    // closures::deputy()

    // High order function


    enums::categorize_bike(enums::Bicycles::Giant) ;

    let square = |a: i32| a * a;

    hof::apply(square, 6);


}
