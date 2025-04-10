# Balls

This project is a simulation of bouncing balls created with the `nannou` crate. It's a simple project to to learn some basic physics and graphics concepts.

Here's a quick look at the simulation in action:
![demo gif](https://raw.githubusercontent.com/giacomo-folli/balls/master/output1.gif)

## How to Run

1.  Make sure you have Rust installed. You can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2.  Clone the repository.
3.  Navigate to the project directory.
4.  Run the project using `cargo run`.

## Controls

- `R`: Add vertical velocity to all balls.
- `Down`: Decrease the radius of the balls.
- `Up`: Increase the radius of the balls.
- `Left`: Remove a ball.
- `Right`: Add a ball.

## Fixes

- Physics logic rewrite
- Jitter in balls when they are close to each other
- Add collision handling to top wall too

## New Stuff

- Add more advanced collision detection.
- Let shape change over time.
- Load a soundtrack and make the balls react to its frequencies.
- Allow user customization in a .yaml file (keep it simple).
- Add more rooms (balls room, sqares room, etc...)
