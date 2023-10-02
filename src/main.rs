use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("{}", line.unwrap())
        if line.unwrap() == "6" {
            Command::new("playerctl")
            .arg("--player=spotify")
            .arg("play-pause")
            .spawn()
            .expect("playerpause failed");
        }
    }
}
