use std::f64::consts::PI;
use std::time::{Duration, Instant};

pub enum AnimationType {
    Timed {
        duration: Duration,
        start_values: Box<[f64]>,
        end_values: Box<[f64]>,
        easing_type: EasingType,
    },
    Conditional {
        condition: fn(current_values: &[f64]) -> bool,
        increase_per_second: Box<[f64]>,
        start_values: Box<[f64]>,
    },
}

#[derive(PartialEq)]
pub enum EasingType {
    Linear,
    EasingOut,
    EasingIn,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    ElasticHalfOut,
    ElasticQuarterOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

impl EasingType {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            EasingType::Linear => x,
            EasingType::EasingIn => Self::ease_in(x),
            EasingType::EasingOut => Self::ease_out(x),
            EasingType::QuadIn => Self::quad_in(x),
            EasingType::QuadOut => Self::quad_out(x),
            EasingType::QuadInOut => Self::quad_in_out(x),
            EasingType::CubicIn => Self::cubic_in(x),
            EasingType::CubicOut => Self::cubic_out(x),
            EasingType::CubicInOut => Self::cubic_in_out(x),
            EasingType::QuartIn => Self::quart_in(x),
            EasingType::QuartOut => Self::quart_out(x),
            EasingType::QuartInOut => Self::quart_in_out(x),
            EasingType::QuintIn => Self::quint_in(x),
            EasingType::QuintOut => Self::quint_out(x),
            EasingType::QuintInOut => Self::quint_in_out(x),
            EasingType::SineIn => Self::sine_in(x),
            EasingType::SineOut => Self::sine_out(x),
            EasingType::SineInOut => Self::sine_in_out(x),
            EasingType::ExpoIn => Self::expo_in(x),
            EasingType::ExpoOut => Self::expo_out(x),
            EasingType::ExpoInOut => Self::expo_in_out(x),
            EasingType::CircIn => Self::circ_in(x),
            EasingType::CircOut => Self::circ_out(x),
            EasingType::CircInOut => Self::circ_in_out(x),
            EasingType::ElasticIn => Self::elastic_in(x),
            EasingType::ElasticOut => Self::elastic_out(x),
            EasingType::ElasticHalfOut => Self::elastic_half_out(x),
            EasingType::ElasticQuarterOut => Self::elastic_quarter_out(x),
            EasingType::ElasticInOut => Self::elastic_in_out(x),
            EasingType::BackIn => Self::back_in(x),
            EasingType::BackOut => Self::back_out(x),
            EasingType::BackInOut => Self::back_in_out(x),
            EasingType::BounceIn => Self::bounce_in(x),
            EasingType::BounceOut => Self::bounce_out(x),
            EasingType::BounceInOut => Self::bounce_in_out(x),
        }
    }

    fn ease_in(t: f64) -> f64 {
        t * t * t
    }

    fn ease_out(t: f64) -> f64 {
        let t = t - 1.0;
        t * t * t + 1.0
    }

    fn quad_in(t: f64) -> f64 {
        t * t
    }

    fn quad_out(t: f64) -> f64 {
        -t * (t - 2.0)
    }

    fn quad_in_out(t: f64) -> f64 {
        let t = t * 2.0;
        if t < 1.0 {
            t * t / 2.0
        } else {
            let t = t - 1.0;
            -0.5 * (t * (t - 2.0) - 1.0)
        }
    }

    fn cubic_in(t: f64) -> f64 {
        t * t * t
    }

    fn cubic_out(t: f64) -> f64 {
        let t = t - 1.0;
        t * t * t + 1.0
    }

    fn cubic_in_out(t: f64) -> f64 {
        let t = t * 2.0;
        if t < 1.0 {
            t * t * t / 2.0
        } else {
            let t = t - 2.0;
            (t * t * t + 2.0) / 2.0
        }
    }

    fn quart_in(t: f64) -> f64 {
        t * t * t * t
    }

    fn quart_out(t: f64) -> f64 {
        let t = t - 1.0;
        -(t * t * t * t - 1.0)
    }

    fn quart_in_out(t: f64) -> f64 {
        let t = t * 2.0;
        if t < 1.0 {
            t * t * t * t / 2.0
        } else {
            let t = t - 2.0;
            -(t * t * t * t - 2.0) / 2.0
        }
    }

    fn quint_in(t: f64) -> f64 {
        t * t * t * t * t
    }

    fn quint_out(t: f64) -> f64 {
        let t = t - 1.0;
        t * t * t * t * t + 1.0
    }

    fn quint_in_out(t: f64) -> f64 {
        let t = t * 2.0;
        if t < 1.0 {
            t * t * t * t * t / 2.0
        } else {
            let t = t - 2.0;
            (t * t * t * t * t + 2.0) / 2.0
        }
    }

    fn sine_in(t: f64) -> f64 {
        -((t * PI / 2.0).cos() - 1.0)
    }

    fn sine_out(t: f64) -> f64 {
        (t * PI / 2.0).sin()
    }

    fn sine_in_out(t: f64) -> f64 {
        -(t * PI).cos() / 2.0 + 0.5
    }

    fn expo_in(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else {
            2.0f64.powf(10.0 * (t - 1.0))
        }
    }

    fn expo_out(t: f64) -> f64 {
        if t == 1.0 {
            1.0
        } else {
            -(2.0f64.powf(-10.0 * t)) + 1.0
        }
    }

    fn expo_in_out(t: f64) -> f64 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            let t = t * 2.0;
            if t < 1.0 {
                2.0f64.powf(10.0 * (t - 1.0)) / 2.0
            } else {
                -(2.0f64.powf(-10.0 * (t - 1.0))) / 2.0 + 1.0
            }
        }
    }

    fn circ_in(t: f64) -> f64 {
        -(1.0 - t * t).sqrt() - 1.0
    }

    fn circ_out(t: f64) -> f64 {
        (1.0 - t * t).sqrt()
    }

    fn circ_in_out(t: f64) -> f64 {
        let t = t * 2.0;
        if t < 1.0 {
            -(1.0 - t * t).sqrt() / 2.0
        } else {
            ((1.0 - t * t).sqrt() + 1.0) / 2.0
        }
    }

    fn back_in(t: f64) -> f64 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        c3 * t * t * t - c1 * t * t
    }

    fn back_out(t: f64) -> f64 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        let t = t - 1.0;
        c3 * t * t * t + c1 * t * t + 1.0
    }

    fn back_in_out(t: f64) -> f64 {
        let c1 = 1.70158;
        let c2 = c1 * 1.525;
        let t = t * 2.0;
        if t < 1.0 {
            (c2 + 1.0) * t * t * t - c2 * t * t
        } else {
            let t = t - 2.0;
            (c2 + 1.0) * t * t * t + c2 * t * t + 2.0
        }
    }

    fn elastic_in(t: f64) -> f64 {
        let c4 = (2.0 * PI) / 3.0;
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            -(2.0f64.powf(10.0 * (t - 1.0)) * (c4 * (t - 1.0)).sin())
        }
    }

    fn elastic_out(t: f64) -> f64 {
        let c4 = (2.0 * PI) / 3.0;
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0f64.powf(-10.0 * t) * (c4 * t).sin() + 1.0
        }
    }

    fn elastic_half_out(t: f64) -> f64 {
        let c4 = (2.0 * PI) / 3.0;
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0f64.powf(-10.0 * t) * (c4 * t).sin() + 0.5
        }
    }

    fn elastic_quarter_out(t: f64) -> f64 {
        let c4 = (2.0 * PI) / 3.0;
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0f64.powf(-10.0 * t) * (c4 * t).sin() + 0.25
        }
    }

    fn elastic_in_out(t: f64) -> f64 {
        let c5 = (2.0 * PI) / 4.5;
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            let t = t * 2.0;
            if t < 1.0 {
                -(2.0f64.powf(10.0 * (t - 1.0)) * (c5 * (t - 1.0 - 0.1125)).sin()) / 2.0
            } else {
                2.0f64.powf(-10.0 * (t - 1.0)) * (c5 * (t - 1.0 - 0.1125)).sin() / 2.0 + 1.0
            }
        }
    }

    fn bounce_in(t: f64) -> f64 {
        1.0 - Self::bounce_out(1.0 - t)
    }

    fn bounce_out(t: f64) -> f64 {
        let n1 = 7.5625;
        let d1 = 2.75;
        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }

    fn bounce_in_out(t: f64) -> f64 {
        if t < 0.5 {
            Self::bounce_in(t * 2.0) * 0.5
        } else {
            Self::bounce_out(t * 2.0 - 1.0) * 0.5 + 0.5
        }
    }
}

