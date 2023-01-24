use rand::Rng;
use std::clone::Clone;
use std::io;
use std::io::Write;
use std::process::exit;

// Constants
const FLEE_CHANCE: u32 = 3;
const ENCOUNTER_CHANCE: u32 = 3;
const INV_SIZE: u32 = 10;
const INV_WIDTH: u32 = 5;
const EMPTY_STRING: String = String::new();
const HELP_COMMAND: &str = "Full list of commands coming later";

// Structs
#[derive(Clone)]
struct Player {
    health: i32,
    defence: i32,
    damage: i32,
    speed: i32,
    inventory: [String; INV_SIZE as usize],
    weapon: String,
    armor: String,
    position: (i32, i32), // x, y centered on 0, 0
}

struct Enemy {
    name: String,
    health: i32,
    defence: i32,
    damage: i32,
    speed: i32,
}

fn main() {
    // Create player instance
    let mut player: Player = Player {
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
    println!("Welcome! Press any key to continue");

    // Game loop
    loop {
        // Get and process input
        let input: String = get_input();
        match input.as_str() {
            "help" => println!("{}", HELP_COMMAND),
            "debug" => println!("Debug\n{}, {}", player.health, player.weapon), // Temporary command. Remove later
            "stats" => stats(&mut player),
            "inventory" => open_inventory(&mut player),
            "w" | "a" | "s" | "d" => walk(&mut player, &input),
            _ => println!("Type \"help\" for a list of commands"),
        }
    }
}

// Basic input function
fn get_input() -> String {
    // Typing prompt
    print!(">");
    io::stdout().flush().expect("Flush failed");
    // Get input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Readline failed");
    let input: String = input.trim().to_lowercase();
    // Handle exit
    if input == "exit" {
        exit(0);
    }
    // Clear screen
    clearscreen::clear().unwrap();
    return input;
}

fn open_inventory(player: &mut Player) {
    // Calculate length and number of lines
    let mut lines: u32 = INV_SIZE / INV_WIDTH;
    if INV_SIZE as f32 / INV_WIDTH as f32 % 1.0 != 0.0 {
        lines += 1;
    }

    // Display inventory items
    for i in 0..lines {
        let mut line: String = String::new();
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

fn stats(player: &mut Player) {
    // Sets empty slots to None then print all stats and equipment to the screen
    let weapon: &str = if player.weapon == "" {
        "None"
    } else {
        &player.weapon
    };
    let armor: &str = if player.armor == "" {
        "None"
    } else {
        &player.armor
    };
    println!(
        "Weapon: {}, Armor: {}\nHealth: {}, Defence: {}\nDamage: {}, Speed: {}",
        weapon, armor, player.health, player.defence, player.damage, player.speed
    );
}

fn walk(player: &mut Player, direction: &str) {
    // Random chance to encounter enemy
    let enemy_found: bool = rand::thread_rng().gen_range(0..ENCOUNTER_CHANCE) == 0;
    let mut encounter_message: String = String::new();
    if enemy_found {
        encounter_message += "and encountered an enemy";
    }

    // Update position
    match direction {
        "w" => {
            player.position = (player.position.0, player.position.1 + 1);
            println!("You walked forward {}", encounter_message);
            encounter_message = String::from("You walked forward and encountered an enemy");
        }
        "a" => {
            player.position = (player.position.0 - 1, player.position.1);
            println!("You walked left {}", encounter_message);
            encounter_message = String::from("You walked left and encountered an enemy");
        }
        "s" => {
            player.position = (player.position.0, player.position.1 - 1);
            println!("You walked backwards {}", encounter_message);
            encounter_message = String::from("You walked backwards and encountered an enemy");
        }
        _ => {
            player.position = (player.position.0 + 1, player.position.1);
            println!("You walked right {}", encounter_message);
            encounter_message = String::from("You walked right and encountered an enemy");
        }
    }

    // Initalize fight with enemy if found
    if enemy_found {
        enemy_encounter(player, encounter_message);
    }
}

// Options that come up to start a fight then returns or calls fight function
fn enemy_encounter(player: &mut Player, encounter_message: String) {
    loop {
        println!("1. Fight  2. Flee");
        let input: String = get_input();
        if input == "1" {
            fight(player, String::from("Slime"));
            break;
        }
        if input == "2" {
            let flee_success: bool = flee(None);
            if flee_success {
                println!("You successfully fled the fight");
                break;
            } else {
                println!("You failed to flee the fight");
                get_input();
                fight(player, String::from("Slime"));
                break;
            }
        }
        clearscreen::clear().unwrap();
        println!("{}", encounter_message);
    }
}

fn fight(player_original: &mut Player, enemy_name: String) {
    // Create enemy and player instance and set turn to one with highest speed
    let mut player: Player = player_original.clone();
    let mut enemy: Enemy = Enemy {
        name: enemy_name,
        health: 50,
        defence: 0,
        damage: 15,
        speed: 50,
    };
    let mut player_turn: bool = if player.speed > enemy.speed {
        true
    } else {
        false
    };

    // Fight loop
    loop {
        // Check for deaths
        if enemy.health <= 0 {
            println!("Enemy died");
            break;
        }
        if player.health <= 0 {
            println!("You died");
            break;
        }

        // Player turn
        if player_turn {
            player_turn = false;
            loop {
                // Loop until valid input is entered then process input
                println!(
                    "Health: {}    {} Health: {}",
                    player.health, enemy.name, enemy.health
                );
                println!("1. Attack 2. Magic");
                println!("3. Item   4. Flee");
                let input: String = get_input();
                match input.as_str() {
                    "1" => {
                        attack(&mut player, &mut enemy, true);
                        get_input();
                        break;
                    }
                    "2" => {
                        println!("You don't know any magic dumbass");
                        get_input();
                        break;
                    }
                    "3" => {
                        println!("Inventory here");
                        get_input();
                        break;
                    }
                    "4" => {
                        if flee(None) {
                            println!("You successfully fled from {}!", enemy.name);
                            return;
                        } else {
                            println!("You failed to flee from {}", enemy.name);
                        }
                        get_input();
                        break;
                    }
                    _ => {}
                }
            }
        }
        // Enemy turn
        else {
            player_turn = true;
            attack(&mut player, &mut enemy, false);
            get_input();
        }
    }
}

// Used in fight to do regular damage to player and enemies
fn attack(player: &mut Player, enemy: &mut Enemy, player_attack: bool) {
    // Player attack
    if player_attack {
        let mut damage: i32 = player.damage - enemy.defence;
        if damage < 0 {
            damage = 0;
        }
        enemy.health -= damage;
        println!("You did {} damage", damage);
    }
    // Enemy attack
    else {
        let mut damage: i32 = enemy.damage - player.defence;
        if damage < 0 {
            damage = 0;
        }
        player.health -= damage;
        println!("Enemy did {} damage", damage);
    }
}

// Used in enemy encounter and fight
fn flee(chance_modifier: Option<u32>) -> bool {
    return rand::thread_rng().gen_range(0..FLEE_CHANCE * chance_modifier.unwrap_or(1)) == 0;
}
