use crate::Canvas;
use crate::Color;
use crate::Font;
use crate::Rect;
use crate::Texture;
use crate::TextureCreator;
use crate::TextureQuery;
use crate::Window;

use super::Camera;

pub struct Text<'a> {
    pub text: String,
    pub font: Font<'a, 'a>,
    pub size: u16,
    pub position: [i32; 2],
    pub texture: Texture<'a>,
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    camera: &'a mut Camera,
}

impl<'a> Text<'a> {
    pub fn draw(
        font_path: &str,
        font_size: u16,
        font_context: &'a sdl2::ttf::Sdl2TtfContext,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        text: &str,
        position: &'a mut [i32; 2],
        canvas: &'a mut Canvas<Window>,
        camera: &'a mut Camera,
    ) -> Text<'a> {
        let font = load_font(font_path, font_size, font_context);
        let texture = create_texture_from_text(text, &font, texture_creator);
        render_text(
            canvas,
            text,
            &font,
            texture_creator,
            position[0] - camera.position[0],
            position[1] - camera.position[1],
        );

        Text {
            text: text.to_string(),
            font,
            position: *position,
            size: font_size,
            texture,
            canvas,
            camera,
        }
    }
}

pub fn load_font<'a>(
    path: &str,
    size: u16,
    font_context: &'a sdl2::ttf::Sdl2TtfContext,
) -> Font<'a, 'static> {
    let ttf_context = font_context;
    let font = ttf_context.load_font(path, size).unwrap();
    font
}

pub fn create_texture_from_text<'a>(
    text: &str,
    font: &Font<'a, 'static>,
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
) -> Texture<'a> {
    let surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    texture
}

pub fn render_text<'a>(
    canvas: &mut Canvas<Window>,
    text: &str,
    font: &Font<'a, 'static>,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    x: i32,
    y: i32,
) {
    let texture = create_texture_from_text(text, font, texture_creator);

    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target)).unwrap();
}
