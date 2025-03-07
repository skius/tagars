use crate::balls_interpolator::BallsInterpolatorComponent;
use crate::multiplayer::{Ball, Food, ReceiveMessage, SendMessage};
use crate::slingshot::SlingshotComponent;
use crate::world::{World, WorldComponent};
use clap::Parser;
use crossterm::event::KeyCode;
use spacetimedb_sdk::{Identity, Timestamp};
use std::collections::HashMap;
use std::io;
use std::io::stdout;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use teng::components::Component;
use teng::components::debuginfo::{DebugInfoComponent, DebugMessage};
use teng::rendering::color::Color;
use teng::rendering::pixel::Pixel;
use teng::rendering::render::{HalfBlockDisplayRender, Render};
use teng::rendering::renderer::Renderer;
use teng::util::for_coord_in_line;
use teng::util::planarvec::Bounds;
use teng::util::planarvec2_experimental::ExponentialGrowingBounds;
use teng::{
    Game, SetupInfo, SharedState, UpdateInfo, install_panic_handler, terminal_cleanup,
    terminal_setup,
};

mod balls_interpolator;
mod multiplayer;
mod slingshot;
mod world;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The server to connect to.
    #[clap(short, long, default_value = "http://localhost:3000")]
    server: String,

    /// Use position interpolation
    #[clap(short, long)]
    interpolate: bool,
}

fn main() -> anyhow::Result<()> {
    terminal_setup()?;
    install_panic_handler();

    let args = Args::parse();

    let (receive_rx, send_tx) = multiplayer::connect_to(args.server)?;

    let mut game = Game::new_with_custom_buf_writer();
    game.install_recommended_components();
    game.add_component(Box::new(GameComponent::new(receive_rx, send_tx)));
    game.add_component(Box::new(BallsInterpolatorComponent::new(args.interpolate)));
    game.add_component(Box::new(WorldComponent::new()));
    game.add_component(Box::new(SlingshotComponent::new()));
    game.add_component(Box::new(DebugInfoComponent::new()));
    game.run()?;

    terminal_cleanup()?;

    Ok(())
}

#[derive(Debug)]
struct OldAndNewBall {
    old: Ball,
    new: Ball,
    // The time at which we should render the ball at `old`'s position. This is usually the time at which `new` was received.
    timestamp_at_old: Instant,
}

#[derive(Debug, Default)]
struct GameState {
    world: World,
    raw_balls: HashMap<Identity, OldAndNewBall>,
    balls: HashMap<Identity, Ball>,
    foods: HashMap<u64, Food>,
    receive_rx: Option<Receiver<ReceiveMessage>>,
    send_tx: Option<Sender<SendMessage>>,
    our_identity: Option<Identity>,
}

impl GameState {
    fn sender(&self) -> &Sender<SendMessage> {
        self.send_tx.as_ref().unwrap()
    }

    fn receiver(&self) -> &Receiver<ReceiveMessage> {
        self.receive_rx.as_ref().unwrap()
    }
}

struct GameComponent {
    // store them here intermediately as hack until teng supports modifying shared state before `.run()`.
    // TODO: teng: allow modifying shared state before `.run()`
    receive_rx: Option<Receiver<ReceiveMessage>>,
    send_tx: Option<Sender<SendMessage>>,
    last_tick: Timestamp,
    last_frametime: Duration,
}

impl GameComponent {
    fn new(receive_rx: Receiver<ReceiveMessage>, send_tx: Sender<SendMessage>) -> Self {
        Self {
            receive_rx: Some(receive_rx),
            send_tx: Some(send_tx),
            last_tick: Timestamp::now(),
            last_frametime: Duration::from_secs(0),
        }
    }

