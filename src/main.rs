use nannou::{prelude::*, window::KeyPressedFn};

fn main() {
    nannou::app(model).update(update).run();
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PAD: f32 = 20.0;
const BOT_LIM: f32 = -(HEIGHT as f32 / 2.0) + PAD;
const TOP_LIM: f32 = (HEIGHT as f32 / 2.0) - PAD;
const LEF_LIM: f32 = -(WIDTH as f32 / 2.0) + PAD;
const RIG_LIM: f32 = (WIDTH as f32 / 2.0) - PAD;

const NO_OF_POINTS: usize = 30;
// const PO_RAD: f32 = 25.0;

#[derive(Clone)]
struct Point {
    position: Vec2,
    vel_x: f32,
    vel_y: f32,
    col: Rgb8,
    resting: bool,
}

struct Model {
    points: Vec<Point>,
    rad: f32,
}

impl Point {
    pub fn new(p: Vec2) -> Self {
        Point {
            position: p,
            vel_x: [10.0, -10.0][random_range(0, 1)],
            vel_y: random_range(-1.5, 1.5),
            col: rgb(
                random_range(0u8, 255u8),
                random_range(0u8, 255u8),
                random_range(0u8, 255u8),
            ),
            resting: false,
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let mut points: Vec<Point> = Vec::new();

    for _ in 0..NO_OF_POINTS {
        points.push(Point::new(vec2(
            random_range(LEF_LIM + 50.0, RIG_LIM - 50.0),
            0.0,
        )));
    }

    Model { points, rad: 25.0 }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            for point in model.points.iter_mut() {
                point.vel_y += 20.0;
                point.vel_x += random_range(-20.0, 20.0);
            }
        }
        Key::Down => {
            if model.rad > 3.0 {
                model.rad -= 1.0;
            }
        }
        Key::Up => {
            if model.rad < 40.0 {
                model.rad += 1.0;
            }
        }
        _other_key => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let grav: f32 = -1.2;
    let e_loss = 0.9;

    let other_points = model.points.clone();
    for point in model.points.iter_mut() {
        let mut y = point.position.y;
        let mut x = point.position.x;

        // if point.position.y <= BOT_LIM + model.rad + 0.1
        //     && point.vel_y.abs() < 0.15
        //     && point.vel_x.abs() < 0.15
        // {
        //     point.resting = true;
        //     point.vel_x = 0.0;
        //     point.vel_y = 0.0;
        //     continue;
        // } else {
        //     point.resting = false;
        // }

        // START

        let near_floor = point.position.y <= BOT_LIM + model.rad + 0.1;
        let slow = point.vel_y.abs() < 0.1 && point.vel_x.abs() < 0.1;
        let mut supported = near_floor;

        for other in other_points.iter() {
            if other.position == point.position {
                continue;
            }

            let vertical_gap = other.position.y - point.position.y;
            let horizontal_dist = (other.position.x - point.position.x).abs();

            if vertical_gap > 0.0
                && vertical_gap < model.rad * 2.2
                && horizontal_dist < model.rad * 1.5
            {
                if other.resting {
                    supported = true;
                    break;
                }
            }
        }

        if slow && supported {
            point.resting = true;
            point.vel_x = 0.0;
            point.vel_y = 0.0;
            continue;
        } else {
            point.resting = false;
        }

        // END

        y += point.vel_y;
        x += point.vel_x;

        if y <= BOT_LIM + model.rad {
            y = BOT_LIM + model.rad;
            point.vel_y = -point.vel_y * e_loss;
            point.vel_x *= e_loss;
        }

        if x <= LEF_LIM + model.rad {
            x = LEF_LIM + model.rad;
            point.vel_x = -point.vel_x * e_loss;
        } else if x >= RIG_LIM - model.rad {
            x = RIG_LIM - model.rad;
            point.vel_x = -point.vel_x * e_loss;
        }

        point.vel_y += grav;
        if point.vel_x.abs() < 0.05 {
            point.vel_x = 0.0;
        }
        if point.vel_y.abs() < 0.05 {
            point.vel_y = 0.0;
        }

        if y <= BOT_LIM + model.rad + 0.1 {
            point.vel_x *= 0.9;
        }

        point.position = vec2(x, y);
    }

    handle_collisions(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect().pad(PAD);
    draw.rect().wh(win.wh()).color(BLACK);

    draw.background().color(PLUM);

    for point in model.points.iter() {
        draw.ellipse()
            .xy(point.position)
            .radius(model.rad)
            .color(point.col);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn handle_collisions(model: &mut Model) {
    let e_loss = 0.98;

    for i in 0..model.points.len() {
        let (left, right) = model.points.split_at_mut(i + 1);
        let a = &mut left[i];

        for b in right {
            let delta = b.position - a.position;
            let distance = delta.length();

            if distance < model.rad * 2.0 {
                if distance > model.rad * 2.0 - 0.05 {
                    continue; // skip near-threshold overlap
                }

                let normal = delta.normalize_or_zero();

                let a_vel = vec2(a.vel_x, a.vel_y);
                let b_vel = vec2(b.vel_x, b.vel_y);
                let relative_velocity = b_vel - a_vel;
                let vel_along_normal = relative_velocity.dot(normal);

                if vel_along_normal > 0.0 {
                    continue;
                }

                if a.resting && (a.vel_x.abs() + a.vel_y.abs()) > 0.1 {
                    a.resting = false;
                }
                if b.resting && (b.vel_x.abs() + b.vel_y.abs()) > 0.1 {
                    b.resting = false;
                }

                let impulse = -vel_along_normal * e_loss;
                let impulse_vec = normal * impulse;

                a.vel_x -= impulse_vec.x * 0.5;
                a.vel_y -= impulse_vec.y * 0.5;
                b.vel_x += impulse_vec.x * 0.5;
                b.vel_y += impulse_vec.y * 0.5;

                if distance < model.rad * 2.0 - 0.01 {
                    let overlap = model.rad * 2.0 - distance;
                    let correction = normal * (overlap / 2.0);

                    a.position -= correction;
                    b.position += correction;

                    a.position.x = a.position.x.clamp(LEF_LIM + model.rad, RIG_LIM - model.rad);
                    a.position.y = a.position.y.clamp(BOT_LIM + model.rad, TOP_LIM - model.rad);
                    b.position.x = b.position.x.clamp(LEF_LIM + model.rad, RIG_LIM - model.rad);
                    b.position.y = b.position.y.clamp(BOT_LIM + model.rad, TOP_LIM - model.rad);
                }
            }
        }
    }
}
