# mpr121mediacontroller
Simple media controller using an arduino, mpr121 and rust 🤩

## Features
- Pauses and plays Spotify music
- [ ] Pauses and plays default player
- [ ] Skips and goes back on Spotify
- [ ] Skips and goes back on default player

## Usage
1. Assign the pins on the mpr121 you want to use at the top of the main function in main.rs
```rust
let default_pin = String::from("X");    // Replace "X" with the pin number and do for all pins
```
2. Upload mediacentre.ino to the arduino
3. Figure out the serial port and baud rate. I did this by launching the serial monitor in the Arduino IDE as it is usually setup correctly and just checking there
4. Set those by running 
```bash
stty [BAUD RATE] < /dev/[YOUR SERIAL PORT]
``` 
(Baud rate is generally 9600 and my serial port was /dev/ttyACM0)
5. Run `./mediacontroller < /dev/[YOUR SERIAL PORT]`