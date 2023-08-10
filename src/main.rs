use core::time::Duration;
use ggez::graphics::{self, Color, DrawParam, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult};

struct State {}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        self.draw_snake(&mut canvas);

        canvas.finish(ctx)
    }
}

impl State {
    fn draw_snake(&mut self, canvas: &mut graphics::Canvas) {
        print!("Hrllo");
    }
}

fn main() {
    let state = State {};
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "matth")
        .default_conf(c)
        .build()
        .expect("Could not create context!");
    event::run(ctx, event_loop, state);
}

