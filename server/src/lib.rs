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

    // returns whether there has been an update or not
    pub fn handle_collision(&mut self, other: &mut Ball) -> bool {
        let mut did_update = false;

        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = (dx*dx + dy*dy).sqrt();
        let overlap = self.radius + other.radius - distance;
        if overlap > 0.0 {
            let overlap = overlap / 2.0;
            let dx = dx / distance * overlap;
            let dy = dy / distance * overlap;
            self.x += dx;
            self.y += dy;
            other.x -= dx;
            other.y -= dy;
            // also update velocities, but take into account the mass of each ball
            let self_mass = self.mass();
            let other_mass = other.mass();
            let normal_x = dx / overlap;
            let normal_y = dy / overlap;
            let relative_velocity_x = self.vx - other.vx;
            let relative_velocity_y = self.vy - other.vy;
            let dot_product = relative_velocity_x * normal_x + relative_velocity_y * normal_y;
            // if dot_product < 0.0 {
            let impulse = 2.0 * dot_product / (self_mass + other_mass);
            self.vx -= impulse * normal_x * other_mass;
            // self.vx -= 5.0;
            self.vy -= impulse * normal_y * other_mass;
            other.vx += impulse * normal_x * self_mass;
            other.vy += impulse * normal_y * self_mass;

            did_update = true;
        }

        did_update
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

    let mut balls = ctx.db.balls().iter().collect::<Vec<_>>();

    // Update positions individually
    for ball in &mut balls {
        ball.vx *= DRAG;
        ball.vy *= DRAG;

        ball.x += ball.vx;
        ball.y += ball.vy;
    }

    // Update wall collisions with WORLD_BORDER
    for ball in &mut balls {
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
    }

    // Update collisions
    for ball1_idx in 0..balls.len() {
        for ball2_idx in ball1_idx+1..balls.len() {
            let (balls1, balls2) = balls.split_at_mut(ball2_idx);
            let ball1 = &mut balls1[ball1_idx];
            let ball2 = &mut balls2[0];

            ball1.handle_collision(ball2);
        }

    }

    for ball in balls {
        ctx.db.balls().identity().update(ball.clone());
    }
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
pub fn init(ctx: &ReducerContext) {
    // Add scheduler for update_balls
    let schedule = UpdateBallsSchedule {
        scheduled_id: 0,
        scheduled_at: TimeDuration::from_micros(16_666).into(),
        // scheduled_at: TimeDuration::from_micros(200_000).into(),
    };
    ctx.db.update_balls_schedule().insert(schedule);
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

