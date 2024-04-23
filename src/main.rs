#[allow(unused_assignments)]
#[allow(dead_code)]
mod player;
mod structures;
mod forloop;



fn main() {
    let last_name: &str = "Mainda";
    let salam = player::greeter::hello_name(last_name);
    println!("{}", salam);
    player::play_movie("Rambo: First Blood");
    player::play_audio("Ariana Grande's: Girls");

    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice: &[i32] = &array[2..5];

    println!("{:?}", slice);

    let empl1 = structures::Employee{
        name: String::from("Zadock"),
        gender: String::from("Male"),
        age:  32,
        nums: [6, 7, 8, 9, 10, 11]

    };
    let empl2 = structures::Employee{
        name: String::from("Anna"),
        gender: String::from("Female"),
        age:  22,
        nums: [6, 7, 8, 9, 10, 11]

    };
    let empl3 = structures::Employee{
        name: String::from("Delvin"),
        gender: String::from("Tranny"),
        age:  33,
        nums: [1, 2, 3, 4, 5, 6]

    };


    // print!("{}", empl1.fn_details());
    // print!("{:?}", empl2.fn_details());
    // print!("{:?}", empl3.fn_details());

    // forloop::loop_int();
    // forloop::loop_str();
    // forloop::loop_str_2();
    // forloop::loop_str_3();
    forloop::loop_array()


}


