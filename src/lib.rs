use std::hash::{Hash, Hasher};
use std::ops::Deref;

use rand::Rng;
use raylib::prelude::*;

pub mod camera;

pub fn generate_new_means(means: &mut Vec<Vector3>, cluster_radius: f32) {
    let mut rng = rand::thread_rng();
    for i in 0..means.len() {
        means[i].x = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
        means[i].y = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
        means[i].z = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
    }
}

pub fn recluster_state(
    samples: &mut Vec<Vector3>,
    means: &mut Vec<Vector3>,
    clusters: &mut Vec<Vec<Vector3>>,
) {
    *clusters = vec![Vec::new(); means.len()];
    for p in samples.iter() {
        let mut k: i32 = -1;
        let mut s = f32::MAX;
        for i in 0..means.len() {
            let m = means[i];
            let x = p.clone() - m;
            let sm = x.length();
            if sm < s {
                s = sm;
                k = i as i32;
            }
        }
        clusters[k as usize].push(p.clone());
    }
}

pub fn update_means(
    means: &mut Vec<Vector3>,
    clusters: &mut Vec<Vec<Vector3>>,
    cluster_radius: f32,
) {
    for i in 0..means.len() {
        if clusters[i].len() > 0 {
            means[i] = Vector3::zero();
            for j in 0..clusters[i].len() {
                means[i] = means[i] + clusters[i][j];
            }
            means[i].x /= clusters[i].len() as f32;
            means[i].y /= clusters[i].len() as f32;
            means[i].z /= clusters[i].len() as f32;
        } else {
            let mut rng = rand::thread_rng();
            means[i].x = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
            means[i].y = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
            means[i].z = lerp(-cluster_radius, cluster_radius, rng.gen::<f32>());
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorHash(pub Color);

impl Hash for ColorHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Combine each field of the struct into the hash state
        self.0.r.hash(state);
        self.0.g.hash(state);
        self.0.b.hash(state);
        self.0.a.hash(state);
    }
}
impl Deref for ColorHash {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn cluster_of_color(color: Color, cluster_radius: f32, means: &Vec<Vector3>) -> i32 {
    let p = Vector3::new(
        color.r as f32 / 255.0 * cluster_radius,
        color.g as f32 / 255.0 * cluster_radius,
        color.b as f32 / 255.0 * cluster_radius,
    );

    let mut k: i32 = -1;
    let mut s = f32::MAX;
    for j in 0..means.len() {
        let m = means[j];
        let sm = (p - m).length();
        if sm < s {
            s = sm;
            k = j as i32;
        }
    }

    return k;
}
