use sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{
    game_config::{
        PiecesColor, DOT_SIZE_IN_PXS, GAME_FIELD_SCALE, GRID_X_OFFSET, GRID_Y_OFFSET,
        TEXTURE_MARGIN, TEXTURE_PIECE_OFFSET, TEXTURE_PIECE_SIZE,
    },
    game_state::{GameContext, GameState, Piece, Point},
};
extern crate sdl2;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().unwrap();
        Ok(Renderer { canvas })
    }

    pub fn draw_dot(&mut self, point: &Point, color: PiecesColor) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .load_texture("./assets/minos00.png")
            .unwrap();
        let offset_x: i32 = color as i32 * (TEXTURE_PIECE_SIZE + TEXTURE_PIECE_OFFSET);
        let source_rec = Rect::new(
            offset_x + TEXTURE_MARGIN,
            TEXTURE_MARGIN,
            TEXTURE_PIECE_SIZE as u32,
            TEXTURE_PIECE_SIZE as u32,
        );

        let dest_rec = Rect::new(
            (point.0 as u32 * DOT_SIZE_IN_PXS + GRID_X_OFFSET) as i32,
            (point.1 as u32 * DOT_SIZE_IN_PXS + GRID_Y_OFFSET) as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        );

        self.canvas
            .copy(&mut texture, Some(source_rec), Some(dest_rec))?;

        Ok(())
    }

    fn draw_piece(&mut self, piece: &Piece) -> Result<(), String> {
        for point in &piece.blocks {
            self.draw_dot(point, piece.color.clone());
        }
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background();
        self.draw_game_field();
        self.draw_piece(&context.current_piece);
        self.canvas.present();
        Ok(())
    }

    fn draw_background(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.load_texture("./assets/bg0002_bg0.png")?;
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.canvas
            .copy_ex(&mut texture, None, None, 0.0, None, false, false)?;
        Ok(())
    }

    fn draw_game_field(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .load_texture("./assets/game_field00^q.png")
            .unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        let texture_query = texture.query();
        let width: u32 = (texture_query.width as f32 * GAME_FIELD_SCALE) as u32;
        let height: u32 = (texture_query.height as f32 * GAME_FIELD_SCALE) as u32;
        self.canvas
            .copy(&mut texture, None, Some(Rect::new(100, 0, width, height)))?;

        Ok(())
    }
}
