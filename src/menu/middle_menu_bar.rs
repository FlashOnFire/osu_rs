use super::button::{Button, ButtonState, Layout};
use crate::animations::{Animation, AnimationType, AnimationsManager, EasingType};
use crate::menu::button::ButtonEvent;
use graphics::math::{Matrix2d, Scalar};
use graphics::{image, Context, Text};
use piston_window::{
    Flip, G2d, G2dTexture, G2dTextureContext, GenericEvent, Glyphs, ImageSize, Texture,
    TextureSettings, Transformed,
};
use std::time::Duration;

pub(crate) struct MiddleMenuBar {
    osu_button_tex: G2dTexture,
    osu_button: Button,
    osu_btn_transform: Option<Matrix2d>,
    osu_btn_circle: Option<Layout>,
    test: bool,
}

impl MiddleMenuBar {
    pub fn new(tex_ctx: &mut G2dTextureContext) -> Self {
        let osu_button_tex = Texture::from_path(
            tex_ctx,
            "assets/osu.png",
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        Self {
            osu_button_tex,
            osu_button: Button::new(true),
            osu_btn_transform: None,
            osu_btn_circle: None,
            test: false,
        }
    }

    pub(crate) fn render(
        &mut self,
        c: Context,
        g: &mut G2d,
        glyphs: &mut Glyphs,
        win_width: Scalar,
        win_height: Scalar,
        anim_mgr: &mut AnimationsManager,
    ) {
        let (osu_btn_trans, osu_btn_circle) =
            self.calc_osu_btn_transform(c, win_width, win_height, anim_mgr);
        self.osu_btn_transform = Some(osu_btn_trans);
        self.osu_btn_circle = Some(osu_btn_circle);

        image(&self.osu_button_tex, osu_btn_trans, g);

        if self.test {
            Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
                .draw(
                    "aaaaaaaa",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(20.0, 120.0),
                    g,
                )
                .unwrap();
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(circle) = &self.osu_btn_circle {
            self.osu_button.event(circle, e);
        }

        if e.update_args().is_some() {
            while let Some(btn_event) = self.osu_button.next_event() {
                println!("{:?}", btn_event);
                if btn_event == ButtonEvent::Press {
                    self.test = !self.test;
                }
            }
        }
    }

    fn calc_osu_btn_transform(
        &self,
        c: Context,
        win_width: Scalar,
        win_height: Scalar,
        anim_mgr: &mut AnimationsManager,
    ) -> (Matrix2d, Layout) {
        let (osu_w, osu_h) = self.osu_button_tex.get_size();

        let mut ratio = match self.osu_button.state() {
            ButtonState::Normal => 0.30,
            ButtonState::Hovered => 0.33,
            ButtonState::Pressed => 0.35,
        };

        /*match self.osu_button.state() {
            ButtonState::Normal => todo!(),
            ButtonState::Hovered => anim_mgr.add(
                AnimationType::Timed {
                    duration: Duration::from_millis(2000),
                    start_values: Box::new([0.30]),
                    end_values: Box::new([0.50]),
                    easing_type: EasingType::CubicInOut,
                },
                |v| {
                    ratio = v[0];
                },
                || {},
            ),

            ButtonState::Pressed => todo!(),
        }*/

        let scale = (win_width * ratio) / osu_w as f64;
        let new_width = osu_w as f64 * scale;
        let new_height = osu_h as f64 * scale;

        let coords = (
            win_width / 2.0 - new_width / 2.0,
            win_height / 2.0 - new_height / 2.0,
        );

        let transform = c.transform.trans(coords.0, coords.1).scale(scale, scale);
        let circle = Layout::Circle {
            x: win_width / 2.0,
            y: win_height / 2.0,
            radius: osu_w as f64 * 0.9 / 2.0 * scale,
        };

        (transform, circle)
    }
}
