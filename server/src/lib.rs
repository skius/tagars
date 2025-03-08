mod spatial_hash_grid;

use crate::spatial_hash_grid::{Aabb, SpatialHashGrid, SpatialHashable};
use spacetimedb::{
    Identity, ReducerContext, ScheduleAt, SpacetimeType, Table, TimeDuration, Timestamp,
};

#[spacetimedb::table(name = spawn_foods_schedule, scheduled(spawn_food))]
struct SpawnFoodSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,

    scheduled_at: ScheduleAt,
}

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

#[spacetimedb::table(name = physics_ticks, public)]
struct PhysicsTick {
    #[primary_key]
    #[auto_inc]
    tick_id: u64,
    ticked_at: Timestamp,
}

#[spacetimedb::table(name = foods, public)]
pub struct Food {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub x: f64,
    pub y: f64,
    pub color: Rgb,
}

impl Food {
    pub const MASS: f64 = 3.0;
    pub const MAX_FOODS: u64 = 1000;
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
    pub const RESPAWN_MICROS: i64 = 2_000_000;
    pub const DELTA_RADIUS_REQUIRED_FOR_EATING: f64 = 3.0;

    pub fn mass(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    pub fn random_pos_in_game_field(ctx: &ReducerContext) -> (f64, f64) {
        let x = ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_X - Ball::WORLD_BORDER_MIN_X)
            + Ball::WORLD_BORDER_MIN_X;
        let y = ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_Y - Ball::WORLD_BORDER_MIN_Y)
            + Ball::WORLD_BORDER_MIN_Y;
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

    fn update_mass(&mut self, new_mass: f64) {
        self.radius = (new_mass / std::f64::consts::PI).sqrt();
    }

    fn add_mass(&mut self, mass: f64) {
        self.update_mass(self.mass() + mass);
    }

    fn respawn(&mut self, ctx: &ReducerContext) {
        let when = ctx.timestamp + TimeDuration::from_micros(Self::RESPAWN_MICROS);
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
        ball1.update_mass(new_mass1);
        ball2.respawn(ctx);
        ball2.dead = true;
    }

