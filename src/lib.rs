use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 20;
const BLOCK_SIZE: f64 = 30.0;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Set up the game state
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width((WIDTH as f64 * BLOCK_SIZE) as u32);
    canvas.set_height((HEIGHT as f64 * BLOCK_SIZE) as u32);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let game = Rc::new(RefCell::new(Game::new(context)));

    // Set up the game loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let game_clone = game.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_clone.borrow_mut().update();
        // Schedule the next frame
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();

    // Set up keyboard event listeners
    {
        let game = game.clone();
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            game.borrow_mut().handle_key(event);
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
        keydown_closure.forget();
    }

    Ok(())
}

struct Game {
    context: CanvasRenderingContext2d,
    board: Vec<Vec<u8>>,
    x: i32,
    y: i32,
    frame_count: u32, // Add this line
}

impl Game {
    fn new(context: CanvasRenderingContext2d) -> Self {
        Game {
            context,
            board: vec![vec![0; WIDTH as usize]; HEIGHT as usize],
            x: (WIDTH / 2) as i32,
            y: 0,
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.clear_screen();
        self.draw_board();

        self.frame_count += 1; // Increment the frame counter
        if self.frame_count % 20 == 0 { // Adjust the modulus to control speed
            if self.can_move_down() {
                self.y += 1;
            } else {
                // Place the block on the board
                self.board[self.y as usize][self.x as usize] = 1;
                // Start a new block at the top
                self.x = (WIDTH / 2) as i32;
                self.y = 0;
            }
        }

        self.draw_block(self.x, self.y);
    }

    fn can_move_down(&self) -> bool {
        // Check if the block is at the bottom
        if self.y + 1 >= HEIGHT as i32 {
            return false;
        }
        // Check if there is a block below
        if self.board[(self.y + 1) as usize][self.x as usize] != 0 {
            return false;
        }
        true
    }

    fn handle_key(&mut self, event: KeyboardEvent) {
        match event.key().as_str() {
            "ArrowLeft" => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            "ArrowRight" => {
                if self.x < WIDTH as i32 - 1 {
                    self.x += 1;
                }
            }
            _ => {}
        }
    }

    fn clear_screen(&self) {
        self.context.clear_rect(0.0, 0.0, self.context.canvas().unwrap().width() as f64, self.context.canvas().unwrap().height() as f64);
    }

    fn draw_board(&self) {
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                if self.board[y][x] != 0 {
                    self.draw_cell(x as i32, y as i32, "blue");
                }
            }
        }
    }

    fn draw_block(&self, x: i32, y: i32) {
        self.draw_cell(x, y, "red");
    }

    fn draw_cell(&self, x: i32, y: i32, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(
            x as f64 * BLOCK_SIZE,
            y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        );
    }
}