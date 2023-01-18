use rand::Rng;
use std::io;
use std::io::Write;
use std::process::exit;

// Constants
const FLEE_CHANCE: u32 = 3;
const INV_SIZE: u32 = 10;
const INV_WIDTH: u32 = 5;
const EMPTY_STRING: String = String::new();
const HELP_COMMAND: &str =
    "inventory: access your inventory\nw, a, s, d: walk forwards, left, backwards or right";

// Classes
struct Player {
    health: i32,
    defence: u32,
    damage: u32,
    speed: u32,
    inventory: [String; INV_SIZE as usize],
    weapon: String,
    armor: String,
    position: (i32, i32), // x, y centered on 0, 0
}

struct Enemy {
    name: String, 
    health: u32,
    defence: u32,
    damage: u32,
    speed: u32,
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

    // Starting text
    clearscreen::clear().unwrap();
    println!("Welcome!");

    // Game loop
    loop {
        // Get input
        let input = get_input();

        // Process input
        match input.as_str() {
            "help" => println!("{}", HELP_COMMAND),
            "exit" => exit(0),
            "debug" => println!("Debug\n{}", player.health), // Temporary command. Remove later
            "stats" => stats(&mut player),
            "inventory" => open_inventory(&mut player),
            "w" | "a" | "s" | "d" => walk(&mut player, &input),
            _ => println!("Type \"help\" for a list of commands"),
        }
    }
}

// Functions
fn get_input() -> String {
    // Typing prompt
    print!(">");
    io::stdout().flush().expect("Flush failed");
    // Get input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Readline failed");
    let input = input.trim().to_lowercase();
    // Clear screen
    clearscreen::clear().unwrap();
    return input;
}

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
    // Definitely not the optimal way but whatever
    let enemy_chance = rand::thread_rng().gen_range(0..5);
    let mut enemy_found: [String; 5] = [EMPTY_STRING; 5];
    enemy_found[0] = "and encountered an enemy".to_string();
    // Update position
    match direction {
        "w" => {
            player.position = (player.position.0, player.position.1 + 1);
            println!("You walked forward {}", enemy_found[enemy_chance])
        }
        "a" => {
            player.position = (player.position.0 - 1, player.position.1);
            println!("You walked left {}", enemy_found[enemy_chance])
        }
        "s" => {
            player.position = (player.position.0, player.position.1 - 1);
            println!("You walked backwards {}", enemy_found[enemy_chance])
        }
        _ => {
            player.position = (player.position.0 + 1, player.position.1);
            println!("You walked right {}", enemy_found[enemy_chance])
        }
    }
    // Initalize fight with enemy
    if enemy_chance == 0 {
        enemy_encounter(player);
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

fn enemy_encounter(player: &mut Player) {
    loop {
        println!("1. Fight  2. Flee");
        let input = get_input();
        if input == "1" {
            fight(player, String::from("Slime"));
            break;
        }
        if input == "2" {
            let flee_success = rand::thread_rng().gen_range(0..FLEE_CHANCE) == 0;
            if flee_success {
                println!("You successfully fled the fight");
            }
        }
    }
}

fn fight(player: &mut Player, enemy_name: String) {
    println!("You're in a fight with a {}", enemy_name);
    let mut health: i32 = player.health as i32;
    let mut enemy = Enemy {
        name: enemy_name,
        health: 50,
        defence: 0,
        damage: 5,
        speed: 50,
    };

    loop {
        println!("1. Attack 2. Magic");
        println!("3. Item   4. Flee");
        let input = get_input();
    }
}