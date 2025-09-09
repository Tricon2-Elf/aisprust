# aisp

Some tools and server emu for some old game.

Network needs a redesign to allow some shared player context and global notify messages.

<br>
<br>

Note that this was mostly meant as a fun side project, also to gain some experience with and maybe learn rust.

## Crates

- **aisp**: general aisp library
- **packfileutil**: some utility for headlock data files.
- **aisp_hook**: game hooks and patches. sends console output to udp 127.0.0.1:9999.
- **aisp_launcher**: launches game with hook, outputs console from udp
- **aisp_packet_macros**: some macros that makes packets somewhat easier
- **aisp_packet**: network messages
- **fmt_c**: utility for c format compatability. ported from older c# code. does not support `.`/percision formats.

## Paths

Theres some fixed paths in this project that i havent bothered to make settable or change. For usage it should probably be changed

- aisp_server: set environment `ITEM_LIST` `crates/aisp_server/src/servers/msg_server.rs:64`

## Usage

if you need to disable game network encryption, the `aisp_launcher`` with`aisp_hook`` might be needed.
Otherwise you can start the game directly.

1. put `connection.txt` in game install directory
   this makes the game connect to ip address listed in the txt file.

2. start server.
   there is a parameter option for encryption to use with cargo `cargo run -- -e true`, otherwise you can run the binary directly

3. start game

   **if encrypted**: start game `ai sp@ce.exe "[GAME_INSTALL_DIR]/data" -d [DLL_PATH]`
   **otherwise** start game `aispace_launcher.exe "[GAME_INSTALL_DIR]"`

## Compile

if on linux and need to compile `aisp_hook` and `aisp_launcher` go into into crate directory and run the following. (needs [cargo-xwin](https://github.com/rust-cross/cargo-xwin) unless xwin is implemented directly in build.rs)

```
XWIN_ARCH=x86 cargo xwin build
```

for the rest, normal `cargo build` or `cargo run` should work.

## Hooks

- wine had issues with its WSAGetLastError, unsure if happens in real windows. hooked and returns zero/success. can have some side effects on closing server.
- hooks function that creates network stream and focres it to unencrypted tcp.
- hooks gets setting string from csv.
- hooks logs for vce. useful for debugging.
- hooks item base table to check data. seems like first is missing :/

the game has some (ai)std::stream pointers here and there where the logs are outputted to. example is the vce log hooks, or packet send/recv functions. could be nice to change those at one point.

## Credits

- `crates/aisp_hook/lib/detours` contains packaged [microsoft detours](https://github.com/microsoft/Detours) which is licensed under MIT.
