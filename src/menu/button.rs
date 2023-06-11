use piston::{input, GenericEvent};
use piston_window::MouseButton;
use std::collections::VecDeque;

pub(crate) enum Layout {
    Rectangle { x: f64, y: f64, dx: f64, dy: f64 },
    Circle { x: f64, y: f64, radius: f64 },
}

impl Layout {
    fn mouse_inside(&self, [mouse_x, mouse_y]: [f64; 2]) -> bool {
        match self {
            Layout::Rectangle { x, y, dx, dy } => {
                mouse_x >= *x && mouse_x <= *x + *dx && mouse_y >= *y && mouse_y <= *y + *dy
            }
            Layout::Circle { x, y, radius } => {
                ((x - mouse_x).powi(2) + (y - mouse_y).powi(2)).sqrt() <= *radius
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ButtonEvent {
    Click,
    MouseEnter,
    MouseLeave,
    Press,
    Cancel,
    Toggle(bool),
}

pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

pub(crate) struct Button {
    events: VecDeque<ButtonEvent>,
    cursor_inside: bool,
    was_inside: bool,
    pressed: bool,
    toggleable: bool,
    toggle_state: bool,
}

impl Button {
    pub fn new(toggleable: bool) -> Self {
        Self {
            events: VecDeque::new(),
            cursor_inside: false,
            was_inside: false,
            pressed: false,
            toggleable,
            toggle_state: false,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, layout: &Layout, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            let inside = layout.mouse_inside(pos);

            if inside {
                if !self.cursor_inside {
                    self.cursor_inside = true;
                    self.events.push_back(ButtonEvent::MouseEnter);
                }
            } else if self.cursor_inside {
                self.cursor_inside = false;
                self.events.push_back(ButtonEvent::MouseLeave);
            }
        }

        if let Some(input::Button::Mouse(MouseButton::Left)) = e.press_args() {
            if self.cursor_inside {
                self.pressed = true;
                self.was_inside = true;
                self.events.push_back(ButtonEvent::Press);
            }
        }

        if let Some(input::Button::Mouse(MouseButton::Left)) = e.release_args() {
            self.pressed = false;
            if self.cursor_inside {
                if self.toggleable {
                    self.toggle_state = !self.toggle_state;
                    println!("Toggle state: {}", self.toggle_state);
                    self.events
                        .push_back(ButtonEvent::Toggle(self.toggle_state));
                } else {
                    self.events.push_back(ButtonEvent::Click);
                }
            } else if self.was_inside {
                self.events.push_back(ButtonEvent::Cancel);
            }
            self.was_inside = false;
        }
    }

    pub fn state(&self) -> ButtonState {
        match (self.pressed, self.cursor_inside) {
            (false, false) => ButtonState::Normal,
            (false, true) => ButtonState::Hovered,
            (true, _) => ButtonState::Pressed,
        }
    }

    pub fn next_event(&mut self) -> Option<ButtonEvent> {
        self.events.pop_front()
    }
}
