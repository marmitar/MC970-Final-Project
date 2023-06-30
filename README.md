# Conway's Game of Life: parallel implementation in Rust

This project implements [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) to observe the performance in parallel.

## How to run

To run simply execute:

```raw
$ cargo run [paralell|serial]
```

And the dependencies will be downloaded and the simulation ran in a window.

## Libraries

- [piston_window](https://docs.rs/piston_window/latest/piston_window/): Used to render UI for the game.
- [clap](https://github.com/clap-rs/clap): Used to parse CLI arguments for program.
- [rand](https://github.com/rust-random/rand): Used to generate random live cells.
- [rayon](https://github.com/rayon-rs/rayon): Used to paralellize.

## Team

- Tiago de Paula Alves
- Vitor Jundi Moriya
