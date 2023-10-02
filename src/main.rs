use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let spotify_pause_pin = String::from("6");
    let default_pause_pin = String::from("7");
    let spotify_next_pin = String::from("8");
    let spotify_prev_pin = String::from("9");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if line.as_ref().unwrap() == &spotify_pause_pin {
            run_command("spotify", "play-pause");
        }
        else if line.as_ref().unwrap() == &default_pause_pin {
            run_command("", "play-pause");
        }
        else if line.as_ref().unwrap() == &spotify_next_pin {
            run_command("spotify", "next");
        }
        else if line.as_ref().unwrap() == &spotify_prev_pin {
            run_command("spotify", "previous");
        }
    }
}

fn run_command(player: &str, command: &str) {
    Command::new("playerctl")
    .arg(format!("--player={player}"))
    .arg(command)
    .spawn()
    .expect("player failed");
}