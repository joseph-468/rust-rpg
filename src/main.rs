use std::io;
use std::io::Write;
use std::process::exit;

// Constants
const INV_SIZE: i32 = 10;
const INV_WIDTH: i32 = 5;
const EMPTY_STRING: String = String::new();
const HELP_COMMAND: &str = "inventory: access your inventory\nw, a, s, d: walk forwards, left, backwards or right";

// Classes
struct Player {
    health: u32,
    damage: u32,
    defence: u32,
    inventory: [String; INV_SIZE as usize],
    position: (i32, i32), // x, y centered on 0, 0
}

fn main() {
    // Setup player
    let mut player = Player {
        health: 100,
        damage: 10,
        defence: 5,
        inventory: [EMPTY_STRING; INV_SIZE as usize],
        position: (0, 0),
    };

    // Temporary menu
    print!("\x1B[2J\x1B[1;1H");
    print!("Welcome!\n>");
    io::stdout().flush().expect("Flush failed");

    // Game loop
    loop {
        // Get input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Readline failed"); 
        let input = input.trim().to_lowercase();
        
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        // Process input
        match input.as_str() {
            "help" => println!("{}", HELP_COMMAND),
            "exit" => exit(0),
            "debug" => println!("Debug"), // Temporary command. Remove later
            "inventory" => open_inventory(&mut player),
            "w" | "a" | "s" | "d" => walk(&mut player, &input),
            _ => println!("Type \"help\" for a list of commands"),
        }
        
        print!(">");
        io::stdout().flush().expect("Flush failed");
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

fn walk(player: &mut Player, direction: &str) {
    match direction {
        "w" => {player.position = (player.position.0, player.position.1 + 1); println!("You walked forward")},
        "a" => {player.position = (player.position.0 - 1, player.position.1); println!("You walked left")},
        "s" => {player.position = (player.position.0, player.position.1 - 1); println!("You walked backwards")},
        _ => {player.position = (player.position.0 + 1, player.position.1); println!("You walked right")},
    }
}
