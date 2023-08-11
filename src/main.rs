use ggez::graphics::{self, Canvas, Color, DrawParam, Quad};
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult};

const CELL_SIZE: f32 = 20.;
struct State {
    snake: Snake,
}

#[derive(Clone)]
struct Snake {
    body: Vec<BodyCell>,
    direction: Direction,
}

#[derive(Clone, Copy)]
struct BodyCell {
    row: u32,
    column: u32,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.snake = self.get_next_body_pos();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.draw_snake(&mut canvas, ctx);
        canvas.finish(ctx)
    }
}

impl State {
    fn draw_snake(&self, canvas: &mut Canvas, ctx: &Context) {
        for cell in &self.snake.body {
            self.draw_cell(canvas, ctx, cell.row, cell.column, Color::GREEN);
        }
    }

    fn get_next_body_pos(&self) -> Snake {
        let mut new_snake = self.snake.clone();
        match self.snake.direction {
            Direction::Up => new_snake.body[0].row -= 1,
            Direction::Down => new_snake.body[0].row += 1,
            Direction::Left => new_snake.body[0].column -= 1,
            Direction::Right => new_snake.body[0].column += 1,
        }
        for i in 1..self.snake.body.len() {
            new_snake.body[i] = self.snake.body[i - 1];
        }
        return new_snake;
    }

    fn draw_cell(&self, canvas: &mut Canvas, ctx: &Context, row: u32, column: u32, color: Color) {
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let grid_width_in_cells = (window_width / CELL_SIZE) as u32;
        let grid_height_in_cells = (window_height / CELL_SIZE) as u32;
        let invalid_position =
            row < 1 || row > grid_width_in_cells || column < 1 || column > grid_height_in_cells;
        if !invalid_position {
            canvas.draw(
                &Quad {},
                DrawParam::new()
                    .dest([column as f32 * CELL_SIZE, row as f32 * CELL_SIZE])
                    .color(color)
                    .scale([CELL_SIZE, CELL_SIZE]),
            )
        }
    }
}

fn main() {
    let snake = Snake {
        body: vec![BodyCell { row: 20, column: 5 }],
        direction: Direction::Right,
    };
    let state = State { snake };
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "matth")
        .default_conf(c)
        .build()
        .expect("Could not create context!");
    event::run(ctx, event_loop, state);
}
