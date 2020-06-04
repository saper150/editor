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
        [0.0, 2.0 / (params.top - params.bottom), 0.0, 0.0],
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
pub fn identity() -> Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
