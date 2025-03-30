pub mod vec;
pub mod geom;
pub mod particles;

use crate::{
    vec::*,
    geom::*,
    particles::*,
};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    particles: Vec<Particle>,
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

