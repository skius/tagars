use teng::components::Component;
use teng::{SetupInfo, SharedState, UpdateInfo};
use teng::rendering::color::Color;
use teng::rendering::pixel::Pixel;
use teng::rendering::render::{HalfBlockDisplayRender, Render};
use teng::rendering::renderer::Renderer;
use teng::util::bidivec::BidiVec;
use teng::util::for_coord_in_line;
use teng::util::planarvec2_experimental::Bounds;
use teng::util::planarvec::PlanarVec;
use crate::GameState;

#[derive(Debug, Default)]
pub struct World {
    // The world coordinates where the center of the viewport should be placed in the world.
    camera_attach: (i64, i64),
    screen_width: usize,
    // in half blocks
    screen_height: usize,
}

impl World {
    const WORLD_BORDER_MIN_X: i64 = -200;
    const WORLD_BORDER_MAX_X: i64 = 200;
    const WORLD_BORDER_MIN_Y: i64 = -200;
    const WORLD_BORDER_MAX_Y: i64 = 200;

    pub fn to_screen_pos(&self, world_x: i64, world_y: i64) -> Option<(usize, usize)> {
        let camera_x = self.camera_attach.0;
        let camera_y = self.camera_attach.1;

        // reattach at top-left corner of viewport
        let camera_x = camera_x - (self.screen_width as i64) / 2;
        let camera_y = camera_y + (self.screen_height as i64) / 2;

        // TODO: does this make sense for coords to the left/up of camera?
        let screen_x = world_x - camera_x;
        let screen_y = camera_y - world_y;

        if screen_x < 0 || screen_y < 0 {
            return None;
        }
        if screen_x >= self.screen_width as i64 || screen_y >= self.screen_height as i64 {
            return None;
        }

        Some((screen_x as usize, screen_y as usize))
    }

    pub fn to_world_pos(&self, screen_x: i64, screen_y: i64) -> (i64, i64) {
        let camera_x = self.camera_attach.0;
        let camera_y = self.camera_attach.1;

        // reattach at top-left corner of viewport
        let camera_x = camera_x - (self.screen_width as i64) / 2;
        let camera_y = camera_y + (self.screen_height as i64) / 2;

        let world_x = camera_x + screen_x;
        let world_y = camera_y - screen_y;

        (world_x, world_y)
    }
}

pub struct WorldComponent {
    display: HalfBlockDisplayRender,
}

impl WorldComponent {
    pub fn new() -> Self {
        Self {
            display: HalfBlockDisplayRender::new(0, 0),
        }
    }
}

impl Component<GameState> for WorldComponent {
    fn setup(&mut self, setup_info: &SetupInfo, shared_state: &mut SharedState<GameState>) {
        self.on_resize(setup_info.display_info.width(), setup_info.display_info.height(), shared_state);
    }

    fn on_resize(&mut self, width: usize, height: usize, shared_state: &mut SharedState<GameState>) {
        shared_state.custom.world.screen_width = width;
        shared_state.custom.world.screen_height = 2 * height;
        self.display.resize_discard(width, 2 * height);
    }

    fn update(&mut self, update_info: UpdateInfo, shared_state: &mut SharedState<GameState>) {
        let our_ball = shared_state.custom.our_identity.as_ref().and_then(|identity| shared_state.custom.balls.get(identity));
        if let Some(ball) = our_ball {
            shared_state.custom.world.camera_attach = (ball.x.floor() as i64, ball.y.floor() as i64);
        }

        // render to half block display
        self.display.clear();
        // first render checkerboard pattern (so that balls can overwrite it)
        // let checkerboard_width = 5;
        // let checkerboard_color_a = Color::Rgb([50, 50, 50]);
        // let checkerboard_color_b = Color::Rgb([100, 100, 100]);
        // for x in 0..shared_state.custom.world.screen_width {
        //     for y in 0..shared_state.custom.world.screen_height {
        //         if (x / checkerboard_width + y / checkerboard_width) % 2 == 0 {
        //             self.display.set_color(x, y, checkerboard_color_a);
        //         } else {
        //             self.display.set_color(x, y, checkerboard_color_b);
        //         }
        //     }
        // }

        for ball in shared_state.custom.balls.values() {
            let (screen_x, screen_y) = shared_state.custom.world.to_screen_pos(ball.x.floor() as i64, ball.y.floor() as i64).unwrap();
            let radius = ball.radius as i64;
            for_coord_in_line(false, (screen_x as i64 - radius, 0), (screen_x as i64 + radius, 0), |x, _| {
                for_coord_in_line(false, (0, screen_y as i64 - radius), (0, screen_y as i64 + radius), |_, y| {
                    if (x - screen_x as i64).pow(2) + (y - screen_y as i64).pow(2) <= radius.pow(2) {
                        if x < 0 || y < 0 {
                            return;
                        }
                        let rgb = [ball.color.r, ball.color.g, ball.color.b];
                        self.display.set_color(x as usize, y as usize, Color::Rgb(rgb));
                    }
                });
            });
        }
    }

    fn render(&self, renderer: &mut dyn Renderer, shared_state: &SharedState<GameState>, depth_base: i32) {
        let checkerboard_pattern_depth = depth_base;
        let coord_depth = checkerboard_pattern_depth + 1;
        let balls_depth = coord_depth + 1;
        self.display.render(renderer, 0, 0, balls_depth);

        // render checkerboard
        let checkerboard_scale = 20.0;
        for sy in 0..shared_state.display_info.height() {
            for sx in 0..shared_state.display_info.width() {
                let (x, y) = (sx as i64, sy as i64);
                let (x, y) = shared_state.custom.world.to_world_pos(x, 2 * y);
                let color = if ((x as f64 / checkerboard_scale).floor() as i64 + (y as f64 / checkerboard_scale).floor() as i64) % 2 == 0 {
                    [10; 3]
                } else {
                    [20; 3]
                };
                // check that we're inside world border still
                if x < World::WORLD_BORDER_MIN_X || x > World::WORLD_BORDER_MAX_X || y < World::WORLD_BORDER_MIN_Y || y > World::WORLD_BORDER_MAX_Y {
                    continue;
                }
                renderer.render_pixel(sx, sy, Pixel::transparent().with_bg_color(color), checkerboard_pattern_depth);
            }
        }

        let world = &shared_state.custom.world;

        // render world coords
        for y in 0..world.screen_height {
            let world_y = world.to_world_pos(0, y as i64).1;
            if world_y % 20 == 0 {
                format!("{:?}", world_y).render(renderer, 0, y/2, coord_depth);
            }
        }

        for x in 0..world.screen_width {
            let world_x = world.to_world_pos(x as i64, 0).0;
            if world_x % 100 == 0 {
                format!("|{:?}", world_x).render(
                    renderer,
                    x,
                    shared_state.display_info.height() - 1,
                    coord_depth,
                );
            }
        }
    }

}

