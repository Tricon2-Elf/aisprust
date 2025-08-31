# aisp

Some tools and server emu for some old game.

Network needs a redesign to allow some shared player context and global notify messages.
encryption needs to be finished and tested. (none of the available camellia libraries has access to state which is needed)

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

- `crates/aisp_launcher/main.rs:55`
- `crates/aisp_server/src/servers/msg_server.rs:62`

## Hooks

- wine had issues with its WSAGetLastError, unsure if happens in real windows. hooked and returns zero/success. can have some side effects on closing server.
- hooks function that creates network stream and focres it to unencrypted tcp.
- hooks gets setting string from csv.
- hooks logs for vce. useful for debugging.
- hooks item base table to check data. seems like first is missing :/

the game has some (ai)std::stream pointers here and there where the logs are outputted to. example is the vce log hooks, or packet send/recv functions. could be nice to change those at one point. 


## Compile


if on linux and need to compile  `aisp_hook` and `aisp_launcher` go into into crate directory and run the following. (needs [cargo-xwin](https://github.com/rust-cross/cargo-xwin) unless xwin is implemented directly in build.rs)
```
XWIN_ARCH=x86 cargo xwin build 
```

for the rest, normal `cargo build` or `cargo run` should work.

## Usage

NOTE: if camellia network encryption is implemented you might not need launcher

1. put `connection.txt` in game install directory
2. start server
3. start game `aispace_launcher.exe "[GAME_INSTALL_DIR]"`

## Credits

- `crates/aisp_hook/lib/detours` contains packaged microsoft detours which is licensed under MIT.
