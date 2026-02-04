# yt-dlgui - A Rust GUI Frontend for [yt-dlp](https://github.com/yt-dlp/yt-dlp)

## Building:
After installing [Rust](https://rustup.rs/), simply run in the root directory:

This icon branch is just a version of the last stable version of the code with an icon reference. In this branch it's required to have an icon in a resources folder, however you can change the name of the icon reference in the `src/main.rs` file (defaults to `resources/zenforic.ico`). This build will give the window and file an icon, as well as change its taskbar icon, via changes in `src/main.rs` and the additional `build.rs`.

`cargo build` for the debug version or:
`cargo build --release` for the release optimized version.

The executables will be found in the target/debug (or release) folders. They can also be run with `cargo run` or `cargo run --release`

## Usage

Simply run the built executables (or from the latest Release) or the cargo run commands above. Simple usage is just entering the URL to download from and clicking download (Enter on your keyboard should work as well). an Advanced section exists for some common arguments as well as a manual arguments entry near the bottom for anything not in the Advanced section.

## Other
[yt-dlp](https://github.com/yt-dlp/yt-dlp) itself is not made by me and all credit goes to the contributors of the original/linked repo.