use spacetimedb::{Identity, ReducerContext, ScheduleAt, SpacetimeType, Table, TimeDuration};


#[spacetimedb::table(name = update_balls_schedule, scheduled(update_balls))]
struct UpdateBallsSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,

    scheduled_at: ScheduleAt,
}

#[derive(SpacetimeType, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone)]
#[spacetimedb::table(name = balls, public)]
pub struct Ball {
    #[primary_key]
    pub identity: Identity,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub color: Rgb,
}

impl Ball {
    pub const WORLD_BORDER_MIN_X: f64 = -200.0;
    pub const WORLD_BORDER_MAX_X: f64 = 200.0;
    pub const WORLD_BORDER_MIN_Y: f64 = -200.0;
    pub const WORLD_BORDER_MAX_Y: f64 = 200.0;
    
    pub fn mass(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

const DRAG: f64 = 0.95;

/// Runs every physics tick and updates each ball's position
#[spacetimedb::reducer]
fn update_balls(ctx: &ReducerContext, _schedule: UpdateBallsSchedule) {
    if ctx.sender != ctx.identity() {
        log::warn!("Unauthorized attempt to update balls from identity {}", ctx.sender);
        return;
    }

    // Update positions individually
    for mut ball in ctx.db.balls().iter() {
        ball.vx *= DRAG;
        ball.vy *= DRAG;

        ball.x += ball.vx;
        ball.y += ball.vy;

        ctx.db.balls().identity().update(ball);
    }
    
    // Update wall collisions with WORLD_BORDER
    for mut ball in ctx.db.balls().iter() {
        if ball.x - ball.radius < Ball::WORLD_BORDER_MIN_X {
            ball.x = Ball::WORLD_BORDER_MIN_X + ball.radius;
            ball.vx = -ball.vx;
        }
        if ball.x + ball.radius > Ball::WORLD_BORDER_MAX_X {
            ball.x = Ball::WORLD_BORDER_MAX_X - ball.radius;
            ball.vx = -ball.vx;
        }
        if ball.y - ball.radius < Ball::WORLD_BORDER_MIN_Y {
            ball.y = Ball::WORLD_BORDER_MIN_Y + ball.radius;
            ball.vy = -ball.vy;
        }
        if ball.y + ball.radius > Ball::WORLD_BORDER_MAX_Y {
            ball.y = Ball::WORLD_BORDER_MAX_Y - ball.radius;
            ball.vy = -ball.vy;
        }
        ctx.db.balls().identity().update(ball);
    }

    // Update collisions
    for mut ball1 in ctx.db.balls().iter() {
        for mut ball2 in ctx.db.balls().iter() {
            if ball1.identity == ball2.identity {
                continue;
            }

            let dx = ball1.x - ball2.x;
            let dy = ball1.y - ball2.y;
            let distance = (dx*dx + dy*dy).sqrt();
            let overlap = ball1.radius + ball2.radius - distance;
            if overlap > 0.0 {
                let overlap = overlap / 2.0;
                let dx = dx / distance * overlap;
                let dy = dy / distance * overlap;
                ball1.x += dx;
                ball1.y += dy;
                ball2.x -= dx;
                ball2.y -= dy;
                // also update velocities, but take into account the mass of each ball
                let ball1_mass = ball1.mass();
                let ball2_mass = ball2.mass();
                let normal_x = dx / overlap;
                let normal_y = dy / overlap;
                let relative_velocity_x = ball1.vx - ball2.vx;
                let relative_velocity_y = ball1.vy - ball2.vy;
                let dot_product = relative_velocity_x * normal_x + relative_velocity_y * normal_y;
                if dot_product < 0.0 {
                    let impulse = 2.0 * dot_product / (ball1_mass + ball2_mass);
                    ball1.vx -= impulse * normal_x * ball2_mass;
                    ball1.vy -= impulse * normal_y * ball2_mass;
                    ball2.vx += impulse * normal_x * ball1_mass;
                    ball2.vy += impulse * normal_y * ball1_mass;

                    ctx.db.balls().identity().update(ball1.clone());
                    ctx.db.balls().identity().update(ball2);

                }

            }
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
    let rgb = Rgb {
        r: ctx.random(),
        g: ctx.random(),
        b: ctx.random(),
    };
    let ball = Ball {
        identity: ctx.sender,
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        radius: 4.0,
        color: rgb,
    };
    ctx.db.balls().insert(ball);
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Remove the ball for the client
    ctx.db.balls().identity().delete(ctx.sender);
}

