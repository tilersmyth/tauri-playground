# Tauri App Playground
Features/concepts that I might want to include in Tauri project

## main.rs logic
- use ARC to share port status between threads
- in watch fn we have an internal pointer that tracks status so we only need to lock the ARC when an update is required
- in reader fn, we have an internal pointer that holds the SerialPort struct
    - if the SerialPort struct is None we unlock the port status ARC to check if it holds a value
    - When checking the port status ARC if it is None, we sleep for a couple seconds to avoid locking out the ARC
    - we rely on port read/write to alert use to connectivity issues (`BrokenPipe` error) so we don't have to open the port connection continuously in the loop

