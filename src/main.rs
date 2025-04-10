use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PAD: f32 = 20.0;
const BOT_LIM: f32 = -(HEIGHT as f32 / 2.0) + PAD;
const TOP_LIM: f32 = (HEIGHT as f32 / 2.0) - PAD;
const LEF_LIM: f32 = -(WIDTH as f32 / 2.0) + PAD;
const RIG_LIM: f32 = (WIDTH as f32 / 2.0) - PAD;
const NO_OF_POINTS: usize = 30;

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

fn main() {
    nannou::app(model).update(update).run();
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

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
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
        Key::Left => {
            if model.points.len() > 1 {
                model.points.pop();
            }
        }
        Key::Right => {
            if model.points.len() < 100 {
                model.points.push(Point {
                    position: vec2(0.0, 0.0),
                    vel_x: [10.0, -10.0][random_range(0, 1)],
                    vel_y: random_range(-1.5, 1.5),
                    col: rgb(
                        random_range(0u8, 255u8),
                        random_range(0u8, 255u8),
                        random_range(0u8, 255u8),
                    ),
                    resting: false,
                });
            }
        }
        _other_key => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    handle_wall_collisions(model);
    handle_balls_collisions(model);
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

fn handle_wall_collisions(model: &mut Model) {
    let grav: f32 = -1.2;
    let e_loss = 0.9;

    // let other_points = model.points.clone();
    for point in model.points.iter_mut() {
        let mut y = point.position.y;
        let mut x = point.position.x;

        y += point.vel_y;
        x += point.vel_x;

        if y <= BOT_LIM + model.rad {
            y = BOT_LIM + model.rad;
            point.vel_y = -point.vel_y * e_loss;
            point.vel_x *= e_loss;
        }

        if y >= TOP_LIM - model.rad {
            y = TOP_LIM - model.rad;
            point.vel_y = -point.vel_y * e_loss;
            // No energy loss when collision is with top border
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
}

fn handle_balls_collisions(model: &mut Model) {
    for _ in 0..3 {
        // Multiple iterations to ensure overlaps are fully resolved
        resolve_ball_penetrations(model);
    }

    // Then handle velocity changes from collisions
    resolve_ball_collisions(model);

    // Set up data for processing rest states
    let positions: Vec<Vec2> = model.points.iter().map(|p| p.position).collect();
    let mut resting: Vec<bool> = vec![false; model.points.len()];

    // Mark balls that are resting on the floor
    mark_floor_resting_balls(model, &mut resting);

    // Then iteratively find balls resting on other balls
    mark_stacked_resting_balls(model, &positions, &mut resting);
}

fn resolve_ball_penetrations(model: &mut Model) {
    for i in 0..model.points.len() {
        let (left, right) = model.points.split_at_mut(i + 1);
        let a = &mut left[i];

        for b in right {
            let delta = b.position - a.position;
            let distance = delta.length();

            if distance < model.rad * 2.0 - 0.001 {
                let normal = if distance > 0.001 {
                    delta.normalize()
                } else {
                    // If balls are exactly at the same position, move in random direction
                    let angle = random_range(0.0, std::f32::consts::PI * 2.0);
                    Vec2::new(angle.cos(), angle.sin())
                };

                // Calculate and apply position correction
                let overlap = model.rad * 2.0 - distance;
                let correction = normal * overlap;

                a.position -= correction * 0.5;
                b.position += correction * 0.5;

                a.position.x = a.position.x.clamp(LEF_LIM + model.rad, RIG_LIM - model.rad);
                a.position.y = a.position.y.clamp(BOT_LIM + model.rad, TOP_LIM - model.rad);
                b.position.x = b.position.x.clamp(LEF_LIM + model.rad, RIG_LIM - model.rad);
                b.position.y = b.position.y.clamp(BOT_LIM + model.rad, TOP_LIM - model.rad);
            }
        }
    }
}

fn resolve_ball_collisions(model: &mut Model) {
    let e_loss = 0.98;

    for i in 0..model.points.len() {
        let (left, right) = model.points.split_at_mut(i + 1);
        let a = &mut left[i];

        a.resting = false;

        for b in right {
            let delta = b.position - a.position;
            let distance = delta.length();

            if distance < model.rad * 2.0 + 0.1 {
                let normal = delta.normalize_or_zero();

                let a_vel = vec2(a.vel_x, a.vel_y);
                let b_vel = vec2(b.vel_x, b.vel_y);
                let relative_velocity = b_vel - a_vel;
                let vel_along_normal = relative_velocity.dot(normal);

                if vel_along_normal < 0.0 {
                    let impulse = -vel_along_normal * e_loss;
                    let impulse_vec = normal * impulse;

                    a.vel_x -= impulse_vec.x * 0.5;
                    a.vel_y -= impulse_vec.y * 0.5;
                    b.vel_x += impulse_vec.x * 0.5;
                    b.vel_y += impulse_vec.y * 0.5;
                }
            }
        }
    }
}

fn mark_floor_resting_balls(model: &mut Model, resting: &mut Vec<bool>) {
    for i in 0..model.points.len() {
        let point = &mut model.points[i];

        // A ball is resting on the floor if:
        // 1. It's very close to the floor
        // 2. It has very little vertical velocity
        // 3. It has very little horizontal velocity
        if point.position.y <= BOT_LIM + model.rad + 0.1
            && point.vel_y.abs() < 0.1
            && point.vel_x.abs() < 0.1
        {
            point.resting = true;
            resting[i] = true;
            point.vel_x = 0.0;
            point.vel_y = 0.0;
        }
    }
}

fn mark_stacked_resting_balls(model: &mut Model, positions: &Vec<Vec2>, resting: &mut Vec<bool>) {
    let mut changed = true;
    let mut pass_count = 0;
    const MAX_PASSES: usize = 5;

    while changed && pass_count < MAX_PASSES {
        changed = false;
        pass_count += 1;

        for i in 0..model.points.len() {
            if resting[i] {
                continue;
            }

            let point = &mut model.points[i];

            for j in 0..positions.len() {
                if i == j || !resting[j] {
                    continue;
                }

                let delta = point.position - positions[j];
                let distance = delta.length();

                if distance <= model.rad * 2.0 + 0.1 {
                    let angle = delta.y.atan2(delta.x);

                    // Check if the ball is approximately on top (within 30 degrees of vertical)
                    if angle > std::f32::consts::PI * 0.4
                        && angle < std::f32::consts::PI * 0.6
                        && point.vel_y.abs() < 0.2
                        && point.vel_x.abs() < 0.2
                    {
                        point.resting = true;
                        resting[i] = true;
                        point.vel_x *= 0.5;
                        point.vel_y = 0.0;
                        changed = true;
                        break;
                    }
                }
            }
        }
    }
}
