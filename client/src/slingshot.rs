use crossterm::event::{Event, MouseButton, MouseEventKind};
use teng::components::Component;
use teng::rendering::color::Color;
use teng::rendering::render::{HalfBlockDisplayRender, Render};
use teng::rendering::renderer::Renderer;
use teng::util::for_coord_in_line;
use teng::{BreakingAction, SetupInfo, SharedState, UpdateInfo};
use crate::{multiplayer, GameState};

pub struct SlingshotComponent {
    // 'Some' with screen coords of the first mouse down event during this slingshot
    first_down: Option<(usize, usize)>,
    // 'Some' with screen coords of the last mouse up event
    last_release: Option<(usize, usize)>,
    // relative (x, y) of the slingshot from the player
    slingshot: Option<(i64, i64)>,
    half_block_display_render: HalfBlockDisplayRender,
}

impl SlingshotComponent {
    pub fn new() -> Self {
        Self {
            first_down: None,
            last_release: None,
            slingshot: None,
            half_block_display_render: HalfBlockDisplayRender::new(0, 0),
        }
    }
}

impl Component<GameState> for SlingshotComponent {
    fn setup(&mut self, setup_info: &SetupInfo, shared_state: &mut SharedState<GameState>) {
        self.on_resize(setup_info.display_info.width(), setup_info.display_info.height(), shared_state);
    }

    fn on_resize(
        &mut self,
        width: usize,
        height: usize,
        shared_state: &mut SharedState<GameState>,
    ) {
        self.half_block_display_render
            .resize_discard(width, 2 * height);
    }

    fn on_event(
        &mut self,
        event: Event,
        shared_state: &mut SharedState<GameState>,
    ) -> Option<BreakingAction> {
        match event {
            Event::Mouse(event) => {
                let (x, y) = (event.column as usize, event.row as usize);
                match event.kind {
                    MouseEventKind::Up(MouseButton::Left) => {
                        self.last_release = Some((x, y));
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        self.first_down.get_or_insert((x, y));
                        // we have no release for _this down_ yet
                        self.last_release = None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }

    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        let game_state = &mut shared_state.custom;

        let mut slingshot = None;



        // if we have an in-bounds first_down and a last_release (anywhere), set a slingshot
        if let Some((initial_x, initial_y)) = self.first_down {
            if let Some((last_x, last_y)) = self.last_release {
                let dx = last_x as i64 - initial_x as i64;
                let dy = last_y as i64 - initial_y as i64;
                // screen coords are flipped in y
                let dy = -dy;

                // invert because we want to apply 'slingshot' force
                slingshot = Some((-dx, -dy));
            } else {
                // we must still be pressing
                // TODO: 'pause' ball? maybe not.
            }
        }

        self.slingshot = slingshot;

        if let Some((s_x, s_y)) = self.slingshot {
            const AMPLIFIER: f64 = 0.1;
            let impulse_x = s_x as f64 * AMPLIFIER;
            let impulse_y = -s_y as f64 * AMPLIFIER * 2.0;

            game_state.sender().send(multiplayer::SendMessage::Impulse(impulse_x, impulse_y)).unwrap();

            self.first_down = None;
            self.last_release = None;
            // TODO: unpause
        }

        // prepare render:
        // render a line in screenspace
        self.half_block_display_render.clear();
        if let Some((initial_x, initial_y)) = self.first_down {
            if shared_state.mouse_info.left_mouse_down {
                let (last_x, last_y) = shared_state.mouse_info.last_mouse_pos;

                let start = (initial_x as i64, initial_y as i64 * 2);
                let end = (last_x as i64, last_y as i64 * 2);

                // draw a lind from initial to last. use the mouse interpolator
                for_coord_in_line(false, start, end, |x, y| {
                    let x = x as usize;
                    let y = y as usize;
                    self.half_block_display_render
                        .set_color(x, y, Color::Rgb([255; 3]));
                });
            }
        }
    }

    fn render(
        &self,
        renderer: &mut dyn Renderer,
        shared_state: &SharedState<GameState>,
        depth_base: i32,
    ) {
        self.half_block_display_render
            .render(renderer, 0, 0, depth_base);
    }
}
