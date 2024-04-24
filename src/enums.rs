#[derive(Debug)]

pub enum Bicycles {
    Scott,
    Giant,
    BMX,
    Mamba,
    Raleigh,
}

//Match Function

pub fn categorize_bike(bike: Bicycles) {
    match bike {
      Bicycles::Scott => {
            println!("Sporty road bike");
        },
        Bicycles::Giant => {
            println!("Sweet and rigid dual bike");
        },
        Bicycles::BMX => {
            println!("Urban streets shredder");
        },
        Bicycles::Mamba => {
            println!("For all your delivery needs");
        },
        Bicycles::Raleigh => {
            println!("Climbers and Descenders.");
        }
    }
}





