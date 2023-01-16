use rand::Rng;
use std::io;
use std::io::Write;
use std::process::exit;

// Constants
const INV_SIZE: u32 = 10;
const INV_WIDTH: u32 = 5;
const EMPTY_STRING: String = String::new();
const HELP_COMMAND: &str =
    "inventory: access your inventory\nw, a, s, d: walk forwards, left, backwards or right";

// Classes
struct Player {
    health: u32,
    defence: u32,
    damage: u32,
    speed: u32,
    inventory: [String; INV_SIZE as usize],
    weapon: String,
    armor: String,
    position: (i32, i32), // x, y centered on 0, 0
}

fn main() {
    // Setup player
    let mut player = Player {
        health: 100,
        defence: 5,
        damage: 10,
        speed: 10,
        weapon: "".to_string(),
        armor: "".to_string(),
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
        io::stdin().read_line(&mut input).expect("Readline failed");
        let input = input.trim().to_lowercase();

        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        // Process input
        match input.as_str() {
            "help" => println!("{}", HELP_COMMAND),
            "exit" => exit(0),
            "debug" => println!("Debug\n"), // Temporary command. Remove later
            "stats" => stats(&mut player),
            "inventory" => open_inventory(&mut player),
            "w" | "a" | "s" | "d" => walk(&mut player, &input),
            _ => println!("Type \"help\" for a list of commands"),
        }

        // Nice little arrow typing prompt
        print!(">");
        io::stdout().flush().expect("Flush failed");
    }
}

// Functions
fn open_inventory(player: &mut Player) {
    let mut lines: u32 = INV_SIZE / INV_WIDTH;
    if INV_SIZE as f32 / INV_WIDTH as f32 % 1.0 != 0.0 {
        lines += 1;
    }

    // Display inventory items
    for i in 0..lines {
        let mut line = String::new();
        for j in 0..INV_WIDTH {
            let current_index: u32 = j + i * INV_WIDTH;
            if current_index < INV_SIZE {
                if INV_SIZE.to_string().len() > (current_index + 1).to_string().len() {
                    for _ in 0..(current_index + 1).to_string().len() {
                        line += "0";
                    }
                }
                line += &(current_index + 1).to_string();
                line += ". ";
                line += &player.inventory[(current_index) as usize];
            }
        }
        println!("{}", line);
    }
}

fn walk(player: &mut Player, direction: &str) {
    let enemy_change = rand::thread_rng().gen_range(0..10); // Use something like this later
    match direction {
        "w" => {
            player.position = (player.position.0, player.position.1 + 1);
            println!("You walked forward")
        }
        "a" => {
            player.position = (player.position.0 - 1, player.position.1);
            println!("You walked left")
        }
        "s" => {
            player.position = (player.position.0, player.position.1 - 1);
            println!("You walked backwards")
        }
        _ => {
            player.position = (player.position.0 + 1, player.position.1);
            println!("You walked right")
        }
    }
}

fn stats(player: &mut Player) {
    // What even is this
    let weapon = if player.weapon == "" {
        "None"
    } else {
        &player.weapon
    };
    let armor = if player.armor == "" {
        "None"
    } else {
        &player.armor
    };

    println!(
        "Weapon: {}, Armor: {}\nHealth: {}, Defence: {}\nDamage: {}, Speed: {}",
        weapon, armor, player.health, player.defence, player.damage, player.speed
    );
}
