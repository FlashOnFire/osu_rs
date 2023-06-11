use fps_counter::FPSCounter;
use music_manager::MusicManager;
use piston::WindowSettings;
use piston_window::*;
mod menu;

use menu::main_menu::MainMenu;
mod music_manager;

const INITIAL_WIDTH: f64 = 1280.0;
const INITIAL_HEIGHT: f64 = 720.0;

fn main() {
    let window_size = [INITIAL_WIDTH, INITIAL_HEIGHT];
    let mut window: PistonWindow = WindowSettings::new("better_osu", window_size)
        .exit_on_esc(true)
        .controllers(false)
        .vsync(false)
        .build()
        .unwrap();

    window.set_max_fps(9999);
    //println!("{}", window.get_event_settings().max_fps);

    let mut fps_counter = FPSCounter::new();
    let mut fps = 0;

    let mut music_mgr = MusicManager::new();
    music_mgr.set_volume(0.3);
    music_mgr.play_file("welcome.mp3");

    let mut glyphs = window.load_font("assets/Roboto-Regular.ttf").unwrap();
    let mut tex_ctx = window.create_texture_context();

    let in_menu = true;
    let mut menu = MainMenu::new(&mut tex_ctx);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            fps = fps_counter.tick();
            clear([0.0, 0.0, 0.0, 1.0], g);
            //println!("{}", fps);

            if in_menu {
                menu.render(c, g, &mut glyphs);
            }

            Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
                .draw(
                    &(String::from("Volume : ") + &music_mgr.volume().to_string()),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(20.0, 30.0),
                    g,
                )
                .unwrap();

            Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
                .draw(
                    &format!("FPS: {}", fps),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(20.0, 60.0),
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        e.mouse_scroll(|[_horizontal, vertical]| {
            let mut new_vol = music_mgr.volume();

            if vertical.is_sign_positive() {
                new_vol += 0.01
            } else if vertical.is_sign_negative() {
                new_vol -= 0.01
            }

            new_vol = new_vol.clamp(0.0, 1.0);
            music_mgr.set_volume(new_vol);
        });

        menu.event(&e);
    }
}
