use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        let dt = graphics::Text::new("self.dt");
        graphics::clear(ctx);
        //graphics::draw(ctx, &dt, 11);
        Ok(())
    }
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),
    };

    let mut c = conf::Conf::new();
    c.window_setup.title = "Epic".to_string();
    c.window_mode.resizable = true;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello ggez", "Me")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