pub struct Animation {
    id: u32,
    animation_type: AnimationType,
    start_time: Instant,
    tick_callback: fn(Box<[f64]>),
    end_callback: fn(),
}

impl Animation {
    pub fn new(
        id: u32,
        animation_type: AnimationType,
        start_time: Instant,
        tick_callback: fn(Box<[f64]>),
        end_callback: fn(),
    ) -> Self {
        Self {
            id,
            animation_type,
            start_time,
            tick_callback,
            end_callback,
        }
    }

    pub fn tick(&self) -> bool {
        match &self.animation_type {
            AnimationType::Timed {
                duration,
                start_values,
                end_values,
                easing_type,
            } => {
                let t = self.start_time.elapsed().as_millis() as f64 / duration.as_millis() as f64;
                if t > 1.0 {
                    (self.tick_callback)(end_values.clone());
                    (self.end_callback)();
                    return true;
                } else {
                    let t = easing_type.apply(t);
                    let current_values = start_values
                        .iter()
                        .zip(end_values.iter())
                        .map(|(start, end)| start + (end - start) * t)
                        .collect::<Box<[f64]>>();
                    (self.tick_callback)(current_values);
                }
            }
            AnimationType::Conditional {
                condition,
                increase_per_second,
                start_values,
            } => {
                let current_values = start_values
                    .iter()
                    .zip(increase_per_second.iter())
                    .map(|(start, increase)| start + increase)
                    .collect::<Box<[f64]>>();
                if condition(&current_values) {
                    (self.end_callback)();
                    return true;
                } else {
                    (self.tick_callback)(current_values);
                }
            }
        }

        false
    }
}

impl PartialEq for Animation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct AnimationsManager {
    animations: Vec<Animation>,
    cur_id: u32,
}

impl AnimationsManager {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            cur_id: 0,
        }
    }

    pub fn add(
        &mut self,
        animation_type: AnimationType,
        tick_callback: fn(Box<[f64]>),
        end_callback: fn(),
    ) {
        self.animations.push(Animation::new(
            self.cur_id,
            animation_type,
            Instant::now(),
            tick_callback,
            end_callback,
        ));

        self.cur_id += 1;
    }

    pub fn remove(&mut self, animation: &Animation) {
        let index = self
            .animations
            .iter()
            .position(|x| x == animation)
            .expect("Animation not found");

        self.animations.swap_remove(index);
    }

    pub fn tick(&mut self) {
        self.animations.retain(|animation| !animation.tick());
    }

    pub fn stop_animation(&mut self, animation: &Animation) {
        let index = self
            .animations
            .iter()
            .position(|x| x == animation)
            .expect("Animation not found");

        (self.animations[index].end_callback)();
        self.animations.swap_remove(index);
    }
}