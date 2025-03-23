use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    point: Point,
}

struct Point {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

enum Dim {
    X,
    Y,
}

fn model(app: &App) -> Model { 
    let _window = app.new_window().view(view).build().unwrap();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();
    let x0 = random_range(boundary.left(), boundary.right());
    let y0 = random_range(boundary.top(), boundary.bottom()/2.0);
    Model { 
        _window: _window, 
        point: Point{
            x: x0, y: y0,
            vx: 0.0, vy: 0.0,
        }
    }
}

fn bounce(app: &App, model: &mut Model, dim: Dim) {
    let boundary = app.window_rect();
    let bx = (boundary.left() + boundary.right())/2.0;
    let by = (boundary.top() + boundary.bottom())/2.0;

    match dim {
        Dim::X => if (model.point.vx) * (model.point.x - bx) > 0.0 {
            model.point.vx = -1.0 * model.point.vx;
        }
        Dim::Y => if (model.point.vy) * (model.point.y - by) > 0.0 {
            model.point.vy = -1.0 * model.point.vy;
        }
    }
}

fn in_bounds(app: &App, model: &Model) -> Option<Dim> {
    let boundary = app.window_rect();
    let Point{ x, y, vx: _, vy: _ } = model.point;
    if (x < boundary.left()) || (x > boundary.right()) {
        return Some(Dim::X);
    }
    if (y > boundary.top()) || (y < boundary.bottom()) {
        return Some(Dim::Y);
    }
    None
}

fn update(app: &App, model: &mut Model, _update: Update) {

    // Keep a log of the time since starting
    let dx = random_range(-0.1, 0.1);
    let dy = random_range(-0.1, 0.1);

    model.point.x += model.point.vx;
    model.point.y += model.point.vy;
    model.point.vx += dx;
    model.point.vy += dy;

    match in_bounds(app,model) {
        Some(dim) => bounce(app, model, dim),
        None      => return,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse at the x/y coordinates
    draw.ellipse().color(STEELBLUE).x_y(model.point.x, model.point.y);

    draw.to_frame(app, &frame).unwrap();
}

