#[allow(dead_code)]
pub fn loop_int() {
    for i in 1..5 {
        println!("{}", i);
    }
}

pub fn loop_str() {
    for i in { "Zadock" }.chars() {
        println!("{}", i);
    }
}

pub fn loop_str_2() {
    for i in "Mainda".chars() {
        println!("{}", i);
    }
}

pub fn loop_str_3() {
    let surname = String::from("Kinara");

    for i in surname.chars() {
        println!("{}", i);
    }
}

pub fn loop_array() {
    let pets: [&str; 5] = ["Cat", "Dog", "Cow", "Hen", "Rabbit"];
    for pet in pets.iter() {
        if pet == &{ "Cat" } {
            println!("I like meowing because I am { }", pet);
            continue;
        }
        else if pet == &{"Hen"} {
            println!("{}s are meant to be eaten", pet);
            break;
        }
        println!("{}", pet);
    }
}

pub fn loop_enum(){
  let students: [&str; 5] = ["zad", "Abel", "Tom", "Purity", "Diana"]; 

  for (index, i) in {1..5}.enumerate(){
    println!("{0}: {1} * {1} => {2}", index, i , {i * i});
  }
}