# mpr121mediacontroller
Simple media controller using an arduino, mpr121 and rust 🤩

## Features
- [x] Pauses and plays Spotify music
- [x] Pauses and plays default player
- [x] Skips and goes back on Spotify

## Usage
1. Assign the pins on the mpr121 you want to use at the top of the main function in main.rs
```rust
let default_pin = String::from("X");    // Replace "X" with the pin number and do for all pins
```
2. Upload mediacentre.ino to the arduino
3. Figure out the serial port and baud rate. I did this by launching the serial monitor in the Arduino IDE as it is usually setup correctly and just checking there
4. Set those by running `stty [BAUD RATE] < /dev/[YOUR SERIAL PORT]` 
(Baud rate is generally 9600 and my serial port was /dev/ttyACM0)

5. Run `./mediacontroller < /dev/[YOUR SERIAL PORT]`

## NOTE
If your Arduino IDE when you try to upload code, says permission denied to /dev/[YOUR SERIAL PORT], type `ls -l /dev/[YOUR SERIAL PORT]`.
This will print something like
```
crw-rw---- 1 root uucp 166, 0 Oct  3 15:07 /dev/ttyACM0
```
Look for the word after root, in my case "uucp". This is the group you must add yourself to. To do that type:
```
sudo usermod -aG [NAME OF GROUP] [YOUR USERNAME]
```
so in my case it was:
```
sudo usermod -aG uucp rahul
```
Then log out and login again, or reboot. :D