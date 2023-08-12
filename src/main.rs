use ggez::graphics::{self, Canvas, Color, DrawParam, Quad};
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, Context, ContextBuilder, GameError, GameResult};
use rand::random;
use std::thread;
use std::time::Duration;

const CELL_SIZE: f32 = 20.;
struct State {
    snake: Snake,
    grid: Grid,
    food: Food,
    end_game: bool,
}

struct Food {
    position: Cell,
}

#[derive(Clone)]
struct Snake {
    body: Vec<Cell>,
    direction: Direction,
}

struct Grid {
    width_in_cells: u32,
    height_in_cells: u32,
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
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

        let k_ctx = &ctx.keyboard;

        // Lose if snake goes on its body
        for cell_nb in 1..self.snake.body.len() {
            if self.snake.body[0] == self.snake.body[cell_nb] {
                self.end_game = true;
                println!("YOU LOSE!");
                if k_ctx.is_key_pressed(KeyCode::R) {
                    println!("Restarted");
                    let new_snake = Snake {
                        body: vec![
                            Cell { row: 20, column: 7 },
                            Cell { row: 20, column: 5 },
                            Cell { row: 20, column: 6 },
                        ],
                        direction: Direction::Right,
                    };
                    self.end_game = false;
                    self.snake = new_snake;
                    let new_food = Food {
                        position: Cell {
                            row: random::<u32>() % self.grid.height_in_cells,
                            column: random::<u32>() % self.grid.width_in_cells,
                        },
                    };
                    self.food = new_food;
                }
                return Ok(());
            }
        }

        // Check for keypress to change direction
        if k_ctx.is_key_pressed(KeyCode::Z) {
            self.snake.direction = Direction::Up;
        } else if k_ctx.is_key_pressed(KeyCode::Q) {
            self.snake.direction = Direction::Left;
        } else if k_ctx.is_key_pressed(KeyCode::D) {
            self.snake.direction = Direction::Right;
        } else if k_ctx.is_key_pressed(KeyCode::S) {
            self.snake.direction = Direction::Down;
        }

        // If snake eats food, make him grow
        if self.snake.body[0] == self.food.position {
            self.snake = self.get_next_body_pos(true);
            let new_food = Food {
                position: Cell {
                    row: random::<u32>() % self.grid.height_in_cells,
                    column: random::<u32>() % self.grid.width_in_cells,
                },
            };
            self.food = new_food;
        } else {
            self.snake = self.get_next_body_pos(false);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        if self.end_game {
            self.draw_end_game_screen(&mut canvas);
            return canvas.finish(ctx);
        } else {
            self.draw_snake(&mut canvas);
            self.draw_cell(
                &mut canvas,
                self.food.position.row,
                self.food.position.column,
                Color::RED,
            );
            canvas.finish(ctx)
        }
    }
}

impl State {
    fn draw_end_game_screen(&self, canvas: &mut Canvas) -> () {
        let text = ggez::graphics::Text::new("YOU LOSE!");
        let screen_center_x = (self.grid.width_in_cells as f32 * CELL_SIZE) / 2.0;
        let screen_center_y = (self.grid.height_in_cells as f32 * CELL_SIZE) / 2.0;

        canvas.draw(
            &text,
            DrawParam::new().dest([screen_center_x, screen_center_y]),
        );
        let text = ggez::graphics::Text::new(format!("Snake length: {}", self.snake.body.len()));
        canvas.draw(
            &text,
            DrawParam::new().dest([screen_center_x, screen_center_y + 40.]),
        );
        let text = ggez::graphics::Text::new("Press r to restart.");
        canvas.draw(
            &text,
            DrawParam::new().dest([screen_center_x, screen_center_y + 20.]),
        );
    }
    fn draw_snake(&self, canvas: &mut Canvas) {
        for cell in &self.snake.body {
            self.draw_cell(canvas, cell.row, cell.column, Color::GREEN);
        }
    }

    fn get_next_body_pos(&self, append_cell: bool) -> Snake {
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
        if append_cell {
            let last_cell = self.snake.body.last();
            let last_cell = match last_cell {
                Some(cell) => cell,
                None => &Cell { row: 1, column: 1 },
            };
            new_snake.body.push(*last_cell);
        }
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
            Cell { row: 20, column: 7 },
            Cell { row: 20, column: 5 },
            Cell { row: 20, column: 6 },
        ],
        direction: Direction::Right,
    };
    let grid = init_grid(&ctx);
    let food = Food {
        position: Cell {
            row: random::<u32>() % &grid.height_in_cells,
            column: random::<u32>() % &grid.width_in_cells,
        },
    };

    let state = State {
        snake,
        grid,
        food,
        end_game: false,
    };

    event::run(ctx, event_loop, state);
}
