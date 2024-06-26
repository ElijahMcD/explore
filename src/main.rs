// Create a cool command line game that will use the users decisions and take them around the solar system and hopefully they will learn something new
// The intro should give general information about traversing space within the game. Decisions should have a concequence if reckless
// The user will leave earth with a rocket that has enough rocket to get to a few inner planet.
// Planets should have their respective moons and if they are known to have water or methane then the player should be able to refuel on the planet to go on
// to other planets in the system. Game is over when the user leaves the solar system, gets back to earth after visiting all 8 planets thinks to visit pluto which will not be an explicit option.
// If they player runs out of fuel on a planet without a moon that has fuel or the planet itself has no fuel then the player will be lost at space and the game will also end. 
// How to begin well we need an intro screen that gets the players name.

use std::fs;
use std::io;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::path::Path;
use std::io::Result;
use std::thread;
use std::collections::HashMap;

static FOLDER: &str = "gameSaves";
// static ONE_SECOND = time::Duration::from_secs(1);

const SECS: u32 = 150;

// Hashmaps
let solar_distance = HashMap::from([
    ("Mercury", 1),
    ("Venus", 5),
    ("Earth", 7),
    ("Mars", 10),
    ("Jupiter", 16),
    ("Saturn", 20),
    ("Uranus", 25),
    ("Neptune", 29),
    ("Pluto", 36),
    ]);

let moon_parent = HashMap::from([
    ("Moon", "Earth"),
    ("Phobos", "Mars"),
    ("Deimos", "Mars"),
    ("Europa", "Jupiter"),
    ("Ganymede", "Jupiter"),
    ("Io", "Jupiter"),
    ("Calisto", "Jupiter"),
    ("Titan", "Saturn"),
    ("Lapetus", "Saturn"),
    ("Enceladus", "Saturn"),
    ("Hyperion", "Saturn"),
    ("Titania", "Uranus"),
    ("Miranda", "Uranus"),
    ("Oberon", "Uranus"),
    ("Ariel", "Uranus"),
    ("Triton", "Neptune"),
    ("Thalassa", "Neptune"),
])

let moon_distance = HashMap::from([
    ("Moon", 1),
    ("Phobos", 1),
    ("Deimos", 1),
    ("Europa", 1),
    ("Ganymede", 1),
    ("Io", 1),
    ("Calisto", 1),
    ("Titan", 1),
    ("Lapetus", 1),
    ("Enceladus", 1),
    ("Hyperion", 1),
    ("Titania", 1),
    ("Miranda", 1),
    ("Oberon", 1),
    ("Ariel", 1),
    ("Triton", 1),
    ("Thalassa", 1),
])

// need an set to keep track of all visited locations
// need function to append visited locations to the set



fn main() {
    let mute bodies_visited = 0;
    let mut fuel = 100;
    // Create a folder that will hold game save data if one doesn't exists. DONE
    // no player account exists -> start a new game. 
    // if a player account exists -> let user select saved game or start new. 
    // Allow 3 save states. 
    // If the save state is new then commit to the below statement
    make_dir(FOLDER);

    println!("What is your name Captain?");
    let egg = "isha";
    let mut player = String::new();
    
    io::stdin().read_line(&mut player).expect("Failed To read line.");
    let player: String = player.trim().parse().expect("Error parsing String");
    
    save_game(player.clone());
    // Name easter Egg. If player name is = Isha print out ascci art of "Hey Isha <3"
    if player.to_ascii_lowercase().contains(egg) {
      println!(r"__/\\\________/\\\_________________________________________/\\\\\\\\\\\_______________/\\\__________________________________        ");
      thread::sleep_ms(SECS);
      println!(r" _\/\\\_______\/\\\________________________________________\/////\\\///_______________\/\\\__________________________________       ");
      thread::sleep_ms(SECS);
      println!(r"  _\/\\\_______\/\\\________________________________________\/////\\\///_______________\/\\\__________________________________      ");
      thread::sleep_ms(SECS);
      println!(r"   _\/\\\\\\\\\\\\\\\_____/\\\\\\\\____\//\\\/\\\________________\/\\\______/\\\\\\\\\\_\/\\\__________/\\\\\\\\\______________     ");
      thread::sleep_ms(SECS);
      println!(r"    _\/\\\/////////\\\___/\\\/////\\\____\//\\\\\_________________\/\\\_____\/\\\//////__\/\\\\\\\\\\__\////////\\\_____________    ");
      thread::sleep_ms(SECS);
      println!(r"     _\/\\\_______\/\\\__/\\\\\\\\\\\______\//\\\__________________\/\\\_____\/\\\\\\\\\\_\/\\\/////\\\___/\\\\\\\\\\____________   ");
      thread::sleep_ms(SECS);
      println!(r"      _\/\\\_______\/\\\_\//\\///////____/\\_/\\\___________________\/\\\_____\////////\\\_\/\\\___\/\\\__/\\\/////\\\____________  ");
      thread::sleep_ms(SECS);
      println!(r"       _\/\\\_______\/\\\__\//\\\\\\\\\\_\//\\\\/_________________/\\\\\\\\\\\__/\\\\\\\\\\_\/\\\___\/\\\_\//\\\\\\\\/\\___________ ");
      thread::sleep_ms(SECS);
      println!(r"        _\///________\///____\//////////___\////__________________\///////////__\//////////__\///____\///___\////////\//____________");
      thread::sleep_ms(SECS);
    }
    
    println!("Welcome aboard, Capt. {player}");
    println!("You are about to embark on a once in a life time journey.");
    ask_confirm("But before we lift off, should we go over the mission?");
}

fn ask_confirm(question: &str) -> bool {
    println!("{}", question);
    println!("Y/n");
    loop {
        let mut input = [0];
        let _ = std::io::stdin().read(&mut input);
        match input[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("Y/n only."),
        }
    }
}

fn get_Destination() {
    // Ask user where they would like to go next,
    // Check to see if the Destination is a planet or a moon
    // if its a moon find its parent, 
    // call for the distance function to get the proper fuel consumption modifier
    
} 

fn get_fuel() {
    // checks fuel level and returns the value back to the player
}

fn harvest_fuel() {
    // Allows the player to harvest fuel from a planet or moon if the body is harvestable
}

fn add_location() {
    // adds location to the set of visited locations and increments the visited locations counter by 1
}

fn 

fn make_dir(folder_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(folder_path)?;
    Ok(())
}

//fn read_save(player: String) {
//    let new_path = format!("{player}.txt");
//    // fs::File::read("/gameSaves")
//}

fn save_game(username: String) {
    let new_path = format!("./{FOLDER}/{username}.txt"); // ./ uses a relative path, so it is required for this code to not make a folder on the users local drive and store files there.
    //println!("{}", new_path);
    let mut data_file = File::create(new_path).expect("Creation Failed.");

    data_file.write("Hello, World!".as_bytes()).expect("Write Failed.");

    //println!("Game Saved!");
}