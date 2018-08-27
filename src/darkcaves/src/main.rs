extern crate pancurses;
extern crate rand;

use rand::Rng;
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position(i32, i32);

struct State {
    player: Position,
    velocity: Position,
    size: Position,
    tail: Vec<Position>,
    length: usize,
    apple: Position,
}

impl State {
    fn tick(&mut self) {
        self.tail.push(self.player);
        if self.tail.len() > self.length {
            self.tail.remove(0);
        }

        self.player.0 = (self.player.0 + self.velocity.0 + self.size.0) % self.size.0;
        self.player.1 = (self.player.1 + self.velocity.1 + self.size.1) % self.size.1;

        if self.tail.iter().any(|&position| position == self.player) {
            // We died.
            self.length = 5;
            self.tail.clear();
        }

        if self.player == self.apple {
            self.length += 1;
            let mut rng = rand::thread_rng();
            self.apple = Position(rng.gen_range(0, self.size.0), rng.gen_range(1, self.size.1));
        }
    }

    fn input(&mut self, input: pancurses::Input) {
        match input {
            pancurses::Input::KeyUp => self.velocity = Position(0, -1),
            pancurses::Input::KeyDown => self.velocity = Position(0, 1),
            pancurses::Input::KeyLeft => self.velocity = Position(-1, 0),
            pancurses::Input::KeyRight => self.velocity = Position(1, 0),
            _ => {}
        }
    }
}

fn render(window: &pancurses::Window, state: &State) {
    window.attrset(pancurses::COLOR_PAIR(1));
    for position in &state.tail {
        window.mvaddch(position.1, position.0, '#');
    }
    window.mvaddch(state.player.1, state.player.0, '#');
    window.attrset(pancurses::COLOR_PAIR(2));
    window.mvaddch(state.apple.1, state.apple.0, '#');
}

fn main() {
    let window = pancurses::initscr();
    let result = std::panic::catch_unwind(|| {
        pancurses::start_color();
        pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_RED);
        pancurses::init_pair(2, pancurses::COLOR_GREEN, pancurses::COLOR_GREEN);
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);

        window.nodelay(true);
        window.keypad(true);

        let mut state = State {
            player: Position(10, 10),
            velocity: Position(0, 1),
            size: Position(window.get_max_x(), window.get_max_y()),
            tail: Vec::new(),
            length: 5,
            apple: Position(20, 20),
        };

        loop {
            window.clear();
            render(&window, &state);
            window.refresh();
            std::thread::sleep(std::time::Duration::from_millis(100));
            state.tick();
            if let Some(keypress) = window.getch() {
                state.input(keypress);
            }
        }
    });
    pancurses::endwin();

    if let Err(e) = result {
        if let Some(e) = e.downcast_ref::<&'static str>() {
            writeln!(&mut std::io::stderr(), "Error: {}", e).unwrap();
        } else {
            writeln!(&mut std::io::stderr(), "Unknown error: {:?}", e).unwrap();
        }
        std::process::exit(1);
    }
}
