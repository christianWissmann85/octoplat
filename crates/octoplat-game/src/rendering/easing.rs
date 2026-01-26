//! Easing functions for smooth animations
//!
//! Provides common easing functions for visual transitions.
//! These are utility functions available for animation smoothing.

#![allow(dead_code)]

/// Linear interpolation between two values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp a value between min and max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Ease out quadratic (decelerating)
/// Fast at start, slow at end
pub fn ease_out_quad(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Ease in quadratic (accelerating)
/// Slow at start, fast at end
pub fn ease_in_quad(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    t * t
}

/// Ease in-out cubic (smooth start and end)
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Ease out elastic (bouncy overshoot)
pub fn ease_out_elastic(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
}

/// Ease out bounce
pub fn ease_out_bounce(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
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

/// Ease out back (slight overshoot)
pub fn ease_out_back(t: f32) -> f32 {
    let t = clamp(t, 0.0, 1.0);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;

    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Smooth interpolation with target tracking
/// Returns the new value after moving towards target with given speed
pub fn smooth_towards(current: f32, target: f32, speed: f32, dt: f32) -> f32 {
    if (target - current).abs() < 0.001 {
        return target;
    }
    let diff = target - current;
    current + diff * (speed * dt).min(1.0)
}

/// Spring-like interpolation (for bouncy effects)
/// velocity is passed by reference and updated
pub fn spring_towards(
    current: f32,
    target: f32,
    velocity: &mut f32,
    stiffness: f32,
    damping: f32,
    dt: f32,
) -> f32 {
    let spring_force = (target - current) * stiffness;
    let damping_force = -*velocity * damping;

    *velocity += (spring_force + damping_force) * dt;
    current + *velocity * dt
}
