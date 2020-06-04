extern crate freetype as ft;

use crate::check_error;


#[derive(Debug, Clone, Copy)]
struct Coroners {
    bottom_right: (f32, f32),
    bottom_left: (f32, f32),
    top_right: (f32, f32),
    top_left: (f32, f32),
}

#[derive(Clone, Copy)]
pub struct AtlasGlyph {
    size: [f32; 2],
    bearing: [f32; 2],
    pub advance_width: f32,
    uv: Coroners,
}

impl AtlasGlyph {
    pub fn quad(&self, xpos: f32, ypos: f32) -> [[f32; 4]; 4] {
        let x = xpos + self.bearing[0];
        let y = ypos + self.bearing[1];

        let scale = 0.0;


        let bottom_left = [x, y + scale, self.uv.bottom_left.0, self.uv.bottom_left.1];

        let top_left = [x, y - self.size[1], self.uv.top_left.0, self.uv.top_left.1];

        let top_right = [
            x + self.size[0] + scale,
            y - self.size[1],
            self.uv.top_right.0,
            self.uv.top_right.1,
        ];
        let bottom_right = [
            x + self.size[0] + scale,
            y + scale,
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
    glyphs: std::collections::HashMap<char, AtlasGlyph>,
    pub advance_height: f32,

    texture_height: i32,
    texture_width: i32,
    occupied_width: i32,

    face: ft::Face,
}

impl FontAtlas {
    pub fn new(scale: u32) -> FontAtlas {
        let cache_width = 2000;
        let cache_height = 100;

        let texture = generate_texture(cache_width, cache_height);
        
        let library = ft::Library::init().unwrap();
        library.set_lcd_filter(ft::LcdFilter::LcdFilterDefault).unwrap();
        let face = library.new_face("./assets/MonacoB.ttf", 0).unwrap();

        face.set_pixel_sizes(0, scale).unwrap();

        FontAtlas {
            texture: texture,
            glyphs: std::collections::HashMap::new(),
            advance_height: (face.size_metrics().unwrap().height / 64) as f32,
            texture_height: cache_height,
            texture_width: cache_width,
            occupied_width: 0,
            face: face,
        }
    }

    pub fn get_glyph(&mut self, char: char) -> AtlasGlyph {
        let g = self.glyphs.get(&char);

        match g {
            Some(g) => { g.clone() }
            None => {
                let glyph = self.load_char(char);
                self.glyphs.insert(char, glyph.clone());
                glyph
            }
        }

    }

    fn load_char(&mut self, char: char) -> AtlasGlyph {
        self.face
            .load_char(char as usize, ft::face::LoadFlag::DEFAULT)
            .unwrap();

        self.face.glyph().render_glyph(ft::RenderMode::Lcd).unwrap();
        let uv = self.load_bitmap_to_texture(&self.face.glyph().bitmap());

        return AtlasGlyph {
            size: [
                (self.face.glyph().bitmap().width() / 3) as f32,
                self.face.glyph().bitmap().rows() as f32,
            ],
            bearing: [
                self.face.glyph().bitmap_left() as f32,
                self.face.glyph().bitmap_top() as f32,
            ],
            advance_width: (self.face.glyph().advance().x >> 6) as f32,
            uv: uv,
        };
    }

    fn load_bitmap_to_texture(&mut self, bitmap: &ft::Bitmap) -> Coroners {
        let mut dest: Vec<u8> = Vec::new();
        dest.reserve((bitmap.rows() * bitmap.width()) as usize);

        for y in 0..bitmap.rows() {
            for x in 0..bitmap.width() {
                let index = (x + (bitmap.pitch()) * y) as usize;
                dest.push(bitmap.buffer()[index]);
            }
        }

        // let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        //     image::ImageBuffer::from_raw((bitmap.width() / 3) as u32, (bitmap.rows() as u32), dest.clone())
        //         .unwrap();
        // img.save("example_image.png");

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                self.occupied_width,
                0,
                bitmap.width() / 3,
                bitmap.rows(),
                gl::RGB,
                gl::UNSIGNED_BYTE,
                dest.as_ptr() as *const gl::types::GLvoid,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            check_error!();
        }

        let width = bitmap.width() / 3;

        let uv = Coroners {
            top_left: (
                self.occupied_width as f32 / self.texture_width as f32,
                bitmap.rows() as f32 / self.texture_height as f32,
            ),
            top_right: (
                (self.occupied_width + width) as f32 / self.texture_width as f32,
                bitmap.rows() as f32 / self.texture_height as f32,
            ),
            bottom_left: (self.occupied_width as f32 / self.texture_width as f32, 0.0),
            bottom_right: (
                (self.occupied_width + width) as f32 / self.texture_width as f32,
                0.0,
            ),
        };

        self.occupied_width += width + 1;

        uv
    }
}

fn generate_texture(width: i32, height: i32) -> gl::types::GLuint {
    let mut texture: gl::types::GLuint = 0;

    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        check_error!();

        gl::GenTextures(1, &mut texture);
        check_error!();

        gl::BindTexture(gl::TEXTURE_2D, texture);
        check_error!();

        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, width, height);
        check_error!();
    }

    check_error!();
    texture
}
