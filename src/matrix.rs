pub type Matrix = [[f32; 4]; 4];

pub struct OrtoParams {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

pub fn orto(params: OrtoParams) -> Matrix {
    [
        [2.0 / (params.right - params.left), 0.0, 0.0, 0.0],
        [0.0, -2.0 / (params.top - params.bottom), 0.0, 0.0],
        [0.0, 0.0, -2.0 / (params.far - params.near), 0.0],
        [
            -((params.right + params.left) / (params.right - params.left)),
            -((params.top + params.bottom) / (params.top - params.bottom)),
            -((params.far + params.near) / (params.far - params.near)),
            1.0,
        ],
    ]
}

#[allow(dead_code)]
pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, -y, z, 1.0],
    ]
}
#[allow(dead_code)]
pub fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut m: Matrix = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                m[j][i] += a[k][i] * b[j][k];
            }
        }
    }
    m
}

#[allow(dead_code)]
pub fn identity() -> Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
