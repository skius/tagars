use std::collections::HashMap;
use std::io;
use std::io::stdout;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
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
use crate::multiplayer::Ball;

mod multiplayer;
mod slingshot;

fn main() -> anyhow::Result<()> {
    terminal_setup()?;
    install_panic_handler();

    let (receive_rx, send_tx) = multiplayer::connect_to("http://localhost:3000".to_string())?;

    let mut game = Game::new_with_custom_buf_writer();
    game.install_recommended_components();
    game.add_component(Box::new(GameComponent::new(receive_rx, send_tx)));
    game.add_component(Box::new(slingshot::SlingshotComponent::new()));
    game.add_component(Box::new(DebugInfoComponent::new()));
    game.run()?;

    terminal_cleanup()?;

    Ok(())
}

#[derive(Debug, Default)]
struct GameState {
    balls: HashMap<Identity, Ball>,
    receive_rx: Option<Receiver<multiplayer::ReceiveMessage>>,
    send_tx: Option<Sender<multiplayer::SendMessage>>,
}

impl GameState {
    fn sender(&self) -> &Sender<multiplayer::SendMessage> {
        self.send_tx.as_ref().unwrap()
    }
    
    fn receiver(&self) -> &Receiver<multiplayer::ReceiveMessage> {
        self.receive_rx.as_ref().unwrap()
    }
}

struct GameComponent {
    // store them here intermediately as hack until teng supports modifying shared state before `.run()`.
    // TODO: teng: allow modifying shared state before `.run()`
    receive_rx: Option<Receiver<multiplayer::ReceiveMessage>>,
    send_tx: Option<Sender<multiplayer::SendMessage>>,
    display: HalfBlockDisplayRender,
}

impl GameComponent {
    fn new(receive_rx: std::sync::mpsc::Receiver<multiplayer::ReceiveMessage>, send_tx: std::sync::mpsc::Sender<multiplayer::SendMessage>) -> Self {
        Self {
            receive_rx: Some(receive_rx),
            send_tx: Some(send_tx),
            display: HalfBlockDisplayRender::new(0, 0),
        }
    }

    fn apply_messages(&mut self, game_state: &mut GameState) {
        while let Ok(message) = game_state.receiver().try_recv() {
            match message {
                multiplayer::ReceiveMessage::NewBall(ball) => {
                    game_state.balls.insert(ball.identity.clone(), ball);
                }
                multiplayer::ReceiveMessage::UpdateBall(ball) => {
                    game_state.balls.insert(ball.identity.clone(), ball);
                }
                multiplayer::ReceiveMessage::DeleteBall(identity) => {
                    game_state.balls.remove(&identity);
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

    fn on_resize(&mut self, width: usize, height: usize, shared_state: &mut SharedState<GameState>) {
        self.display = HalfBlockDisplayRender::new(width, 2 * height);
    }

    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        self.apply_messages(&mut shared_state.custom);


        // render to half block display
        self.display.clear();
        for ball in shared_state.custom.balls.values() {
            let x = ball.x as i64;
            let y = ball.y as i64;
            let radius = ball.radius as i64;
            for_coord_in_line(false, (x - radius, 0), (x + radius, 0), |x, _| {
                for_coord_in_line(false, (0, y - radius), (0, y + radius), |_, y| {
                    if (x - ball.x as i64).pow(2) + (y - ball.y as i64).pow(2) <= radius.pow(2) {
                        if x < 0 || y < 0 {
                            return;
                        }
                        let rgb = [ball.color.r, ball.color.g, ball.color.b];
                        self.display.set_color(x as usize, y as usize, Color::Rgb(rgb));
                    }
                });
            });
        }

        shared_state.debug_info.custom.insert("balls_length".to_string(), format!("balls: {}", shared_state.custom.balls.len()));
    }

    fn render(&self, renderer: &mut dyn Renderer, shared_state: &SharedState<GameState>, depth_base: i32) {
        self.display.render(renderer, 0, 0, depth_base);
    }
}