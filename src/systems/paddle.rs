use std::f32::consts::PI;

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH};

const TAU: f32 = PI * 2.0;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount == 0.0 {
                    continue;
                }
                let scaled_amount = 0.05 * mv_amount as f32;
                let (paddle_x, paddle_y) = (transform.translation().x, transform.translation().y);
                let angle = (paddle_y - ARENA_HEIGHT / 2.0).atan2(paddle_x - ARENA_WIDTH / 2.0);
                let mut new_angle = angle + scaled_amount;
                new_angle = clamp_angle(new_angle, paddle.min_angle, paddle.max_angle);
                transform.set_x((new_angle.cos() + 1.0) / 2.0 * ARENA_WIDTH);
                transform.set_y((new_angle.sin() + 1.0) / 2.0 * ARENA_HEIGHT);
                transform.roll_local(angle - new_angle);
            }
        }
    }
}

fn clamp_angle(angle: f32, min: f32, max: f32) -> f32 {
    let min_angle = normalize_angle(min - angle);
    let max_angle = normalize_angle(max - angle);

    if min_angle <= 0.0 && max_angle >= 0.0 {
        return angle;
    }

    if min_angle.abs() < max_angle.abs() {
        min
    } else {
        max
    }
}

fn normalize_angle(angle: f32) -> f32 {
    // reduce the angle.
    let mut angle = angle % TAU;

    // Force it to be the positive remainder, so that 0 <= angle < 360.
    angle = (angle + TAU) % TAU;

    if angle > PI {
        angle - TAU
    } else {
        angle
    }
}
