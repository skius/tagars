use std::collections::HashMap;
use std::io;
use std::io::stdout;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use clap::Parser;
use crossterm::event::KeyCode;
use spacetimedb_sdk::Identity;
use teng::components::Component;
use teng::rendering::pixel::Pixel;
use teng::rendering::renderer::Renderer;
use teng::util::planarvec::Bounds;
use teng::util::planarvec2_experimental::ExponentialGrowingBounds;
use teng::{
    install_panic_handler, terminal_cleanup, terminal_setup, Game, SetupInfo, SharedState,
    UpdateInfo,
};
use teng::components::debuginfo::{DebugInfoComponent, DebugMessage};
use teng::rendering::color::Color;
use teng::rendering::render::{HalfBlockDisplayRender, Render};
use teng::util::for_coord_in_line;
use crate::balls_interpolator::BallsInterpolatorComponent;
use crate::multiplayer::{Ball, ReceiveMessage, SendMessage};
use crate::slingshot::SlingshotComponent;
use crate::world::{World, WorldComponent};

mod multiplayer;
mod slingshot;
mod world;
mod balls_interpolator;

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
}

impl GameComponent {
    fn new(receive_rx: Receiver<ReceiveMessage>, send_tx: Sender<SendMessage>) -> Self {
        Self {
            receive_rx: Some(receive_rx),
            send_tx: Some(send_tx),
        }
    }

    fn apply_messages(&mut self, game_state: &mut GameState) {
        while let Ok(message) = game_state.receiver().try_recv() {
            match message {
                ReceiveMessage::NewBall(ball) => {
                    game_state.raw_balls.insert(ball.identity.clone(), OldAndNewBall {
                        old: ball.clone(),
                        new: ball,
                        timestamp_at_old: Instant::now(),
                    });
                }
                ReceiveMessage::UpdateBall(old_ball, new_ball) => {
                    game_state.raw_balls.insert(new_ball.identity.clone(), OldAndNewBall {
                        old: old_ball,
                        new: new_ball,
                        timestamp_at_old: Instant::now(),
                    });
                }
                ReceiveMessage::DeleteBall(identity) => {
                    game_state.raw_balls.remove(&identity);
                    game_state.balls.remove(&identity);
                }
                ReceiveMessage::OurIdentity(identity) => {
                    game_state.our_identity = Some(identity);
                }
            }
        }
    }
}

impl Component<GameState> for GameComponent {
    fn setup(&mut self, setup_info: &SetupInfo, shared_state: &mut SharedState<GameState>) {
        self.on_resize(setup_info.display_info.width(), setup_info.display_info.height(), shared_state);
        let receiver = self.receive_rx.take().unwrap();
        let sender = self.send_tx.take().unwrap();
        shared_state.custom.receive_rx = Some(receiver);
        shared_state.custom.send_tx = Some(sender);
    }

    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        self.apply_messages(&mut shared_state.custom);

        shared_state.debug_info.custom.insert("balls_length".to_string(), format!("balls: {}", shared_state.custom.balls.len()));
    }
}