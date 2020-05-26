use image::*;
use rusttype::{Font, GlyphId, Point, PositionedGlyph, Scale};

#[derive(Debug)]
struct Corrners {
    bottom_right: (f32, f32),
    bottom_left: (f32, f32),
    top_right: (f32, f32),
    top_left: (f32, f32),
}

pub struct AtlasGlyph {
    bounding_box: rusttype::Rect<f32>,
    pub advance_width: f32,
    uv: Corrners,
}

impl AtlasGlyph {
    pub fn quad(&self, xpos: f32, ypos: f32) -> [[f32; 4]; 4] {
        let bottom_left = [
            self.bounding_box.min.x + xpos,
            ypos - self.bounding_box.min.y,
            self.uv.bottom_left.0,
            self.uv.bottom_left.1,
        ];

        let top_left = [
            self.bounding_box.min.x + xpos,
            ypos - self.bounding_box.max.y,
            self.uv.top_left.0,
            self.uv.top_left.1,
        ];

        let top_right = [
            self.bounding_box.max.x + xpos,
            ypos - self.bounding_box.max.y,
            self.uv.top_right.0,
            self.uv.top_right.1,
        ];
        let bottom_right = [
            self.bounding_box.max.x + xpos,
            ypos - self.bounding_box.min.y,
            self.uv.bottom_right.0,
            self.uv.bottom_right.1,
        ];

        [bottom_left, top_left, top_right, bottom_right]
    }

    pub fn indices(index: i32) -> [i32; 6] {
        [
            0 + index * 4,
            1 + index * 4,
            2 + index * 4,
            0 + index * 4,
            2 + index * 4,
            3 + index * 4,
        ]
    }
}

pub struct FontAtlas {
    pub texture: gl::types::GLuint,
    pub glyphs: std::collections::HashMap<char, AtlasGlyph>,
}

impl FontAtlas {
    pub fn new() -> FontAtlas {
        let x = load_font();
        FontAtlas {
            texture: x.0,
            glyphs: x.1,
        }
    }
}

fn load_texture(buffer: *const gl::types::GLvoid, width: u32, height: u32) -> gl::types::GLuint {
    let mut texture: gl::types::GLuint = 0;
    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::GenTextures(1, &mut texture);

        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            width as i32,
            height as i32,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    texture
}

pub fn load_font() -> (
    gl::types::GLuint,
    std::collections::HashMap<char, AtlasGlyph>,
) {
    let chars = b'a'..b'z';

    let font_data = include_bytes!("../assets/monaco.ttf");
    let font =
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes");

    let glyphs: Vec<(PositionedGlyph, (i32, i32), char)> = (chars)
        .map(char::from)
        .map(|char| {
            let glyph = font
                .glyph(char)
                .scaled(Scale::uniform(100.0))
                .positioned(Point { x: 0.0, y: 0.0 });

            let bb = glyph.pixel_bounding_box().unwrap();
            let width = bb.max.x - bb.min.x;
            let height = bb.max.y - bb.min.y;

            (glyph, (width, height), char)
        })
        .collect();

    let total_width: u32 = glyphs.iter().map(|x| ((x.1).0) as u32).sum();
    let max_height = glyphs.iter().map(|x| (x.1).1 as u32).max().unwrap();

    let mut buff = vec![0; (total_width * max_height) as usize];

    let mut acc_width = 0;

    let mut atlas_glyphs: std::collections::HashMap<char, AtlasGlyph> =
        std::collections::HashMap::new();

    for g in glyphs {
        let glyph = g.0;
        let dimmensions = g.1;
        let bb = glyph.pixel_bounding_box().unwrap();

        glyph.draw(|x, y, v| {
            let yy = (y) * total_width;
            let xx = x + acc_width;
            let position_in_buffer = xx + yy;
            buff[position_in_buffer as usize] = (v * 255.0) as u8;
        });

        let uv = Corrners {
            top_left: (
                acc_width as f32 / total_width as f32,
                dimmensions.1 as f32 / max_height as f32,
            ),
            top_right: (
                acc_width as f32 / total_width as f32 + dimmensions.0 as f32 / total_width as f32,
                dimmensions.1 as f32 / max_height as f32,
            ),
            bottom_left: (acc_width as f32 / total_width as f32, 0.0),
            bottom_right: (
                acc_width as f32 / total_width as f32 + dimmensions.0 as f32 / total_width as f32,
                0.0,
            ),
        };

        acc_width += dimmensions.0 as u32;
        atlas_glyphs.insert(
            g.2,
            AtlasGlyph {
                uv: uv,
                bounding_box: rusttype::Rect {
                    max: rusttype::Point {
                        x: bb.max.x as f32,
                        y: bb.max.y as f32,
                    },
                    min: rusttype::Point {
                        x: bb.min.x as f32,
                        y: bb.min.y as f32,
                    },
                },
                advance_width: glyph.unpositioned().h_metrics().advance_width,
            },
        );
    }

    // let x: ImageBuffer<Luma<u8>, std::vec::Vec<u8>> =
    //     ImageBuffer::from_raw(total_width as u32, max_height as u32, buff.clone()).unwrap();
    // x.save("image_example.png").unwrap();
    let texture = load_texture(
        buff.as_ptr() as *const gl::types::GLvoid,
        total_width,
        max_height,
    );

    (texture, atlas_glyphs)
}
