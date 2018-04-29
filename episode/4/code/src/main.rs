//! Basic hello world example.

extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path;

// First we make a structure to contain the game's state
struct MainState {
    text: String,
    current: usize,
    frames: usize,
    font: graphics::Font,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/font/DejaVuSerif.ttf", 20)?;

        let mut text = String::new();
        let mut f = File::open("resources/text/wikipedia_en_keyboard.txt")?;
        f.read_to_string(&mut text);

        let s = MainState {
            text,
            current: 0,
            frames: 0,
            font: font,
        };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, (255, 255, 255).into());
        let (_, lines) = self.font.get_wrap(&self.text, 500);
        let mut y = 10.0;
        for line in &lines {
            let dest_point = graphics::Point2::new(10.0, y);
            let text = graphics::Text::new(ctx, line, &self.font)?;
            graphics::draw(ctx, &text, dest_point, 0.0)?;
            y += 30.0;
        }
        graphics::set_color(ctx, (255, 0, 0).into());
        let correct_text: String = self.text.chars().take(self.current).collect();
        let (_, lines) = self.font.get_wrap(&correct_text, 500);
        let mut y = 10.0;
        for line in &lines {
            let dest_point = graphics::Point2::new(10.0, y);
            let text = graphics::Text::new(ctx, line, &self.font)?;
            graphics::draw(ctx, &text, dest_point, 0.0)?;
            y += 30.0;
        }
        graphics::present(ctx);

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, text: String) {
        match self.text.chars().nth(self.current) {
            Some(next_char) => {
                if next_char.to_string() == text
                    || (next_char.is_whitespace() && text.chars().all(|c| c.is_whitespace()))
                {
                    println!("We have a match {}", next_char);
                    self.current += 1;
                }
            }
            _ => println!("exit!"),
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
