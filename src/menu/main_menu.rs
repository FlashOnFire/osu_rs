use super::middle_menu_bar::MiddleMenuBar;
use graphics::{image, Context};
use piston_window::*;
use std::cmp::{max, min};

pub struct MainMenu {
    background_tex: G2dTexture,
    middle_menu_bar: MiddleMenuBar,
    last_mouse_coords: [f64; 2],
}

impl MainMenu {
    pub fn new(tex_ctx: &mut G2dTextureContext) -> Self {
        let background_tex = Texture::from_path(
            tex_ctx,
            "assets/background.jpg",
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        let middle_menu_bar = MiddleMenuBar::new(tex_ctx);

        Self {
            background_tex,
            middle_menu_bar,
            last_mouse_coords: [0.0, 0.0],
        }
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // Render background
        let [win_width, win_height] = c.get_view_size();

        let back_w = self.background_tex.get_width() as f64 * 0.9;
        let back_h = self.background_tex.get_height() as f64 * 0.9;

        let max_w_offset = (back_w * 0.02) / 2.0;
        let max_h_offset = (back_h * 0.02) / 2.0;

        let offset_w = Self::map_range(
            self.last_mouse_coords[0],
            0.0,
            win_width,
            -max_w_offset,
            max_w_offset,
        );
        let offset_h = Self::map_range(
            self.last_mouse_coords[1],
            0.0,
            win_height,
            -max_h_offset,
            max_h_offset,
        );

        let scale_w = win_width / back_w;
        let scale_h = win_height / back_h;
        let max_scale = f64::max(scale_w, scale_h);

        let new_height = back_h * scale_h;

        let back_trans = c
            .transform
            .trans(
                -max_w_offset,
                (win_height - new_height) / 2.0 - max_h_offset,
            )
            .trans(offset_w, offset_h)
            .scale(max_scale, max_scale);

        image(&self.background_tex, back_trans, g);

        // Render menu bar
        self.middle_menu_bar
            .render(c, g, glyphs, win_width, win_height);
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(coords) = e.mouse_cursor_args() {
            self.last_mouse_coords = coords;
        }

        self.middle_menu_bar.event(e);
    }

    fn map_range(a: f64, a_min: f64, a_max: f64, b_min: f64, b_max: f64) -> f64 {
        ((a - a_min) / (a_max - a_min)) * (b_max - b_min) + b_min
    }
}
