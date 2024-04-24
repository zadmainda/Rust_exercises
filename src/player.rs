pub fn play_movie(name: &str) {
    println!("You are Watching {}", name);
}

pub fn play_audio(song: &str) {
    println!("Playing {}", song);
}

pub mod greeter {
    pub fn hello_name(name: &str) -> String {
        let greeting = format!("Hello {}", name);
        return greeting;
    }
}
