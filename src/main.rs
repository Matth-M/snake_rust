use ggez::graphics::{self, Color, DrawParam, Quad};
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult};

const CELL_SIZE: f32 = 50.;
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
        let snake = Quad {};
        canvas.draw(
            &snake,
            DrawParam::new()
                .dest([150., 150.])
                .color(Color::GREEN)
                .scale([CELL_SIZE, CELL_SIZE]),
        );
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

