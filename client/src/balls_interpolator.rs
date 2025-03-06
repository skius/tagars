use teng::components::Component;
use teng::{SharedState, UpdateInfo};
use crate::GameState;
use crate::multiplayer::Ball;

pub struct BallsInterpolatorComponent {
    do_interpolation: bool,
}

impl BallsInterpolatorComponent {
    pub fn new(do_interpolation: bool) -> Self {
        Self { do_interpolation }
    }
}

impl Component<GameState> for BallsInterpolatorComponent {
    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        let game_state = &mut shared_state.custom;
        if !self.do_interpolation {
            // simply copy over the most recent state to the output field
            for (identity, old_and_new_ball) in game_state.raw_balls.iter() {
                game_state.balls.insert(identity.clone(), old_and_new_ball.new.clone());
            }
        } else {
            const PHYSICS_TICKS_PER_SECOND: f64 = 60.0;
            let physics_tick_duration = 1.0 / PHYSICS_TICKS_PER_SECOND;

            // interpolate between old and new balls
            for (identity, old_and_new_ball) in game_state.raw_balls.iter() {
                let old_ball = &old_and_new_ball.old;
                let new_ball = &old_and_new_ball.new;
                let timestamp_at_old = old_and_new_ball.timestamp_at_old;
                let timestamp_now = update_info.current_time;
                let time_since_old = (timestamp_now - timestamp_at_old).as_secs_f64();
                let time_since_old = time_since_old.min(physics_tick_duration);

                let fraction = time_since_old / physics_tick_duration;

                // only interpolate x and y for now
                let x = old_ball.x + fraction * (new_ball.x - old_ball.x);
                let y = old_ball.y + fraction * (new_ball.y - old_ball.y);
                // let vx = old_ball.vx + fraction * (new_ball.vx - old_ball.vx);
                // let vy = old_ball.vy + fraction * (new_ball.vy - old_ball.vy);
                // let radius = old_ball.radius + fraction * (new_ball.radius - old_ball.radius);
                // let color = old_ball.color.interpolate(new_ball.color, fraction);

                let interpolated_ball = Ball {
                    x,
                    y,
                    ..new_ball.clone()
                };

                game_state.balls.insert(identity.clone(), interpolated_ball);
            }
        }
    }
}