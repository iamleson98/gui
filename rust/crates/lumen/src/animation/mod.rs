//! Animation system — easing functions, tweens, and transitions.
//!
//! Usage:
//! ```text
//! let anim = Animation::new(0.0, 1.0, Duration::from_millis(300), Easing::EaseOut);
//! anim.tick(0.016); // 16ms frame
//! let value = anim.value(); // current animated value
//! ```text

use std::time::Duration;

/// Easing functions for smooth animations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    BounceOut,
    Spring,
}

impl Easing {
    /// Apply the easing function to a progress value (0..1).
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t * t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - ((-2.0 * t + 2.0).powi(3)) / 2.0
                }
            }
            Easing::BounceOut => {
                const N1: f32 = 7.5625;
                const D1: f32 = 2.75;
                if t < 1.0 / D1 {
                    N1 * t * t
                } else if t < 2.0 / D1 {
                    let t = t - 1.5 / D1;
                    N1 * t * t + 0.75
                } else if t < 2.5 / D1 {
                    let t = t - 2.25 / D1;
                    N1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / D1;
                    N1 * t * t + 0.984375
                }
            }
            Easing::Spring => {
                // Simple spring approximation
                let c = (2.0 * std::f32::consts::PI) * 3.0; // frequency
                let d = 0.5; // damping
                1.0 - (std::f32::consts::E).powf(-d * t) * (c * t).cos()
            }
        }
    }
}

/// A tween animation from `from` to `to` over `duration`.
pub struct Animation {
    from: f32,
    to: f32,
    duration: Duration,
    easing: Easing,
    elapsed: f32,
    running: bool,
    completed: bool,
    on_complete: Option<Box<dyn Fn()>>,
}

impl Animation {
    pub fn new(from: f32, to: f32, duration: Duration, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration,
            easing,
            elapsed: 0.0,
            running: false,
            completed: false,
            on_complete: None,
        }
    }

    /// Start the animation.
    pub fn start(&mut self) {
        self.elapsed = 0.0;
        self.running = true;
        self.completed = false;
    }

    /// Advance the animation by `dt` seconds.
    pub fn tick(&mut self, dt: f32) {
        if !self.running || self.completed {
            return;
        }
        self.elapsed += dt;
        let total = self.duration.as_secs_f32();
        if self.elapsed >= total {
            self.elapsed = total;
            self.running = false;
            self.completed = true;
            if let Some(f) = &self.on_complete {
                f();
            }
        }
    }

    /// Get the current animated value.
    pub fn value(&self) -> f32 {
        let total = self.duration.as_secs_f32().max(1e-6);
        let progress = (self.elapsed / total).clamp(0.0, 1.0);
        let eased = self.easing.apply(progress);
        self.from + (self.to - self.from) * eased
    }

    /// Get the raw progress (0..1) without easing applied.
    pub fn progress(&self) -> f32 {
        let total = self.duration.as_secs_f32().max(1e-6);
        (self.elapsed / total).clamp(0.0, 1.0)
    }

    /// Is the animation currently running?
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Has the animation completed?
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Set a callback to run when the animation completes.
    pub fn on_complete<F: Fn() + 'static>(&mut self, f: F) {
        self.on_complete = Some(Box::new(f));
    }

    /// Reset the animation to its initial state.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.running = false;
        self.completed = false;
    }
}

/// An animatable color transition.
pub struct ColorAnimation {
    from: crate::core::Color,
    to: crate::core::Color,
    duration: Duration,
    easing: Easing,
    elapsed: f32,
    running: bool,
}

impl ColorAnimation {
    pub fn new(from: crate::core::Color, to: crate::core::Color, duration: Duration, easing: Easing) -> Self {
        Self { from, to, duration, easing, elapsed: 0.0, running: false }
    }

    pub fn start(&mut self) {
        self.elapsed = 0.0;
        self.running = true;
    }

    pub fn tick(&mut self, dt: f32) {
        if !self.running { return; }
        self.elapsed += dt;
        let total = self.duration.as_secs_f32();
        if self.elapsed >= total {
            self.elapsed = total;
            self.running = false;
        }
    }

    pub fn value(&self) -> crate::core::Color {
        let total = self.duration.as_secs_f32().max(1e-6);
        let progress = (self.elapsed / total).clamp(0.0, 1.0);
        let eased = self.easing.apply(progress);
        self.from.lerp(self.to, eased)
    }

    pub fn is_running(&self) -> bool { self.running }
}

#[cfg(test)]
    use crate::core::Color;
mod tests {
    #[allow(unused_imports)]
    use super::*;
    
    

    #[test]
    fn easing_linear() {
        assert!((Easing::Linear.apply(0.5) - 0.5).abs() < 1e-6);
        assert!((Easing::Linear.apply(0.0) - 0.0).abs() < 1e-6);
        assert!((Easing::Linear.apply(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn easing_ease_out() {
        let v = Easing::EaseOut.apply(0.5);
        assert!(v > 0.5); // ease-out should be past midpoint at t=0.5
        assert!((Easing::EaseOut.apply(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn animation_basic() {
        let mut a = Animation::new(0.0, 100.0, Duration::from_millis(100), Easing::Linear);
        a.start();
        assert!(a.is_running());
        a.tick(0.05); // 50ms
        assert!((a.value() - 50.0).abs() < 1.0);
        a.tick(0.05); // 100ms total
        assert!(!a.is_running());
        assert!(a.is_completed());
        assert!((a.value() - 100.0).abs() < 1e-6);
    }

    #[test]
    fn animation_easing() {
        let mut a = Animation::new(0.0, 1.0, Duration::from_secs(1), Easing::EaseIn);
        a.start();
        a.tick(0.5);
        // With ease-in (t^3), at t=0.5, value should be 0.125
        assert!((a.value() - 0.125).abs() < 1e-6);
    }

    #[test]
    fn animation_on_complete() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let called = Rc::new(RefCell::new(false));
        let c = called.clone();
        let mut a = Animation::new(0.0, 1.0, Duration::from_millis(10), Easing::Linear);
        a.on_complete(move || { *c.borrow_mut() = true; });
        a.start();
        a.tick(0.1);
        assert!(*called.borrow());
    }

    #[test]
    fn color_animation() {
        let mut a = ColorAnimation::new(Color::BLACK, Color::WHITE, Duration::from_secs(1), Easing::Linear);
        a.start();
        a.tick(0.5);
        let c = a.value();
        assert!((c.r as i32 - 128).abs() <= 1);
    }

    #[test]
    fn animation_reset() {
        let mut a = Animation::new(0.0, 10.0, Duration::from_millis(100), Easing::Linear);
        a.start();
        a.tick(0.05);
        a.reset();
        assert!(!a.is_running());
        assert!(!a.is_completed());
    }

    #[test]
    fn easing_bounce() {
        // Bounce should overshoot past 1.0 then settle
        let v = Easing::BounceOut.apply(0.5);
        assert!(v >= 0.0 && v <= 1.0);
        assert!((Easing::BounceOut.apply(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn easing_clamps() {
        assert!((Easing::Linear.apply(-1.0) - 0.0).abs() < 1e-6);
        assert!((Easing::Linear.apply(2.0) - 1.0).abs() < 1e-6);
    }
}
