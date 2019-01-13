//! A collection of types and functions for representing Matrices
use math::vector::Vec3;

// todo: Missing examples

/// Represents a 4x4 matrix as an array of 16 floats.
pub type Mat4x4 = [f32; 16];

/// A trait that represents the functionality for a matrix
pub trait Matrix {
    /// Creates an identity matrix
    fn identity() -> Self;
    /// Creates a perspective matrix
    ///
    /// # Arguments
    ///
    /// `dimensions` - a tuple containing the width and height of the provided dimensions
    /// `fov` - the field of view for the view
    /// `z` - a tuple containing the near and far z planes
    ///
    /// # Returns
    /// a Matrix
    fn perspective(dimensions: (u32, u32), fov: f32, z: (f32, f32)) -> Self;
    /// Creates a view matrix
    ///
    /// # Arguments
    ///
    /// `position` - The position of the "camera"
    /// `direction` - The direction the camera is facing
    /// `up` - The orientation of the camnera
    fn view(position: &Vec3, direction: &Vec3, up: &Vec3) -> Self;
    /// Converts the matrix into 4 slices of 4 point arrays
    fn to_array(&self) -> [[f32; 4]; 4];
}

impl Matrix for Mat4x4 {
    fn identity() -> Mat4x4 {
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]
    }

    fn perspective(dimensions: (u32, u32), fov: f32, z: (f32, f32)) -> Mat4x4 {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;

        let (near, far) = z;

        let f = 1.0 / (fov / 2.0).tan();

        [
            f * aspect_ratio,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            (far + near) / (far - near),
            1.0,
            0.0,
            0.0,
            -(2.0 * far * near) / (far - near),
            0.0,
        ]
    }

    fn view(position: &Vec3, direction: &Vec3, up: &Vec3) -> Mat4x4 {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [
            up[1] * f[2] - up[2] * f[1],
            up[2] * f[0] - up[0] * f[2],
            up[0] * f[1] - up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let p = [
            -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
        ];

        [
            s_norm[0], u[0], f[0], 0.0, s_norm[1], u[1], f[1], 0.0, s_norm[2], u[2], f[2], 0.0,
            p[0], p[1], p[2], 1.0,
        ]
    }

    fn to_array(&self) -> [[f32; 4]; 4] {
        [
            [self[0], self[1], self[2], self[3]],
            [self[4], self[5], self[6], self[7]],
            [self[8], self[9], self[10], self[11]],
            [self[12], self[13], self[14], self[15]],
        ]
    }
}
