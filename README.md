# Rusty Racing: A Crash Course in Rust (for OSCON 2020)

If you would like to support this project, please [sponsor](https://github.com/sponsors/CleanCut) me. 💖

# Before the training...

Please take 10 minutes and follow these instructions as soon as possible (before OSCON). I want you to have everything
downloaded AND have time to report any platform-specific issues so I can fix them! Please [contact me]
if you have trouble with any preparation steps.

I use macOS, and that is what I developed this course on.  Everything _ought_ to work similarly on major Linux
distributions and Windows.

[contact me]: mailto:nathan.stocks@gmail.com

## Install Rust

Rust 1.42.0 or newer is required for this course!  The latest stable version is recommended (1.42.0 at the time this was written).

- [ ] Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
  button and follow the instructions to install Rust for your operating system.
  - Please DO NOT install rust via some other package manager.  It will probably be a version that is _too old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.42.0 (b8cedc004 2020-03-09)
$ cargo --version
cargo 1.42.0 (86334295e 2020-01-31)
```

## Run the Example

- [ ] Clone or download this repository to the computer you will be using during OSCON. For example:

```shell
git clone git@github.com:cleancut/rusty_racing.git
```

- [ ] Run the example. This accomplishes two things: 1) All the cargo dependencies get downloaded (trust me, you don't want
  to have to wait for this on the conference Wi-Fi). 2) If there are any issues with your platform, [contact me] so I
  can fix things before you get to the conference.

```shell
cd rusty_racing
cargo run --example simple_racer
```

## Configure your IDE/Editor

- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking

## Try out the editor

- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new hello_world`
  - `cd hello_world`
  - `cargo run`
  - Open `src/main.rs` in your IDE/editor
    - If you've configured things correctly, you ought to at least have syntax highlighting!
  - Back in the terminal, do `cargo run` again to see your new message.



- [ ] Check out the descriptions of the tools and books.

# End of Preparation!

Now you are ready for OSCON!

## Resources

- [How To Learn Rust](https://github.com/CleanCut/rusty_racing/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)
- [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/?referralCode=AF30FAD8C6CCCC2C94F0)
  on Udemy (my pre-recorded version of this training)

## Assets

- Race cars images are Public Domain and can be downloaded [here](https://looneybits.itch.io/2d-race-cars)
- Engine sound is Public Domain and can be downloaded [here](https://freesound.org/people/MarlonHJ/sounds/242740/)
- Tire squeaking sound is Public Domain and can be downloaded [here](https://freesound.org/people/RutgerMuller/sounds/104026/)
- Collision sound is under the Creative Commons Attribution license and can be downloaded [here](https://freesound.org/people/qubodup/sounds/147660/)

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).