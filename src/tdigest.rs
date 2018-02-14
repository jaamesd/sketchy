#![feature(nll)]
#![feature(saturating_float_casts)]
#![feature(two_phase_borrows)]

use std;

#[derive(Debug, Clone, Default)]
pub struct Tdigest {
    delta: f64,
    mass: f64,
    nan_mass: f64,
    centroids: Vec<Centroid>,
    buffer: Vec<Centroid>,
}

impl Tdigest {
    pub fn new(delta: f64, buffer_size: usize) -> Tdigest {
        Tdigest {
            delta: delta,
            mass: 0.0,
            centroids: Vec::with_capacity(delta as usize),
            buffer: Vec::with_capacity(buffer_size),
            nan_mass: 0.0,
        }
    }

    pub fn scaling_function(quantile: f64, delta: f64) -> f64 {
        delta * (f64::asin(2.0 * quantile - 1.0) / std::f64::consts::PI + 0.5)
        // quantile * delta
    }

    pub fn inv_scaling_function(k_factor: f64, delta:f64) -> f64 {
        (f64::sin((k_factor / delta - 0.5) * std::f64::consts::PI) + 1.0) / 2.0
        // k_factor / delta
    }


    pub fn merge_sample(&mut self, value: f64) {
        self.buffer.push(Centroid{mass: 1.0, sum: value});
        if self.buffer.capacity() == self.buffer.len()
        {
            let original_buffer_capacity: usize = self.buffer.capacity();
            self.merge_centroids();
            self.buffer.shrink_to_fit();
            self.buffer.reserve_exact(original_buffer_capacity);
        }
    }

    pub fn merge_sample_buffer(&mut self, other_buffer: &mut Vec<f64>) {
        let original_buffer_capacity: usize = self.buffer.capacity();
        let mut other_centroids: Vec<Centroid> = other_buffer.drain(0..).map(|x| Centroid{mass: 1.0, sum: x}).collect();
        self.buffer.append(&mut other_centroids);

        if self.buffer.len() >= original_buffer_capacity
        {
            self.merge_centroids();
            self.buffer.shrink_to_fit();
            self.buffer.reserve_exact(original_buffer_capacity);
        }
    }

    pub fn merge_tdigest(&mut self, other_tdigest: &mut Tdigest) {
        let original_buffer_capacity: usize = self.buffer.capacity();

        self.buffer.append(&mut other_tdigest.buffer);
        self.buffer.append(&mut other_tdigest.centroids);

        self.merge_centroids();
        self.buffer.shrink_to_fit();
        self.buffer.reserve_exact(original_buffer_capacity);
    }

    pub fn merge_centroids(&mut self) {
        let mut new_mass = self.mass;
        let mut new_nan_mass = self.nan_mass;
        self.buffer.retain(|&x| {
            if x.sum.is_nan() {
                new_nan_mass += x.mass;
                false
            } else {
                new_mass += x.mass;
                true
            }
        });
        self.buffer.append(&mut self.centroids);
        self.buffer.sort_by(|a, b| {
            let a_val: f64 = a.sum/a.mass;
            let b_val: f64 = b.sum/b.mass;
            a_val.partial_cmp(&b_val).unwrap()
        });

        let delta = self.delta;
        let mut quantile = 0.0;
        let mut quantile_limit = 0.0;
        self.centroids = self.buffer.drain(0..).fold(Vec::new(), |mut acc, x| {

            let new_quantile = quantile + x.mass/new_mass;
            if new_quantile < quantile_limit {
                quantile = new_quantile;
                *(acc.last_mut().unwrap()) += x;
                acc
            } else {
                quantile = new_quantile;
                quantile_limit = Tdigest::inv_scaling_function(Tdigest::scaling_function(quantile, delta) + 1.0, delta);
                acc.push(x);
                acc
            }
        });

        self.mass = new_mass;
        self.nan_mass = new_nan_mass;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Centroid {
    mass : f64,
    sum : f64,
}

impl Centroid {
    pub fn new(mass: f64, sum:f64) -> Centroid {
        Centroid {
            mass: mass,
            sum: sum,
        }
    }
}

impl std::ops::AddAssign for Centroid {
    fn add_assign(&mut self, other: Centroid) {
        *self = *self + other
    }
}

impl std::ops::Add for Centroid {
    type Output = Centroid;

    fn add(self, other: Centroid) -> Centroid {
        Centroid {
            mass: self.mass + other.mass,
            sum: self.sum + other.sum,
        }
    }
}