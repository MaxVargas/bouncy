use nannou::prelude::*;
use libm::*;

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
        let r = random_range(10.0, 50.0);

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
        // https://ocw.mit.edu/courses/8-01sc-classical-mechanics-fall-2016/mit8_01scs22_chapter15.pdf
        // 
        let angle1 = self.movement_angle();
        let angle2 = particle.movement_angle();
        let v1 = self.speed();
        let v2 = particle.speed();
        let m1 = self.mass();
        let m2 = particle.mass();

        let contact_angle = 0.0;

        (self.vx, self.vy) = ugly(m1,m2,v1,v2,angle1,angle2,contact_angle);
        (particle.vx, particle.vy) = ugly(m2,m1,v2,v1,angle2,angle1,contact_angle);
    }
    //fn energy(&self) -> f32 {
    //    // factor of 2 is not important...
    //    let v2 = self.vx.pow(2) + self.vy.pow(2);
    //    let m = self.mass();
    //    m*v2
    //}
    //fn momentum(&self) -> Vec<f32> {
    //    let m = self.mass();
    //    let mv = vec![m * self.vx, m *self.vy];
    //    mv
    //}
    fn speed(&self) -> f32 {
        ((self.vx.pow(2) + self.vy.pow(2)) as f32).pow(0.5)
    }
    fn movement_angle(&self) -> f32 {
        atan2(self.x.into(), self.y.into()) as f32
    }
    fn mass(&self) -> f32 {
        self.radius.pow(2) / (100.0 as f32)
    }
}

fn ugly(
    m1: f32, m2: f32, 
    v1: f32, v2: f32, 
    angle1: f32, angle2: f32, 
    contact_angle: f32
) -> (f32, f32) {
    let fraction = (v1 * cosf(angle1 - contact_angle) * (m1 - m2) + 2.0 * m2 * v2 * cosf(angle2 - contact_angle));
    let a = cosf(contact_angle) * fraction;
    let b = v1 * sinf(angle1 - contact_angle) * cosf(contact_angle + PI/2.0);
    let vx = a+b;

    let c = sinf(contact_angle) * fraction;
    let d = v1 * sinf(angle1 - contact_angle) * sinf(contact_angle + PI/2.0);
    let vy = c+d;

    (vx, vy)
}

fn model(app: &App) -> Model { 
    let _window = app.new_window().view(view).build().unwrap();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();
    let mut particles = vec![];

    for _ in 0..7 {
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
    let n_particles = model.particles.len();
    let mut particles_copy = model.particles.clone();

    for i in 0..n_particles {
        for j in (i+1)..n_particles {
            if model.particles[i].collides(&particles_copy[j]) {
                model.particles[i].smash(&mut particles_copy[j])
            }
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for particle in &mut model.particles {
        // Keep a log of the time since starting
        let dx = random_range(-0.1, 0.1);
        let dy = random_range(-0.1, 0.1);

        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.vx += dx;
        particle.vy += dy;

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