    // returns whether there has been an update or not
    pub fn handle_collision(&mut self, other: &mut Ball, ctx: &ReducerContext) -> bool {
        let mut did_update = false;

        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let overlap = self.radius + other.radius - distance;
        if overlap > 0.0 {
            // we are colliding

            // determine if one eats the other or if they bounce off each other
            if (self.radius - other.radius).abs() > Self::DELTA_RADIUS_REQUIRED_FOR_EATING {
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
                let self_mass = self.mass();
                let other_mass = other.mass();
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

impl SpatialHashable for Ball {
    fn get_aabb(&self) -> Aabb {
        Aabb {
            min_x: (self.x - self.radius).floor() as i64,
            min_y: (self.y - self.radius).floor() as i64,
            max_x: (self.x + self.radius).floor() as i64,
            max_y: (self.y + self.radius).floor() as i64,
        }
    }
}

const DRAG: f64 = 0.95;

#[spacetimedb::reducer]
fn respawn_ball(ctx: &ReducerContext, schedule: RespawnBallsSchedule) {
    if ctx.sender != ctx.identity() {
        log::warn!(
            "Unauthorized attempt to respawn ball from identity {}",
            ctx.sender
        );
        return;
    }
    if ctx
        .db
        .balls()
        .identity()
        .find(schedule.respawn_for_identity)
        .is_none()
    {
        // player disconnected, no need to respawn
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
        log::warn!(
            "Unauthorized attempt to update balls from identity {}",
            ctx.sender
        );
        return;
    }

    // insert physics tick
    let tick = PhysicsTick {
        tick_id: 0,
        ticked_at: ctx.timestamp,
    };
    ctx.db.physics_ticks().insert(tick);
    // delete ticks older than 1 second
    let one_second_ago = ctx.timestamp + TimeDuration::from_micros(-1_000_000);
    for tick in ctx
        .db
        .physics_ticks()
        .iter()
        .filter(|t| t.ticked_at < one_second_ago)
    {
        ctx.db.physics_ticks().tick_id().delete(tick.tick_id);
    }

    // skip
    // return;

    let mut balls = ctx
        .db
        .balls()
        .iter()
        .filter(|b| !b.dead)
        .collect::<Vec<_>>();

    // (food, keep) pairs
    let mut foods = ctx.db.foods().iter().map(|f| (f, true)).collect::<Vec<_>>();

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

    // Handle food eating
    for ball in &mut balls {
        for (food, keep) in &mut foods {
            if !*keep {
                continue;
            }
            let dx = ball.x - food.x;
            let dy = ball.y - food.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < ball.radius {
                // ball eats food
                ball.add_mass(Food::MASS);
                *keep = false;
            }
        }
    }

    // Update collisions
    // Note: at 4000 balls, this is around ~45-50ms, while below shg implementation is ~30-35ms
    // TODO: also use shg to update foods.
    // TODO: store shg across ticks
    // for ball1_idx in 0..balls.len() {
    //     if balls[ball1_idx].dead {
    //         continue;
    //     }
    //     for ball2_idx in ball1_idx+1..balls.len() {
    //         if balls[ball2_idx].dead {
    //             continue;
    //         }
    //         let (balls1, balls2) = balls.split_at_mut(ball2_idx);
    //         let ball1 = &mut balls1[ball1_idx];
    //         let ball2 = &mut balls2[0];
    //
    //         ball1.handle_collision(ball2, ctx);
    //         if ball1.dead {
    //             break;
    //         }
    //     }
    //
    // }

    // update collisions fast
    let mut grid = SpatialHashGrid::new(10);
    for (idx, ball) in balls.iter().enumerate() {
        grid.insert_with_aabb(idx, ball.get_aabb());
    }

    for idx1 in 0..balls.len() {
        let aabb = balls[idx1].get_aabb();
        for &idx2 in grid.get_for_aabb(aabb) {
            if idx1 == idx2 {
                continue;
            }
            if balls[idx1].dead {
                break;
            }
            if balls[idx2].dead {
                continue;
            }
            let idx_min = idx1.min(idx2);
            let idx_max = idx1.max(idx2);
            let (balls1, balls2) = balls.split_at_mut(idx_max);
            let ball1 = &mut balls1[idx_min];
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
    for (food, keep) in foods {
        if !keep {
            ctx.db.foods().id().delete(food.id);
        }
    }
}

/// Applies an impulse to the sender's ball
#[spacetimedb::reducer]
fn apply_impulse(ctx: &ReducerContext, mut impulse_x: f64, mut impulse_y: f64) {
    let mut ball = ctx.db.balls().identity().find(ctx.sender).unwrap();

    // cap impulse
    let impulse = (impulse_x * impulse_x + impulse_y * impulse_y).sqrt();
    let max_impulse = 20.0;
    if impulse > max_impulse {
        let scale = max_impulse / impulse;
        impulse_x *= scale;
        impulse_y *= scale;
    }

    // take into account ball's mass
    // heavier balls get a less significant velocity change from the same impulse.
    let mass = ball.mass();
    // let mass_normalizer = Ball::DEFAULT_RADIUS * Ball::DEFAULT_RADIUS * std::f64::consts::PI;
    let mass_normalizer = Ball::DEFAULT_RADIUS;
    impulse_x /= (ball.radius - Ball::DEFAULT_RADIUS + 1.0).sqrt();
    impulse_y /= (ball.radius - Ball::DEFAULT_RADIUS + 1.0).sqrt();

    ball.vx += impulse_x;
    ball.vy += impulse_y;

    // cap max velocity, different max per radius
    let max_velocity = 10.0 / (ball.radius - Ball::DEFAULT_RADIUS + 1.0).powf(0.8);
    let velocity = (ball.vx * ball.vx + ball.vy * ball.vy).sqrt();
    if velocity > max_velocity {
        let scale = max_velocity / velocity;
        ball.vx *= scale;
        ball.vy *= scale;
    }

    ctx.db.balls().identity().update(ball);
}

#[spacetimedb::reducer]
fn spawn_food(ctx: &ReducerContext, _schedule: SpawnFoodSchedule) {
    if ctx.sender != ctx.identity() {
        log::warn!(
            "Unauthorized attempt to spawn food from identity {}",
            ctx.sender
        );
        return;
    }

    // Spawn a new food, but only if foods are not saturated yet
    if ctx.db.foods().count() >= Food::MAX_FOODS {
        return;
    }

    for _ in 0..100 {
        let food = Food {
            id: 0,
            x: ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_X - Ball::WORLD_BORDER_MIN_X)
                + Ball::WORLD_BORDER_MIN_X,
            y: ctx.random::<f64>() * (Ball::WORLD_BORDER_MAX_Y - Ball::WORLD_BORDER_MIN_Y)
                + Ball::WORLD_BORDER_MIN_Y,
            color: Rgb {
                r: ctx.random(),
                g: ctx.random(),
                b: ctx.random(),
            },
        };
        ctx.db.foods().insert(food);
    }
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

    // Add scheduler for spawn_food
    let schedule = SpawnFoodSchedule {
        scheduled_id: 0,
        scheduled_at: TimeDuration::from_micros(200_000).into(),
    };
    ctx.db.spawn_foods_schedule().insert(schedule);
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
