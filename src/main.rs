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

static folder: &str = "gameSaves";

fn main() {
    // Create a folder that will hold game save data if one doesn't exists. DONE
    // no player account exists -> start a new game.
    // if a player account exists -> let user select saved game or start new. 
    // Allow 3 save states. 
    // If the save state is new then commit to the below statement
    make_dir(folder);

    println!("You are about to embark on a once in a life time journey.");
    println!("Let's Explore.");
    println!("What is your name Captain?");

    let mut player = String::new();

    io::stdin().read_line(&mut player).expect("Failed To read line.");
    let player = player.trim().parse().expect("Error parsing String");

    println!("Hello, Capt. {player}");

    save_game(player);
}

fn make_dir(folder_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(folder_path)?;
    Ok(())
}

//fn read_save(player: String) {
//    let new_path = format!("{player}.txt");
//    // fs::File::read("/gameSaves")
//}

fn save_game(username: String) {
    let new_path = format!("./{folder}/{username}.txt"); // ./ uses a relative path, so it is required for this code to not make a folder on the users local drive and store files there.
    println!("{}", new_path);
    let mut data_file = File::create(new_path).expect("Creation Failed.");

    data_file.write("Hello, World!".as_bytes()).expect("Write Failed.");

    println!("Game Saved!");
}