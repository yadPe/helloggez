// https://github.com/ggez/ggez/issues/263
// https://www.reddit.com/r/rust_gamedev/comments/b9s62l/robo_instructus_some_issues_ive_faced_that_you/
// https://iolivia.me/posts/imgui-ggez/

use ggez::*;

struct State {
    dt: std::time::Duration,
    bg: graphics::Color,
    dt_text: graphics::Text,
    fps: f64,
    pos_x: f32,
    //pos_y: f32,
    dx: f32,
    mouse_pos: nalgebra::Point<f32, nalgebra::base::dimension::U2>,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        self.fps = timer::fps(ctx);
        //self.pos_x += self.dx;
        self.mouse_pos = input::mouse::position(ctx).into();
        input::mouse::set_cursor_hidden(ctx, true);

        // if self.pos_x + 20.0 > ctx.conf.window_mode.width {
        //     self.dx = -self.dx;
        // } else if self.pos_x - 20.0 < 0.0 {
        //     self.dx = -self.dx;
        // }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("{} ms - {} fps - mouse pos {:?}", self.dt.subsec_millis().to_string(), self.fps.round().to_string(), self.mouse_pos.coords.data);
        //self.dt_text = graphics::Text::new(format!("{} ms - {} fps", self.dt.subsec_millis().to_string(), self.fps.round().to_string()));
        graphics::clear(ctx, self.bg);

         let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(self.mouse_pos.coords.data[0], self.mouse_pos.coords.data[1]),
            20.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(0.0, 0.0),))?;

        //graphics::draw(ctx, &self.dt_text, (nalgebra::Point2::new(0.0, 0.0),))?;
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
        pos_x: 100.0,
        dx: 1.0,
        mouse_pos: nalgebra::Point2::new(0.0, 0.0),
    };

    let mut c = conf::Conf::new();
    c.window_setup.title = "Epic".to_string();
    c.window_setup.vsync = true;
    c.window_mode.resizable = true;

    

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello ggez", "Me")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).ok();
}