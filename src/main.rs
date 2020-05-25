extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    points: i32
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        // Render the game board
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });

        // Render our other game objects
        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) -> bool {
        println!("pos_x: {} pos_y: {}", self.snake.pos_x, self.snake.pos_y);
        // If we can't update the snakes position anymore, end the game
        if (!self.snake.update()) {
            println!("Final Score: {}", self.points);
            return false;
        }

        if (self.snake.pos_x == self.food.pos_x) && (self.snake.pos_y == self.food.pos_y) {
            self.food.spawnFood();
            self.points += 1;
        }

        return true;
    }

    fn pressed(&mut self, btn: &Button) {
        // we want a copy of the snakes last direction headed
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64, 
            (self.pos_y * 20) as f64, 
            20_f64
        );

        // Draw the snake from definitions
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(BLACK, square, transform, gl);
        })
    }

    fn update(&mut self) -> bool {
        // Check if snake hits the boundaries
        if (self.pos_x == 0 || self.pos_x == 699 || self.pos_y == 0 || self.pos_y == 699) {
            return false;
        }

        // update the snakes pos based on direction
        match self.dir {
            Direction::Right => self.pos_x += 1,
            Direction::Left => self.pos_x -= 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1
        }

        return true;
    }
}

struct Food {
    pos_x: i32,
    pos_y: i32
}

impl Food {
    fn render(&self,  gl: &mut GlGraphics, args: &RenderArgs) {
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(BLUE, square, transform, gl);
        })
    }

    fn spawnFood(&mut self) {
        use rand::Rng;
        use rand::thread_rng;

        let mut r = thread_rng();

        self.pos_x = r.gen_range(1, 30);
        self.pos_y = r.gen_range(1, 30);

        println!("Food Coordinates: ({}, {})", self.pos_x, self.pos_y);
    }
}

// Allows us to derive the clone traits into our Direction enum. 
#[derive(Clone, PartialEq)]
enum Direction {
    Right, 
    Left, 
    Up, 
    Down
}

fn main() {
    const MAX_GAME_WIDTH: u32 = 700;
    const MAX_GAME_HEIGHT: u32 = 700;
    // Create the game window
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [MAX_GAME_WIDTH, MAX_GAME_HEIGHT]
    ).graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Init the game struct
    const INIT_SNAKE_POS_X: i32 = 2;
    const INIT_SNAKE_POS_Y: i32 = 2;
    const INIT_SNAKE_DIR: Direction = Direction::Right;

    const INIT_FOOD_POS_X: i32 = 4;
    const INIT_FOOD_POS_Y: i32 = 10;

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake { 
            pos_x: INIT_SNAKE_POS_X, 
            pos_y: INIT_SNAKE_POS_Y,
            dir: INIT_SNAKE_DIR
        },
        food: Food {
            pos_x:INIT_FOOD_POS_X,
            pos_y: INIT_FOOD_POS_Y
        },
        points: 0
    };

    // Event loop!
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        // Render the game board at its current state
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        // move the snake whenever an update method is called
        if let Some(_u) = e.update_args() {
            if !game.update() {
                break;
            }
        }

        // Register any button events
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
