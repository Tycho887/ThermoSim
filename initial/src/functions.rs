use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;

fn update_distances_parallel(&mut self) {
    use rayon::prelude::*;
    
    let n = self.positions.len();
    self.distances = vec![vec![0.0; n]; n];
    
    self.distances.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in (i+1)..n {
            let dist = self.positions[i].distance(&self.positions[j]);
            row[j] = dist;
            self.distances[j][i] = dist; // Symmetric matrix
        }
    });
}
