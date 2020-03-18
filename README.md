# Rusty Racing: A Crash Course in Rust
### Companion repository to the OSCON 2020 full-day training


# Before the training...

Please do the following 10 minutes of preparation as soon as possible (before OSCON) so you will both have everything
downloaded AND have time to report any platform-specific issues so I can fix them! Please [contact me]
if you have trouble with any preparation steps.

I use macOS, and that is what I developed this course on.  Everything _ought_ to work similarly on major Linux
distributions and Windows.

[contact me]: mailto:nathan.stocks@gmail.com

## Install Rust

Rust 1.42.0 or newer is required for this course!  The latest stable version is recommended (1.42.0 at the time this was written).

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
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

- Clone or download this repository to the computer you will be using during OSCON. For example:

```shell
git clone git@github.com:cleancut/rusty_racing.git
```

- Run the example. This accomplishes two things: 1) All the cargo dependencies get downloaded (trust me, you don't want
  to have to wait for this on the conference Wi-Fi). 2) If there are any issues with your platform, [contact me] so I
  can fix things before you get to the conference.

```shell
cd rusty_racing
cargo run --example simple_racer
```

## Prepare to Learn

Please do the following (see the [How To Learn Rust](https://github.com/CleanCut/rust_programming/blob/master/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new message`
  - `cd message`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.

# OSCON!

Now you are ready for OSCON!

### Resources

- Live training by the instructor (Nathan Stocks)
- This Repository
- [How To Learn Rust](https://github.com/CleanCut/rusty_racing/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)
