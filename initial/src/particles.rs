use nalgebra::Vector3;

struct Particle {
    mass: f64,
    electric_charge: i8,
    position: Vector3<f64>, // nalgebra 3D vector
    velocity: Vector3<f64>, // nalgebra 3D vector
}

// Define particle system

