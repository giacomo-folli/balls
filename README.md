# Balls

This project is a simulation of bouncing balls created with the `nannou` crate. 

![demo gif](https://raw.githubusercontent.com/giacomo-folli/balls/master/output1.gif)

## Motivation
It's a simple project to to learn some basic physics and graphics concepts.

## Quick Start

1.  Make sure you have Rust installed. You can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2.  Clone the repository.
3.  Navigate to the project directory.
4.  Run the project using `cargo run`.

## Usage

#### Controls

- `R`: Add vertical velocity to all balls.
- `Down`: Decrease the radius of the balls.
- `Up`: Increase the radius of the balls.
- `Left`: Remove a ball.
- `Right`: Add a ball.

## Fixes

| Task | Status |
|:--|:---|
|Physics logic rewrite.| To do|
|Fix jitter when objects are close to each others.| To do|
|Add collision handling to top wall too.|Done ✅|

## Contributing
Every contribution is welcome!!

## Roadmap

| Task | Status |
|:--|:--|
|Add more advanced collision detection.| To do|
|Let shape change over time.| To do|
|Load a soundtrack and make the balls react to its frequencies.| To do|
|Allow user customization in a .yaml file (keep it simple).| To do|
|Add more rooms (balls room, sqares room, etc...)|To do|
