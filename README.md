# UAD Clone

***Abandoned and useless in this state***

A rust based GUI tool to debloat android devices using ADB.

## Dependencies
- adb installed and in PATH
- rust and cargo if you are building from source (mandatory for now because there's no release yet)

## Building from source
```bash
git clone https://github.com/fkgmdev/uadclone.git
cd uadclone
cargo build --release
```
## Usage
First, pair and connect your Android device via usb or wireless debugging.
Then, run
```bash
./target/release/uadclone
```
