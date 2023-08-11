use ggez::graphics::{self, Canvas, Color, DrawParam, Quad};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyMods};
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult};
use std::thread;
use std::time::Duration;

const CELL_SIZE: f32 = 20.;
struct State {
    snake: Snake,
    grid: Grid,
}

#[derive(Clone)]
struct Snake {
    body: Vec<BodyCell>,
    direction: Direction,
}

struct Grid {
    width_in_cells: u32,
    height_in_cells: u32,
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Slow down update rate to make the snake controllable
        thread::sleep(Duration::from_millis(100));

        // Check for keypress to change direction
        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Z) {
            self.snake.direction = Direction::Up;
        } else if k_ctx.is_key_pressed(KeyCode::Q) {
            self.snake.direction = Direction::Left;
        } else if k_ctx.is_key_pressed(KeyCode::D) {
            self.snake.direction = Direction::Right;
        } else if k_ctx.is_key_pressed(KeyCode::S) {
            self.snake.direction = Direction::Down;
        }
        self.snake = self.get_next_body_pos();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.draw_snake(&mut canvas);
        canvas.finish(ctx)
    }
}

impl State {
    fn draw_snake(&self, canvas: &mut Canvas) {
        for cell in &self.snake.body {
            self.draw_cell(canvas, cell.row, cell.column, Color::GREEN);
        }
    }

    fn get_next_body_pos(&self) -> Snake {
        let mut new_snake = self.snake.clone();
        let mut new_head = new_snake.body[0];

        // Find where the head should go next
        // If the head reaches an edge of the screen, make it
        // appear on the other side
        match self.snake.direction {
            Direction::Up => {
                if new_head.row - 1 == 0 {
                    new_head.row = self.grid.height_in_cells;
                } else {
                    new_head.row -= 1;
                }
            }
            Direction::Down => {
                if new_head.row + 1 > self.grid.height_in_cells {
                    new_head.row = 0;
                } else {
                    new_head.row += 1;
                }
            }
            Direction::Left => {
                if new_head.column - 1 == 0 {
                    new_head.column = self.grid.width_in_cells;
                } else {
                    new_head.column -= 1;
                }
            }
            Direction::Right => {
                if new_head.column + 1 > self.grid.width_in_cells {
                    new_head.column = 0;
                } else {
                    new_head.column += 1;
                }
            }
        }
        // Move the rest of the body
        for i in 1..self.snake.body.len() {
            new_snake.body[i] = self.snake.body[i - 1];
        }
        new_snake.body[0] = new_head;
        return new_snake;
    }

    fn draw_cell(&self, canvas: &mut Canvas, row: u32, column: u32, color: Color) {
        let invalid_position = row > self.grid.height_in_cells || column > self.grid.width_in_cells;
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

fn init_grid(ctx: &Context) -> Grid {
    let (window_width, window_height) = ctx.gfx.drawable_size();
    let grid_width_in_cells = (window_width / CELL_SIZE) as u32;
    let grid_height_in_cells = (window_height / CELL_SIZE) as u32;
    Grid {
        width_in_cells: grid_width_in_cells,
        height_in_cells: grid_height_in_cells,
    }
}

fn main() {
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "matth")
        .default_conf(c)
        .build()
        .expect("Could not create context!");

    let snake = Snake {
        body: vec![
            BodyCell { row: 20, column: 7 },
            BodyCell { row: 20, column: 5 },
            BodyCell { row: 20, column: 6 },
        ],
        direction: Direction::Right,
    };
    let grid = init_grid(&ctx);
    let state = State { snake, grid };

    event::run(ctx, event_loop, state);
}
