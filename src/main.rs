pub mod vec;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    particles: Vec<Particle>,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

enum Dim {
    X,
    Y,
}

impl Particle {
    fn new(x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> Self {
        Particle {
            x: x, y: y,
            vx: vx, vy: vy,
            radius: radius,
        }
    }
    fn new_rand(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        let x = random_range(x_min, x_max);
        let y = random_range(y_min, y_max/2.0);
        let r = random_range(30.0, 60.0);

        Particle::new(
            x, y,
            0.0, 0.0,
            r,
        )
    }
    fn collides(&self, particle: &Particle) -> bool {
        let diff: (f32, f32) = (self.x - particle.x, self.y - particle.y);
        let dist: f32 = ((diff.0.pow(2) + diff.1.pow(2)) as f32).pow(0.5);

        if dist <= self.radius + particle.radius {
            return true;
        }
        false
    }
    fn smash(&mut self, particle: &mut Particle) {
        // finds scattering angles in two-dimensional (elastic) collision
        let m1 = self.mass();
        let m2 = particle.mass();

        let v1 = vec![self.vx, self.vy];
        let x1 = vec![self.x, self.y];
        let v2 = vec![particle.vx, particle.vy];
        let x2 = vec![particle.x, particle.y];

        let v1_new = eval(&v1,&v2,&x1,&x2,m1,m2);
        let v2_new = eval(&v2,&v1,&x2,&x1,m2,m1);

        self.vx = v1_new[0];
        self.vy = v1_new[1];
        particle.vx = v2_new[0];
        particle.vy = v2_new[1];

    }
    fn mass(&self) -> f32 {
        self.radius.pow(2) / (100.0 as f32)
    }
}

fn eval(
    v1: &Vec<f32>, v2: &Vec<f32>, 
    x1: &Vec<f32>, x2: &Vec<f32>, 
    m1: f32, m2: f32
) -> Vec<f32> {
    let a = (2.0 * m2) / (m1 + m2);
    let b = dot(&sub(&v1,&v2),&sub(&x1,&x2))/(norm(&sub(&x2,&x1)).pow(2));
    let c = scale(&sub(&x1,&x2), a*b);

    sub(v1, &c)
}

// TODO: clean this up by creating some vec2d type
// which implements addition, dot product, normalization, etc.
// or just import a crate.
fn add(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    let mut sum = vec![];
    let n = v1.len();
    for i in 0..n {
        sum.push(v1[i] + v2[i]);
    }
    sum
}
fn sub(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    let mut sum = vec![];
    let n = v1.len();
    for i in 0..n {
        sum.push(v1[i] - v2[i]);
    }
    sum
}
fn scale(v1: &Vec<f32>, s: f32) -> Vec<f32> {
    let mut v2 = vec![];
    let n = v1.len();
    for i in 0..n {
        v2.push(v1[i] * s);
    }
    v2
}
fn dot(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let n = v1.len();
    for i in 0..n {
        sum += v1[i] * v2[i];
    }
    sum
}
fn norm(v1: &Vec<f32>) -> f32 {
    dot(v1, v1).pow(0.5)
}

fn model(app: &App) -> Model { 
    let _window = app.new_window().view(view).build().unwrap();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();
    let mut particles = vec![];

    for _ in 0..10 {
        particles.push(Particle::new_rand(
            boundary.left(), boundary.right(),
            boundary.top(), boundary.bottom(),
        ));
    }

    Model { 
        _window: _window, 
        particles: particles
    }
}

fn bounce(app: &App, particle: &mut Particle, dim: Dim) {
    let boundary = app.window_rect();
    let bx = (boundary.left() + boundary.right())/2.0;
    let by = (boundary.top() + boundary.bottom())/2.0;

    match dim {
        Dim::X => if (particle.vx) * (particle.x - bx) > 0.0 {
            particle.vx = -1.0 * particle.vx;
        }
        Dim::Y => if (particle.vy) * (particle.y - by) > 0.0 {
            particle.vy = -1.0 * particle.vy;
        }
    }
}

fn in_bounds(app: &App, particle: &Particle) -> Option<Dim> {
    let boundary = app.window_rect();
    if (particle.x < boundary.left()) || (particle.x > boundary.right()) {
        return Some(Dim::X);
    }
    if (particle.y > boundary.top()) || (particle.y < boundary.bottom()) {
        return Some(Dim::Y);
    }
    None
}

fn collisions(model: &mut Model) {
    // this is kinda wacky and can be handled much better...
    // also assuming we dont have triple collisions...
    // TODO: learn to handle this with mutexes / locks...
    let n_particles = model.particles.len();
    let mut particles_copy = model.particles.clone();

    let mut collided: Vec<(usize, usize)> = vec![];

    for i in 0..n_particles {
        for j in (i+1)..n_particles {
            if model.particles[i].collides(&particles_copy[j]) {
                model.particles[i].smash(&mut particles_copy[j]);
                collided.push((i,j));
            }
        }
    }
    for pair in collided.into_iter() {
        model.particles[pair.1] = particles_copy[pair.1].clone();
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for particle in &mut model.particles {
        // Keep a log of the time since starting
        let dx = random_range(-0.5, 0.5);
        let dy = random_range(-0.5, 0.5);

        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.vx += dx;
        particle.vy += dy;

        if particle.vx > 30.0 {
            particle.vx = 30.0;
        } else if particle.vx < -30.0 {
            particle.vx = -30.0;
        }
        if particle.vy > 30.0 {
            particle.vy = 30.0;
        } else if particle.vy < -30.0 {
            particle.vy = -30.0;
        }

        match in_bounds(app,&particle) {
            Some(dim) => bounce(app, particle, dim),
            None      => continue,
        }
    }
    collisions(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse at the x/y coordinates
    for particle in &model.particles {
        draw.ellipse()
            .color(STEELBLUE)
            .x_y(particle.x, particle.y)
            .radius(particle.radius);
    }

    draw.to_frame(app, &frame).unwrap();
}

