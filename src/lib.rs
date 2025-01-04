use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    snake: VecDeque<(i32, i32)>,
    height: u32,
    width: u32,
    direction: (i8, i8),
    is_game_over: bool,
    food: (i32, i32),
    score: u32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<Game, JsValue> {
        if height <= 0 || width <= 0 {
            return Err(JsValue::from_str("Width and height must be > 0"));
        }
        let snake = VecDeque::from([(width as i32 /2, height as i32 /2)]);

        Ok(Game {
            snake,
            height,
            width,
            food: (1, 1),
            is_game_over: false,
            direction: (0, 0),
            score: 0,
        })
    }

    pub fn restart(&mut self) {
        self.snake = VecDeque::from([(self.width as i32 /2, self.height as i32 /2)]);
        self.score = 0;
        self.is_game_over = false;
        self.direction = (0, 0);
        self.food = (1, 1);
    }

    pub fn update(&mut self) {
        if self.is_game_over {
            return;
        }
        let new_head = (
            self.snake[0].0 + self.direction.0 as i32,
            self.snake[0].1 + self.direction.1 as i32,
        );

        if new_head.0 < 0 || new_head.1 < 0 || new_head.0 >= self.width as i32 || new_head.1 >= self.height as i32 {
            self.is_game_over = true;
            return;
        }

        if self.snake.contains(&new_head) {
            self.is_game_over = true;
            return;
        }

        if new_head == self.food {
            self.score += 1;

            if self.score >= (self.width * self.height) {
                // self.score = 0;
                self.is_game_over = true;
                return;
            }
            self.generate_food();
        } else {
            self.snake.pop_back();
        }

        self.snake.push_front(new_head);
        println!("{:?}", self.snake);
    }

    pub fn change_direction(&mut self, key: &str) {
        self.direction = match key {
            "ArrowUp" if self.direction != (0, 1) => (0, -1),
            "ArrowDown" if self.direction != (0, -1) => (0, 1),
            "ArrowLeft" if self.direction != (1, 0) => (-1, 0),
            "ArrowRight" if self.direction != (-1, 0) => (1, 0),
            _ => {self.direction}
        };
    }

    pub fn draw(&self, context: web_sys::CanvasRenderingContext2d) {
        context.clear_rect(0.0, 0.0, (self.width * 10) as f64, (self.height * 10) as f64);

        context.set_fill_style_str(&"#00FF00");
        for (x, y) in &self.snake {
            context.fill_rect(
                *x as f64 * 10.0,
                *y as f64 * 10.0,
                10.0,
                10.0,
            )
        }

        context.set_fill_style_str(&"#000000");
        context.fill_rect(
            self.food.0 as f64 * 10.0,
            self.food.1 as f64 * 10.0,
            10.0,
            10.0,
        );
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    fn generate_food(&mut self) {
        // let mut rng = rand::rng();
        loop {
            let x  = rand::random::<u32>() % self.width;
            let y  = rand::random::<u32>() % self.height;

            if !self.snake.contains(&(x as i32, y as i32)) {
                self.food = (x as i32, y as i32);
                break
            }
        }
    }


}

#[test]
fn test_direction() {
    let mut game = Game::new(40, 40).unwrap();
    game.change_direction("ArrowUp");

    println!("{},{}", game.direction.0, game.direction.1);
}