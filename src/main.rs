use nannou::prelude::*;

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
const PO_RAD: f32 = 25.0;

struct Point {
    position: Vec2,
    vel_x: f32,
    vel_y: f32,
    col: Rgb8,
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
        }
    }
}

struct Model {
    points: Vec<Point>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
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

    Model { points }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let grav: f32 = -1.2;
    let e_loss = 0.9;

    for point in model.points.iter_mut() {
        let mut y = point.position.y;
        let mut x = point.position.x;

        y += point.vel_y;
        x += point.vel_x;

        if y <= BOT_LIM + PO_RAD {
            y = BOT_LIM + PO_RAD;
            point.vel_y = -point.vel_y * e_loss;
            point.vel_x *= e_loss;
        }

        if x <= LEF_LIM + PO_RAD {
            x = LEF_LIM + PO_RAD;
            point.vel_x = -point.vel_x * e_loss;
        } else if x >= RIG_LIM - PO_RAD {
            x = RIG_LIM - PO_RAD;
            point.vel_x = -point.vel_x * e_loss;
        }

        point.vel_y += grav;
        if point.vel_y.abs() < 0.1 {
            point.vel_y = 0.0;
        }
        if point.vel_x.abs() < 0.05 {
            point.vel_x = 0.0;
        }

        if y <= BOT_LIM + PO_RAD + 0.1 {
            point.vel_x *= 0.9;
        }

        point.position = vec2(x, y);
    }

    handle_collisions(&mut model.points);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect().pad(PAD);
    draw.rect().wh(win.wh()).color(BLACK);

    draw.background().color(PLUM);

    for point in model.points.iter() {
        draw.ellipse()
            .xy(point.position)
            .radius(PO_RAD)
            .color(point.col);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn handle_collisions(points: &mut Vec<Point>) {
    let e_loss = 0.98;

    for i in 0..points.len() {
        let (left, right) = points.split_at_mut(i + 1);
        let a = &mut left[i];

        for b in right {
            let delta = b.position - a.position;
            let distance = delta.length();

            // let dsqrd = (b.position.y - a.position.y) * (b.position.y - a.position.y)
            //     + (b.position.x - a.position.x) * (b.position.x - a.position.x);

            // if dsqrd < pow(PO_RAD * 2.0, 2) {
            if distance < PO_RAD * 2.0 {
                let normal = delta.normalize_or_zero();

                let a_vel = vec2(a.vel_x, a.vel_y);
                let b_vel = vec2(b.vel_x, b.vel_y);
                let relative_velocity = b_vel - a_vel;
                let vel_along_normal = relative_velocity.dot(normal);

                if vel_along_normal > 0.0 {
                    continue;
                }

                let impulse = -vel_along_normal * e_loss;
                let impulse_vec = normal * impulse;

                a.vel_x -= impulse_vec.x * 0.5;
                a.vel_y -= impulse_vec.y * 0.5;
                b.vel_x += impulse_vec.x * 0.5;
                b.vel_y += impulse_vec.y * 0.5;

                if distance < PO_RAD * 2.0 - 0.1 {
                    // only then do position correction
                    let overlap = PO_RAD * 2.0 - distance;
                    let correction = normal * (overlap / 2.0);
                    a.position -= correction;
                    b.position += correction;
                }
            }
        }
    }
}
