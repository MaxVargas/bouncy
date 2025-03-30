use nannou::prelude::*;
use crate::vec::*;

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> Self {
        Particle {
            x: x, y: y,
            vx: vx, vy: vy,
            radius: radius,
        }
    }
    pub fn new_rand(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        let x = random_range(x_min, x_max);
        let y = random_range(y_min, y_max/2.0);
        let r = random_range(30.0, 60.0);

        Particle::new(
            x, y,
            0.0, 0.0,
            r,
        )
    }
    pub fn collides(&self, particle: &Particle) -> bool {
        let diff: (f32, f32) = (self.x - particle.x, self.y - particle.y);
        let dist: f32 = ((diff.0.pow(2) + diff.1.pow(2)) as f32).pow(0.5);

        if dist <= self.radius + particle.radius {
            return true;
        }
        false
    }
    pub fn smash(&mut self, particle: &mut Particle) {
        let m1 = self.mass();
        let m2 = particle.mass();

        let v1 = VecNd::<2>::from_vec(vec![self.vx, self.vy]);
        let x1 = VecNd::<2>::from_vec(vec![self.x, self.y]);
        let v2 = VecNd::<2>::from_vec(vec![particle.vx, particle.vy]);
        let x2 = VecNd::<2>::from_vec(vec![particle.x, particle.y]);

        let v1_new = elastic_collision(&v1,&v2,&x1,&x2,m1,m2);
        let v2_new = elastic_collision(&v2,&v1,&x2,&x1,m2,m1);

        self.vx = v1_new.data[0];
        self.vy = v1_new.data[1];
        particle.vx = v2_new.data[0];
        particle.vy = v2_new.data[1];

    }
    fn mass(&self) -> f32 {
        self.radius.pow(2) / (100.0 as f32)
    }
}

// two-dimensional (elastic) collision
fn elastic_collision <const N: usize> (
    v1: &VecNd<N>, v2: &VecNd<N>,
    x1: &VecNd<N>, x2: &VecNd<N>, 
    m1: f32, m2: f32
) -> VecNd<N> {
    let a = (2.0 * m2) / (m1 + m2);
    let b = (v1 - v2).dot(&(x1 - x2)) / ((x2 - x1).norm().pow(2));
    let c = (a*b) * &(x1 - x2);

    v1 - &c
}
