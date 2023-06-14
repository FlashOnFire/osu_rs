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
    osu_btn_animation_id: Option<u32>,
    osu_btn_current_ratio: f64,
    osu_btn_last_state: ButtonState,
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
            osu_btn_animation_id: None,
            osu_btn_current_ratio: 0.35,
            osu_btn_last_state: ButtonState::Normal,
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

    pub fn event<E: GenericEvent>(&mut self, e: &E, anim_mgr: &mut AnimationsManager) {
        if let Some(circle) = &self.osu_btn_circle {
            self.osu_button.event(circle, e);
        }

        if e.update_args().is_some() {
            while let Some(btn_event) = self.osu_button.next_event() {
                println!("{:?}", btn_event);
                if btn_event == ButtonEvent::Press {
                    self.test = !self.test;
                }

                if let Some(id) = self.osu_btn_animation_id {
                    if let Some(a) = anim_mgr.get(id) {
                        a.get_current_values().iter().for_each(|v| {
                            print!("{} ", v);
                        });
                        println!();
                    }
                }
            }
        }
    }

    fn calc_osu_btn_transform(
        &mut self,
        c: Context,
        win_width: Scalar,
        win_height: Scalar,
        anim_mgr: &mut AnimationsManager,
    ) -> (Matrix2d, Layout) {
        let (osu_w, osu_h) = self.osu_button_tex.get_size();

        if self.osu_btn_last_state != self.osu_button.state() {
            if let Some(id) = self.osu_btn_animation_id {
                anim_mgr.remove(id);
            }

            self.osu_btn_animation_id = Option::from(anim_mgr.add(AnimationType::Timed {
                duration: Duration::from_millis(200),
                start_values: Box::new([self.osu_btn_current_ratio]),
                end_values: Box::new([match self.osu_button.state() {
                    ButtonState::Normal => 0.30,
                    ButtonState::Hovered => 0.32,
                    ButtonState::Pressed => 0.35,
                }]),
                easing_type: EasingType::CubicInOut,
            }));

            self.osu_btn_last_state = self.osu_button.state();
        }

        let scale = (win_width * self.osu_btn_current_ratio) / osu_w as f64;
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
