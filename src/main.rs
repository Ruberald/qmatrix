use cursive::view::Nameable;
use rand::prelude::*;
use cursive::view::View;
use cursive::Printer;
use cursive::theme::{BaseColor, Color, PaletteColor};
use cursive::vec::Vec2;
use std::env;

fn main() {
	let mut siv = cursive::default();
    let mut theme = cursive::theme::Theme::default();
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    siv.set_theme(theme);

    let args: Vec<String> = env::args().collect();

	siv.add_global_callback('q', |s| s.quit());

    let (text, config) = parse_args(args);
    println!("{:?}", config);

    let lines: Vec<String> = text.lines().map(String::from).collect();

    siv.run_dummy();
    let canvas = StringCanvas::new(lines, siv.screen_size()).with_name("string_canvas");

	siv.add_fullscreen_layer(canvas);
    
    siv.add_global_callback(cursive::event::Event::Refresh, |s| {
        let screen_size = s.screen_size();
        s.call_on_name("string_canvas", |v: &mut StringCanvas| {
            v.set_size(screen_size);
            v.update();
        });
    });

    siv.set_autorefresh(true);
    siv.set_fps(config.speed as u32);

	siv.run();
}

enum DIRECTION {
    RIGHT,
    LEFT,
}

struct StringCanvas {
    lines: Vec<String>,
    screen_size: Vec2,
    line_pos: Vec<Vec2>, 
    shift: Vec2,
    direction: Vec<DIRECTION>,
}

impl StringCanvas {
    fn new(lines: Vec<String>, screen_size: Vec2) -> Self {
        StringCanvas { 
            lines: lines.clone(), 
            screen_size: screen_size, 
            line_pos: {
                let mut line_pos = Vec::new();

                let mut rng = rand::thread_rng();
                for _ in lines.iter() {
                    line_pos.push(Vec2::new(rng.gen_range(0..100), rng.gen_range(0..50)))
                }

                line_pos
            }, 
            shift: Vec2::new(1, 0),
            direction: {
                let mut direction = Vec::new();

                let mut rng = rand::thread_rng();
                for _ in lines.iter() {
                    direction.push(if rng.gen_bool(0.5) {DIRECTION::RIGHT} else {DIRECTION::LEFT});
                }

                direction
            },
        }
    }

    fn set_size(&mut self, size: Vec2) {
        self.screen_size = size;
    }

    fn move_text(&mut self) {
        self.shift.x = self.shift.x + 1;
    }

    fn change_direction(&mut self, index: usize) {
    }

    fn update(&mut self) {
        for (index, line) in self.lines.iter().enumerate() {
            match self.direction[index] {
                DIRECTION::RIGHT => {
                    self.line_pos[index].x = self.line_pos[index].x + self.shift.x;
                    if self.line_pos[index].x > (self.screen_size.x - line.len()) {
                        self.direction[index] = DIRECTION::LEFT;
                    }
                },

                DIRECTION::LEFT => {
                    self.line_pos[index].x = self.line_pos[index].x - self.shift.x;
                    if self.line_pos[index].x == 0 {
                        self.direction[index] = DIRECTION::RIGHT;
                    }
                }
            }
        }
    }
}

impl View for StringCanvas {
    fn draw(&self, printer: &Printer) {
        for (index, line) in self.lines.iter().enumerate() {
            printer.print(self.line_pos[index], line); 
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(300, 300)
    }
}

#[derive(Default, Debug)]
struct Config {
    mode: String,
    speed: u8,
}

fn parse_args(args: Vec<String>) -> (String, Config) {
    let mut config = Config::default();
    let mut file = String::new();

    let mut args_iter = args.iter();
    loop {
        match args_iter.next() {
            Some(s) => match s.as_str() {
                "--mode" => config.mode = match args_iter.next() {
                    Some(s) => s.clone(),
                    None => {
                        println!("Provide arguments for mode");
                        "".to_string()
                    },
                },

                "--speed" => config.speed = match args_iter.next() {
                    Some(s) => s,
                    None => {
                        println!("Provide arguments for speed");
                        ""
                    },
                }.parse().unwrap_or_else(|_| {
                    println!("Provide a number as speed");
                    0
                }),

                _ => file = s.clone(), 
            },

            None => break,
        }
    }

    (std::fs::read_to_string(file)
        .expect("Error reading file"),
        config )
}
