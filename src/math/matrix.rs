pub struct Matrix {
    m1: [f32; 4],
    m2: [f32; 4],
    m3: [f32; 4],
    m4: [f32; 4],
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            m1: [1.0, 0.0, 0.0, 0.0],
            m2: [0.0, 1.0, 0.0, 0.0],
            m3: [0.0, 0.0, 1.0, 0.0],
            m4: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn perspective(dimensions: (u32, u32), fov: f32, z: (f32, f32)) -> Matrix {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;

        let (near, far) = z;

        let f = 1.0 / (fov / 2.0).tan();

        Matrix {
            m1: [f * aspect_ratio, 0.0, 0.0, 0.0],
            m2: [0.0, f, 0.0, 0.0],
            m3: [0.0, 0.0, (far + near) / (far - near), 1.0],
            m4: [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
        }
    }

    pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Matrix {
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

        Matrix {
            m1: [s_norm[0], u[0], f[0], 0.0],
            m2: [s_norm[1], u[1], f[1], 0.0],
            m3: [s_norm[2], u[2], f[2], 0.0],
            m4: [p[0], p[1], p[2], 1.0],
        }
    }

    pub fn to_array(&self) -> [[f32; 4]; 4] {
        [self.m1, self.m2, self.m3, self.m4]
    }
}