    fn apply_messages(&mut self, game_state: &mut GameState) {
        while let Ok(message) = game_state.receiver().try_recv() {
            match message {
                ReceiveMessage::NewBall(ball) => {
                    game_state.raw_balls.insert(
                        ball.identity.clone(),
                        OldAndNewBall {
                            old: ball.clone(),
                            new: ball,
                            timestamp_at_old: Instant::now(),
                        },
                    );
                }
                ReceiveMessage::UpdateBall(old_ball, new_ball) => {
                    game_state.raw_balls.insert(
                        new_ball.identity.clone(),
                        OldAndNewBall {
                            old: old_ball,
                            new: new_ball,
                            timestamp_at_old: Instant::now(),
                        },
                    );
                }
                ReceiveMessage::DeleteBall(identity) => {
                    game_state.raw_balls.remove(&identity);
                    game_state.balls.remove(&identity);
                }
                ReceiveMessage::OurIdentity(identity) => {
                    game_state.our_identity = Some(identity);
                }
                ReceiveMessage::NewFood(food) => {
                    game_state.foods.insert(food.id, food);
                }
                ReceiveMessage::UpdateFood(food) => {
                    game_state.foods.insert(food.id, food);
                }
                ReceiveMessage::DeleteFood(id) => {
                    game_state.foods.remove(&id);
                }
                ReceiveMessage::NewPhysicsTick(t) => {
                    let duration = t.duration_since(self.last_tick);
                    self.last_frametime = duration.unwrap_or_default();
                    self.last_tick = t;
                }
            }
        }
    }
}

impl Component<GameState> for GameComponent {
    fn setup(&mut self, setup_info: &SetupInfo, shared_state: &mut SharedState<GameState>) {
        self.on_resize(
            setup_info.display_info.width(),
            setup_info.display_info.height(),
            shared_state,
        );
        let receiver = self.receive_rx.take().unwrap();
        let sender = self.send_tx.take().unwrap();
        shared_state.custom.receive_rx = Some(receiver);
        shared_state.custom.send_tx = Some(sender);
    }

    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        self.apply_messages(&mut shared_state.custom);

        shared_state.debug_info.custom.insert(
            "balls_length".to_string(),
            format!("balls: {}", shared_state.custom.balls.len()),
        );
        shared_state.debug_info.custom.insert(
            "frametime ms".to_string(),
            format!("frametime: {:.2}ms", self.last_frametime.as_millis()),
        );

        // listen to keyboard events to apply impulses
        let impulse_strength = 10.0;
        let mut impulse = (0.0, 0.0);
        if shared_state.pressed_keys.did_press_char_ignore_case('w')
            || shared_state.pressed_keys.did_press(KeyCode::Up)
        {
            impulse.1 += impulse_strength;
        }
        if shared_state.pressed_keys.did_press_char_ignore_case('a')
            || shared_state.pressed_keys.did_press(KeyCode::Left)
        {
            impulse.0 -= impulse_strength;
        }
        if shared_state.pressed_keys.did_press_char_ignore_case('s')
            || shared_state.pressed_keys.did_press(KeyCode::Down)
        {
            impulse.1 -= impulse_strength;
        }
        if shared_state.pressed_keys.did_press_char_ignore_case('d')
            || shared_state.pressed_keys.did_press(KeyCode::Right)
        {
            impulse.0 += impulse_strength;
        }

        if impulse != (0.0, 0.0) {
            let message = SendMessage::Impulse(impulse.0, impulse.1);
            shared_state.custom.sender().send(message).unwrap();
        }

        // use mouse look dir to apply impulse
        let (x, y) = shared_state.mouse_info.last_mouse_pos;
        let x = x as i64;
        let y = y as i64;
        // compute diff to center
        let center_x = shared_state.display_info.width() as i64 / 2;
        let center_y = shared_state.display_info.height() as i64 / 2;
        let diff_x = x - center_x;
        let diff_y = y - center_y;
        let diff_y = -2 * diff_y; // pixel ratio of 2, upside down

        // if shared_state.pressed_keys.did_press_char(' ') {
        // if shared_state.mouse_info.right_mouse_down {
        if shared_state.mouse_pressed.right {
            let message = SendMessage::Impulse(diff_x as f64, diff_y as f64);
            shared_state.custom.sender().send(message).unwrap();
        }
    }
}
