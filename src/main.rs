// https://github.com/ggez/ggez/issues/263
// https://www.reddit.com/r/rust_gamedev/comments/b9s62l/robo_instructus_some_issues_ive_faced_that_you/

use ggez::*;

struct State {
    dt: std::time::Duration,
    bg: graphics::Color,
    dt_text: graphics::Text,
    fps: f64,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        self.fps = timer::fps(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt_text = graphics::Text::new(format!("{} ms - {} fps", self.dt.subsec_millis().to_string(), self.fps.round().to_string()));
        graphics::clear(ctx, self.bg);
        graphics::draw(ctx, &self.dt_text, (nalgebra::Point2::new(0.0, 0.0),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),
        bg: graphics::Color::new(0.3, 0.3, 0.3, 1.0),
        dt_text: graphics::Text::new("self.dt".to_string()),
        fps: 0.0,
    };

    let mut c = conf::Conf::new();
    c.window_setup.title = "Epic".to_string();
    c.window_setup.vsync = false;
    c.window_mode.resizable = true;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello ggez", "Me")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).ok();
}