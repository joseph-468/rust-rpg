use std::io;

// Constants
const INV_SIZE: i32 = 10;
const INV_WIDTH: i32 = 5;
const EMPTY_STRING: String = String::new();

// Classes
struct Player {
    health: u32,
    damage: u32,
    defence: u32,
    inventory: [String; INV_SIZE as usize],
}

fn main() {
    // Setup player
    let mut player = Player {
        health: 100,
        damage: 10,
        defence: 5,
        inventory: [EMPTY_STRING; INV_SIZE as usize],
    };

    // Game loop
    loop {
        // Get input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Readline failed"); 
        let input = input.trim().to_lowercase();
        
        // Process input
        match input.as_str() {
            "help" => println!("I am here to help."),
            "inventory" => open_inventory(&mut player),
            _ => println!("Command isn't recognised. Type \"help\" for a list of commands."),
        }
        
    }
}

// Functions
fn open_inventory(player: &mut Player) {
    let mut lines: i32 = INV_SIZE / INV_WIDTH;
    if INV_SIZE as f32 / INV_WIDTH as f32 % 1.0 != 0.0 {
        lines += 1;
    }

    for i in 0..lines {
        let mut line = String::new();
        for j in 0..INV_WIDTH {
            line += &(j+1).to_string();
            line += ". ";
            line += &player.inventory[j as usize];
        }
        println!("{}", line);
    }
}
