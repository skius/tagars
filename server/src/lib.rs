use spacetimedb::{Identity, ReducerContext, ScheduleAt, SpacetimeType, Table, TimeDuration, Timestamp};


#[spacetimedb::table(name = update_balls_schedule, scheduled(update_balls))]
struct UpdateBallsSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,

    scheduled_at: ScheduleAt,
}

#[spacetimedb::table(name = respawn_balls_schedule, scheduled(respawn_ball))]
struct RespawnBallsSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,

    scheduled_at: ScheduleAt,

    respawn_for_identity: Identity,
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
    pub dead: bool,
}

impl Ball {
    pub const DEFAULT_RADIUS: f64 = 4.0;
    pub const WORLD_BORDER_MIN_X: f64 = -200.0;
    pub const WORLD_BORDER_MAX_X: f64 = 200.0;
    pub const WORLD_BORDER_MIN_Y: f64 = -200.0;
    pub const WORLD_BORDER_MAX_Y: f64 = 200.0;

    pub fn mass(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    pub fn random_pos_in_game_field(ctx: &ReducerContext) -> (f64, f64) {
        let x = ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_X - Ball::WORLD_BORDER_MIN_X) + Ball::WORLD_BORDER_MIN_X;
        let y = ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_Y - Ball::WORLD_BORDER_MIN_Y) + Ball::WORLD_BORDER_MIN_Y;
        (x, y)
    }

    pub fn spawn_ball(ctx: &ReducerContext, for_identity: Identity) -> Self {
        let rgb = Rgb {
            r: ctx.random(),
            g: ctx.random(),
            b: ctx.random(),
        };
        let (x, y) = Ball::random_pos_in_game_field(ctx);
        let ball = Ball {
            identity: for_identity,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            radius: Ball::DEFAULT_RADIUS,
            color: rgb,
            dead: false,
        };
        ball
    }

    fn respawn(&mut self, ctx: &ReducerContext) {
        let when = ctx.timestamp + TimeDuration::from_micros(5_000_000);
        let schedule = RespawnBallsSchedule {
            scheduled_id: 0,
            scheduled_at: when.into(),
            respawn_for_identity: self.identity,
        };
        let res = ctx.db.respawn_balls_schedule().try_insert(schedule);
        if let Err(err) = res {
            log::error!("Failed to schedule respawn: {}", err);
        }
    }

    fn handle_eating<'a>(ctx: &ReducerContext, mut ball1: &'a mut Ball, mut ball2: &'a mut Ball) {
        let mut mass1 = ball1.mass();
        let mut mass2 = ball2.mass();
        if mass2 > mass1 {
            (ball1, ball2) = (ball2, ball1);
            (mass1, mass2) = (mass2, mass1);
        }

        // ball1 eats ball2
        let new_mass1 = mass1 + mass2;
        let new_radius1 = (new_mass1 / std::f64::consts::PI).sqrt();
        ball1.radius = new_radius1;
        ball2.respawn(ctx);
        ball2.dead = true;
    }

    // returns whether there has been an update or not
    pub fn handle_collision(&mut self, other: &mut Ball, ctx: &ReducerContext) -> bool {
        let mut did_update = false;

        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = (dx*dx + dy*dy).sqrt();
        let overlap = self.radius + other.radius - distance;
        if overlap > 0.0 {
            // we are colliding
            
            let self_mass = self.mass();
            let other_mass = other.mass();
            
            // determine if one eats the other or if they bounce off each other
            if (self_mass - other_mass).abs() > 2.0 {
                // eating will happen
                Ball::handle_eating(ctx, self, other);
            } else {
                // update positions
                let overlap = overlap / 2.0;
                let dx = dx / distance * overlap;
                let dy = dy / distance * overlap;
                self.x += dx;
                self.y += dy;
                other.x -= dx;
                other.y -= dy;
                
                // also update velocities, but take into account the mass of each ball
                let normal_x = dx / overlap;
                let normal_y = dy / overlap;
                let relative_velocity_x = self.vx - other.vx;
                let relative_velocity_y = self.vy - other.vy;
                let dot_product = relative_velocity_x * normal_x + relative_velocity_y * normal_y;
                let impulse = 2.0 * dot_product / (self_mass + other_mass);
                self.vx -= impulse * normal_x * other_mass;
                self.vy -= impulse * normal_y * other_mass;
                other.vx += impulse * normal_x * self_mass;
                other.vy += impulse * normal_y * self_mass;
            }

            did_update = true;
        }

        did_update
    }
}

const DRAG: f64 = 0.95;

#[spacetimedb::reducer]
fn respawn_ball(ctx: &ReducerContext, schedule: RespawnBallsSchedule) {
    if ctx.sender != ctx.identity() {
        log::warn!("Unauthorized attempt to respawn ball from identity {}", ctx.sender);
        return;
    }

    // insert a new ball for identity
    let ball = Ball::spawn_ball(ctx, schedule.respawn_for_identity);
    ctx.db.balls().identity().update(ball);
}

/// Runs every physics tick and updates each ball's position
#[spacetimedb::reducer]
fn update_balls(ctx: &ReducerContext, _schedule: UpdateBallsSchedule) {
    if ctx.sender != ctx.identity() {
        log::warn!("Unauthorized attempt to update balls from identity {}", ctx.sender);
        return;
    }

    let mut balls = ctx.db.balls().iter().filter(|b| !b.dead).collect::<Vec<_>>();

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
        if balls[ball1_idx].dead {
            continue;
        }
        for ball2_idx in ball1_idx+1..balls.len() {
            if balls[ball2_idx].dead {
                continue;
            }
            let (balls1, balls2) = balls.split_at_mut(ball2_idx);
            let ball1 = &mut balls1[ball1_idx];
            let ball2 = &mut balls2[0];

            ball1.handle_collision(ball2, ctx);
            if ball1.dead {
                break;
            }
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
    let ball = Ball::spawn_ball(ctx, ctx.sender);
    ctx.db.balls().insert(ball);
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Remove the ball for the client
    ctx.db.balls().identity().delete(ctx.sender);
}

