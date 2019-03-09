use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, PADDLE_WIDTH, PADDLE_HEIGHT, ARENA_WIDTH, ARENA_HEIGHT};

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
                let mut angle = (paddle_y - ARENA_HEIGHT / 2.0).atan2(paddle_x - ARENA_WIDTH / 2.0);
                angle += scaled_amount;
                transform.set_x(
                    ((angle.cos() + 1.0) / 2.0 * ARENA_WIDTH));
                transform.set_y(
                    ((angle.sin() + 1.0) / 2.0 * ARENA_HEIGHT));
                transform.roll_local(-scaled_amount);
            }
        }
    }
}
