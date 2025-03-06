use spacetimedb::{Identity, ReducerContext, ScheduleAt, Table, TimeDuration};


#[spacetimedb::table(name = update_balls_schedule, scheduled(update_balls))]
pub struct UpdateBallsSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,

    scheduled_at: ScheduleAt,
}

#[derive(Clone)]
#[spacetimedb::table(name = balls)]
pub struct Ball {
    #[primary_key]
    pub identity: Identity,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
}

impl Ball {
    pub fn mass(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

const DRAG: f64 = 0.99;

/// Runs every physics tick and updates each ball's position
#[spacetimedb::reducer]
fn update_balls(ctx: &ReducerContext, _schedule: UpdateBallsSchedule) {
    // Update positions individually
    for mut ball in ctx.db.balls().iter() {
        ball.vx *= DRAG;
        ball.vy *= DRAG;

        ball.x += ball.vx;
        ball.y += ball.vy;

        ctx.db.balls().identity().update(ball);
    }

    // Update collisions
    for mut ball in ctx.db.balls().iter() {
        let mut did_change = false;
        for mut other in ctx.db.balls().iter() {
            let mut ball = ball.clone();
            if ball.identity == other.identity {
                continue;
            }

            let dx = ball.x - other.x;
            let dy = ball.y - other.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let min_distance = ball.radius + other.radius;

            if distance < min_distance {
                did_change = true;
                let overlap = min_distance - distance;
                let overlap_per_ball = overlap / 2.0;
                let dx = dx / distance * overlap_per_ball;
                let dy = dy / distance * overlap_per_ball;
                ball.x += dx;
                ball.y += dy;
                other.x -= dx;
                other.y -= dy;
                // Update velocities, taking into account mass
                let total_mass = ball.mass() + other.mass();
                let normal_x = dx / overlap_per_ball;
                let normal_y = dy / overlap_per_ball;
                let dot_product = (ball.vx - other.vx) * normal_x + (ball.vy - other.vy) * normal_y;
                let impulse = 2.0 * dot_product / total_mass;
                ball.vx -= impulse * other.mass();
                ball.vy -= impulse * other.mass();
                other.vx += impulse * ball.mass();
                other.vy += impulse * ball.mass();

                ctx.db.balls().identity().update(other);
            }
        }
        if did_change {
            ctx.db.balls().identity().update(ball);
        }
    }

    // log::info!("Updated balls");
}

/// Applies an impulse to the sender's ball
#[spacetimedb::reducer]
fn apply_impulse(ctx: &ReducerContext, impulse_x: f64, impulse_y: f64) {
    let mut ball = ctx.db.balls().identity().find(ctx.sender).unwrap();
    ball.vx += impulse_x;
    ball.vy += impulse_y;
    ctx.db.balls().identity().update(ball);
}


#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Add scheduler for update_balls
    let schedule = UpdateBallsSchedule {
        scheduled_id: 0,
        scheduled_at: TimeDuration::from_micros(16_666).into(),
    };
    _ctx.db.update_balls_schedule().insert(schedule);
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    // Add a new ball for the client
    let ball = Ball {
        identity: ctx.sender,
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        radius: 2.0,
    };
    ctx.db.balls().insert(ball);
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Remove the ball for the client
    ctx.db.balls().identity().delete(ctx.sender);
}

