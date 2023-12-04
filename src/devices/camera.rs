use crate::units::{Pixels, Point, Resolution};
use nalgebra::Vector3;

const DEFAULT_CAMERA_POSITION: Point = Point::new(1.0, 0.5, 0.0);
const DEFAULT_SAMPLE_SIZE: u16 = 50;
const DEFAULT_FOCAL_LENGTH: f64 = 0.5;
const DEFAULT_SENSOR_WIDTH: f64 = 1.0;

const DEFAULT_UP_DIRECTION: Point = Point::new(0.0, 1.0, 0.0);

const DEFAULT_RESOLUTION: Resolution = (1600, 900);

#[derive(Debug)]
pub struct Camera {
    pub sample_size: u16,
    pub position: Vector3<f64>,
    pub look_at: Vector3<f64>,
    pub up_direction: Vector3<f64>,
    pub fov: f64,
    pub resolution: Resolution,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub sensor_width: f64,
    pub pixels: Pixels,
}

pub struct CameraBuilder {
    sample_size: Option<u16>,
    position: Option<Point>,
    look_at: Option<Point>,
    up_direction: Option<Point>,
    resolution: Option<Resolution>,
    focal_length: Option<f64>,
    sensor_width: Option<f64>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            sample_size: None,
            position: None,
            look_at: None,
            up_direction: None,
            resolution: None,
            focal_length: None,
            sensor_width: None,
        }
    }

    pub fn sample_size(&mut self, sample_size: u16) -> &mut Self {
        self.sample_size = Some(sample_size);
        self
    }

    pub fn position_by_coordinates(&mut self, position: Point) -> &mut Self {
        self.position = Some(position);
        self
    }

    pub fn position_by_degrees(
        &mut self,
        _horizontal_degrees: f64,
        _vertical_degrees: f64,
    ) -> &mut Self {
        self.look_at = None; // This is to trigger the default option in the builder
        self
    }

    pub fn look_at(&mut self, coordinate: Point) -> &mut Self {
        self.look_at = Some(coordinate);
        self
    }
    pub fn up_direction_by_coordinates(&mut self, up_direction: Point) -> &mut Self {
        self.up_direction = Some(up_direction);
        self
    }

    pub fn up_direction_by_rotation(&mut self, _rotation: f64) -> &mut Self {
        self
    }

    pub fn resolution(&mut self, resolution: Resolution) -> &mut Self {
        self.resolution = Some(resolution);
        self
    }

    pub fn focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = Some(focal_length);
        self
    }

    pub fn sensor_width(&mut self, sensor_width: f64) -> &mut Self {
        self.sensor_width = Some(sensor_width);
        self
    }

    pub fn build(&self) -> Camera {
        let fov = 2.0
            * ((self.sensor_width.unwrap_or(DEFAULT_SENSOR_WIDTH)
                / (2.0 * self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH)))
            .atan());
        let (width, height) = self.resolution.unwrap_or(DEFAULT_RESOLUTION);
        Camera {
            sample_size: self.sample_size.unwrap_or(DEFAULT_SAMPLE_SIZE),
            position: self.position.unwrap_or(DEFAULT_CAMERA_POSITION),
            look_at: self.look_at.unwrap_or_default(), // 0,0,0 is the default
            up_direction: self.up_direction.unwrap_or(DEFAULT_UP_DIRECTION),
            fov,
            resolution: self.resolution.unwrap_or(DEFAULT_RESOLUTION),
            aspect_ratio: width as f64 / height as f64,
            focal_length: self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH),
            sensor_width: self.sensor_width.unwrap_or(DEFAULT_SENSOR_WIDTH),
            pixels: Vec::new(),
        }
    }
}
