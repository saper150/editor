use rusttype::{Font, Point, PositionedGlyph, Scale};

extern crate freetype as ft;

macro_rules! check_error {
    () => {
        let line = line!();
        let error;
        unsafe {
            error = gl::GetError();
        }
        if error != gl::NO_ERROR {
            let message = match error {
                gl::INVALID_ENUM => "INVALID_ENUM",
                gl::INVALID_VALUE => "INVALID_VALUE",
                gl::INVALID_OPERATION => "INVALID_OPERATION",
                gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                _ => "Unknown error",
            };
            println!("file: {} error on line {} {}", file!(), line, message);
        }
    };
}

#[derive(Debug)]
struct Coroners {
    bottom_right: (f32, f32),
    bottom_left: (f32, f32),
    top_right: (f32, f32),
    top_left: (f32, f32),
}

pub struct AtlasGlyph {
    bounding_box: rusttype::Rect<f32>,
    pub advance_width: f32,
    uv: Coroners,
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
    glyphs: std::collections::HashMap<char, AtlasGlyph>,
    pub advance_height: f32,

    texture_height: i32,
    texture_width: i32,
    occupied_width: i32,

    font: rusttype::Font<'static>,
    scale: rusttype::Scale,
}

impl FontAtlas {
    pub fn new(scale: f32) -> FontAtlas {
        // let ref font = args.nth(1).unwrap();
        // let character = args.next().and_then(|s| s.nfc().next()).unwrap() as usize;

        {
            let library = ft::Library::init().unwrap();
            let face = library.new_face("./assets/consolas.ttf", 0).unwrap();

            // face.set_char_size(40 * 64, 0, 50, 0).unwrap();
            face.set_pixel_sizes(0, 16).unwrap();
            // face.load_char('a' as usize, ft::face::LoadFlag::RENDER).unwrap();
            // face.load_glyph('A' as u32, ft::face::LoadFlag::DEFAULT)
            //     .unwrap();
            face.load_char('a' as usize, ft::face::LoadFlag::DEFAULT)
                .unwrap();

            face.glyph().render_glyph(ft::RenderMode::Lcd).unwrap();
            library.set_lcd_filter(ft::library::LcdFilter::LcdFilterDefault).unwrap();
            // let x: ImageBuffer<Luma<u8>, std::vec::Vec<u8>> =
            //     ImageBuffer::from_raw(total_width as u32, max_height as u32, buff.clone()).unwrap();
            // let texture = load_texture(
            //     buff.as_ptr() as *const gl::types::GLvoid,
            //     total_width,
            //     max_height,
            // );
            let glyph = face.glyph();
            let b = glyph.bitmap().buffer().to_vec();

            println!(
                "{:?} {} {:?} ",
                glyph.bitmap().width(),
                glyph.bitmap().rows(),
                b.len()
            );

            println!(
                "{} {} {} {}",
                glyph.bitmap().width(),
                glyph.bitmap().rows(),
                glyph.bitmap().buffer().len(),
                glyph.bitmap().pitch().abs()
            );

            // for( i = 0; i < src_h; i++ )
            // {
            //     //difference between width and pitch: https://www.freetype.org/freetype2/docs/reference/ft2-basic_types.html#FT_Bitmap
            //     memcpy( dst_ptr, src_ptr, ft_bitmap.width);
            //     dst_ptr += tgt_w * self->atlas->depth;
            //     src_ptr += ft_bitmap.pitch;
            // }

            let mut dest: Vec<u8> = vec![];

            for y in 0..glyph.bitmap().rows() {
                for x in 0..glyph.bitmap().width() {
                    dest.push(
                        b[(x + (glyph.bitmap().pitch()) * y) as usize]
                    )
                }
            }

            for y in 0..glyph.bitmap().rows() {
                for x in 0..glyph.bitmap().width() / 3 {
                    if dest[(x + 0 + (glyph.bitmap().width()) * y) as usize] > 20 {
                        print!("1");
                    } else {
                        print!("0");
                    }
                    // print!("\t{}", b[(x + glyph.bitmap().width() * y) as usize]);
                }
                println!("");
            }
            println!("");

            // for x in 0..glyph.bitmap().width() / 3 {
            //     for y in 0..glyph.bitmap().rows() {
            //         print!("\t{}", b[(x + 1 + glyph.bitmap().rows() * y) as usize]);
            //     }
            //     println!("");
            // }
            // println!("");

            // for x in 0..glyph.bitmap().width() / 3 {
            //     for y in 0..glyph.bitmap().rows() {
            //         print!("\t{}", b[(x + 2 + glyph.bitmap().rows() * y) as usize]);
            //     }
            //     println!("");
            // }

            // 7 41

            let x: image::ImageBuffer<image::Rgb<u8>, &[u8]> = image::ImageBuffer::from_raw(
                (glyph.bitmap().width() / 3) as u32,
                (glyph.bitmap().rows()) as u32,
                &dest[..],
            )
            .unwrap();

            x.save("image_example.png").unwrap();
        }

        let font_data = include_bytes!("../assets/consolas.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

        let uniform_scale = Scale::uniform(scale);

        let e = font.glyph('a');

        let v_metrics = font.v_metrics(uniform_scale);

        let cache_width = 15000;
        //  font.glyph_count() as i32 * scale as i32;
        let cache_height = scale as i32 * 2;

        let texture = generate_texture(cache_width, cache_height);

        FontAtlas {
            texture: texture,
            glyphs: std::collections::HashMap::new(),
            advance_height: v_metrics.ascent - v_metrics.descent + v_metrics.line_gap,
            texture_height: cache_height,
            texture_width: cache_width,
            occupied_width: 0,
            scale: uniform_scale,
            font: font,
        }
    }

    pub fn get_glyph(&mut self, char: char) -> &AtlasGlyph {
        if self.glyphs.contains_key(&char) {
            return self.glyphs.get(&char).unwrap();
        } else {
            let glyph = self.load_char(char);
            self.glyphs.insert(char, glyph);
            return self.glyphs.get(&char).unwrap();
        }
    }

    fn load_char(&mut self, char: char) -> AtlasGlyph {
        let glyph = self
            .font
            .glyph(char)
            .scaled(self.scale)
            .positioned(Point { x: 0.0, y: 0.0 });

        let advance_width = glyph.unpositioned().h_metrics().advance_width;

        match glyph.pixel_bounding_box() {
            Some(bounding_box) => {
                let uv = self.load_glyph_to_texture(&glyph);
                return AtlasGlyph {
                    advance_width: advance_width,
                    bounding_box: to_float_rect(bounding_box),
                    uv: uv,
                };
            }
            None => {
                return AtlasGlyph {
                    advance_width: advance_width,
                    bounding_box: rusttype::Rect {
                        max: Point { x: 0.0, y: 0.0 },
                        min: Point { x: 0.0, y: 0.0 },
                    },
                    uv: Coroners {
                        top_left: (0.0, 0.0),
                        top_right: (0.0, 0.0),
                        bottom_right: (0.0, 0.0),
                        bottom_left: (0.0, 0.0),
                    },
                }
            }
        }
    }

    fn load_glyph_to_texture(&mut self, glyph: &rusttype::PositionedGlyph) -> Coroners {
        let bit_map = draw_glyph(&glyph);
        let bb = glyph.pixel_bounding_box().unwrap();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                self.occupied_width,
                0,
                bb.width(),
                bb.height(),
                gl::RED,
                gl::UNSIGNED_BYTE,
                bit_map.as_ptr() as *const gl::types::GLvoid,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        check_error!();

        let uv = Coroners {
            top_left: (
                self.occupied_width as f32 / self.texture_width as f32,
                bb.height() as f32 / self.texture_height as f32,
            ),
            top_right: (
                (self.occupied_width + bb.width()) as f32 / self.texture_width as f32,
                bb.height() as f32 / self.texture_height as f32,
            ),
            bottom_left: (self.occupied_width as f32 / self.texture_width as f32, 0.0),
            bottom_right: (
                (self.occupied_width + bb.width()) as f32 / self.texture_width as f32,
                0.0,
            ),
        };

        self.occupied_width += bb.width() + 2;

        uv
    }
}

fn to_float_rect(bb: rusttype::Rect<i32>) -> rusttype::Rect<f32> {
    rusttype::Rect {
        max: rusttype::Point {
            x: bb.max.x as f32,
            y: bb.max.y as f32,
        },
        min: rusttype::Point {
            x: bb.min.x as f32,
            y: bb.min.y as f32,
        },
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

        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::R8, width, height);
        check_error!();
    }
    println!("{} {}", width, height);

    // check_error!();
    texture
}

fn draw_glyph(glyph: &PositionedGlyph) -> Vec<u8> {
    let bb = glyph.pixel_bounding_box().unwrap();

    let mut buff: Vec<u8> = vec![0; (bb.width() * bb.height()) as usize];

    glyph.draw(|x, y, v| buff[(x + bb.width() as u32 * y) as usize] = (v * 255.0) as u8);
    buff
}
