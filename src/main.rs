use nalgebra::Vector3;
use ray_tracing::devices::CameraBuilder;

fn main() {
    let camera = CameraBuilder::new()
        .sample_size(1337)
        .position_by_coordinates(Vector3::new(-3.0, -4.0, 5.0))
        .look_at(Vector3::new(0.0, 0.0, 0.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 1.0, 0.0))
        .focal_length(0.5)
        .sensor_width(1.0)
        .resolution((1920, 1080))
        .build();

    println!("Camera: {:#?}", camera)
}
