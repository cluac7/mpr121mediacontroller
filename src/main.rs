use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let spotify_pin = String::from("6");
    let default_pin = String::from("8");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("{}", line.unwrap())
        if line.unwrap() == spotify_pin {
            Command::new("playerctl")
            .arg("--player=spotify")
            .arg("play-pause")
            .spawn()
            .expect("playerpause failed");
        }
    }
}
