use runty8::{rnd, App, Button};

struct Game {
    total: i32,
    points: i32,
    stake: i32,
    frames: i32,
    delay: i32,
    digit: i8,
    pos: i32,
    value: i32,
    hiscore: i32,
    stacks: i32,
    rng: u8,
    game_over: bool,
    anim_time: i32,
    last_stake: i32,
    stake_num: i32,
    anim_num: i32,
}

impl App for Game {
    fn init(pico8: &mut runty8::Pico8) -> Self {
        pico8.set_title("Staking Game".to_owned());
        Self {
            total: 0,
            points: 1000000000,
            stake: 0,
            frames: 0,
            delay: 4,
            digit: 10,
            pos: 103,
            value: 1,
            hiscore: 0,
            stacks: 0,
            rng: 2,
            game_over: false,
            last_stake: 0,
            anim_time: 0,
            stake_num: 0,
            anim_num: 0,
        }
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        self.frames += 1;
        // add menu with difficulty setting + game over + rectfill
        // vfx based on points + pset?
        // animation for adding to total+points "stack"

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
                // don't allow staking more than current points
                let add = self.stake.saturating_add(self.value);
                if self.stake + self.value >= self.points {
                    self.stake = self.points
                }
                if self.points >= self.stake + self.value {
                    self.stake = add
                }
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
            if pico8.btn(Button::Cross)
                && self.stake != 0
                && !self.game_over
                && self.anim_num == self.stake_num
            {
                self.rng = rnd(1.0).round() as u8;
                if self.rng == 1 {
                    let (mut a, b) = self.points.overflowing_add(self.stake);
                    if b {
                        self.total += 1;
                        a -= i32::MIN;
                        // +1 is needed because overflowing takes 1 to reach 0
                        a += 1;
                    }
                    self.points = a;
                }
                if self.rng == 0 {
                    self.points -= self.stake
                }
                self.last_stake = self.stake;
                self.stake = 0;
                self.stake_num += 1;
                // add seperate hiscore for each difficulty
                if self.total > self.stacks {
                    self.stacks = self.total;
                    self.hiscore = self.points
                } else if self.total == self.stacks && self.points > self.hiscore {
                    self.hiscore = self.points
                }
                // add game over and menu screen
                if self.points == 0 {
                    if self.total > 0 {
                        self.total -= 1;
                        self.points = i32::MAX;
                    } else {
                        self.game_over = true
                    }
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
        if self.anim_num != self.stake_num || self.stake_num == 0 {
            if self.stake_num > 0 {
                self.anim_time += 1;
            }
            match self.rng {
                0 => pico8.print(
                    &format!("{}{}", "LOSE -", self.last_stake),
                    Game::WIDTH / 2 - 12 - self.last_stake.to_string().len() as i32,
                    50 + self.anim_time,
                    7,
                ),
                1 => pico8.print(
                    &format!("{}{}", "WIN +", self.last_stake),
                    Game::WIDTH / 2 - 10 - self.last_stake.to_string().len() as i32,
                    50 + self.anim_time,
                    7,
                ),
                _ => pico8.print("PRESS X TO STAKE", Game::WIDTH / 2 - 32, 50, 7),
            };
        }
        if self.anim_time == 30 {
            self.anim_time = 0;
            self.anim_num = self.stake_num;
        }
        pico8.print(&self.game_over.to_string().to_uppercase(), 0, 20, 7);

        pico8.print("STAKING GAME", Game::WIDTH / 2 - 26, 1, 9);
        pico8.print("SEE HOW HIGH YOU CAN GO!", Game::WIDTH / 2 - 48, 11, 9);
        pico8.print(&format!("{:0>10}", self.stake), 67, 30, 7);
        pico8.print(&format!("{:>2}{}", self.digit, text), 67, 40, 7);
        pico8.print(&format!("{}{}", "TRIES: ", self.stake_num), 24, 20, 7);
        pico8.line(self.pos, 36, self.pos + 2, 36, 8);

        // match game difficulty and rename these
        pico8.print(&format!("{}{:0>10}", "PTS:", self.points), 4, 30, 7);
        pico8.print(&format!("{}{}", "STACKS: ", self.total), 70, 20, 7);
        pico8.print(&format!("{}{:0>10}", "PTS:", self.hiscore), 10, 122, 7);
        pico8.print(&format!("{}{}", "STACKS: ", self.stacks), 80, 122, 7);
    }
}

impl Game {
    const WIDTH: i32 = 127;
    const HEIGHT: i32 = 127;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::run::<Game>(assets).unwrap(); // no assets for this game
}
