pub mod devices {
    pub mod camera;
    pub use camera::*;
}

pub mod units {
    pub use nalgebra::Vector3;

    pub type Resolution = (u32, u32);

    pub type Point = Vector3<f64>;
    pub type Pixels = Vec<Vector3<f64>>;
}
