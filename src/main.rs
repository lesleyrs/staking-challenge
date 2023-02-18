use runty8::{App, Button};

struct Game {
    total: i32,
    stake: i32,
    frames: i32,
    delay: i32,
    digit: i8,
    pos: i32,
    value: i32,
    hiscore: i32,
}

impl App for Game {
    fn init(_pico8: &mut runty8::Pico8) -> Self {
        Self {
            total: 0,
            stake: 0,
            frames: 0,
            delay: 4,
            digit: 10,
            pos: 103,
            value: 1,
            hiscore: 0,
        }
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        self.frames += 1;
        // add menu with difficulty setting + game over + rectfill
        // vfx based on points + pset?
        // animation for adding to total "stack"

        // how to calc this instead?
        match self.digit {
            1 => self.value = 1000000000,
            2 => self.value = 100000000,
            3 => self.value = 10000000,
            4 => self.value = 1000000,
            5 => self.value = 100000,
            6 => self.value = 10000,
            7 => self.value = 1000,
            8 => self.value = 100,
            9 => self.value = 10,
            10 => self.value = 1,
            _ => self.value = 1,
        }
        if self.frames > self.delay {
            match self.digit {
                1 => {
                    if pico8.btn(Button::Left) {
                        self.digit = 10;
                        self.pos = 103;
                    }
                }
                _ => {
                    if pico8.btn(Button::Left) {
                        self.digit -= 1;
                        self.pos -= 4;
                    }
                }
            }
            match self.digit {
                10 => {
                    if pico8.btn(Button::Right) {
                        self.digit = 1;
                        self.pos = 67;
                    }
                }
                _ => {
                    if pico8.btn(Button::Right) {
                        self.digit += 1;
                        self.pos += 4;
                    }
                }
            }
            if pico8.btn(Button::Up) {
                let add = self.stake.saturating_add(self.value);
                self.stake = add
            }
            if pico8.btn(Button::Down) {
                // needed to avoid negative numbers while using i32
                if self.stake >= self.value {
                    let sub = self.stake.saturating_sub(self.value);
                    self.stake = sub
                } else if self.stake < self.value {
                    self.stake = 0
                }
            }
            self.frames = 0;
        }
    }

    fn draw(&mut self, pico8: &mut runty8::Pico8) {
        let text = match self.digit {
            1 => "ST DIGIT",
            2 => "ND DIGIT",
            3 => "RD DIGIT",
            _ => "TH DIGIT",
        };
        pico8.cls(0);

        pico8.print("STAKING GAME", Game::WIDTH / 2 - 26, 1, 7);
        pico8.print("SEE HOW HIGH YOU CAN GO!", Game::WIDTH / 2 - 48, 11, 7);
        pico8.print(
            &format!("{}{:0>10}", "PTS:", &self.total.to_string()),
            4,
            30,
            7,
        );
        pico8.print(&format!("{:0>10}", &self.stake.to_string()), 67, 30, 7);
        pico8.print(
            &format!("{:>2}{}", &self.digit.to_string(), text),
            67,
            40,
            7,
        );
        pico8.line(self.pos, 36, self.pos + 2, 36, 8);
    }
}

impl Game {
    const WIDTH: i32 = 127;
    // const HEIGHT: i32 = 127;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::run::<Game>(assets).unwrap(); // no assets for this game
}
