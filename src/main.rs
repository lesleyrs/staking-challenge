use runty8::{rnd, App, Button};

struct Game {
    total: u8,
    points: i32,
    stake: i32,
    frames: i32,
    delay: i32,
    digit: u8,
    pos: i32,
    value: i32,
    hiscore: i32,
    stacks: u8,
    rng: u8,
    game_over: bool,
    anim_time: i32,
    last_stake: i32,
    stake_num: i32,
    anim_num: i32,
}

impl App for Game {
    fn init(pico8: &mut runty8::Pico8) -> Self {
        pico8.set_title("Staking Challenge".to_owned());
        Self {
            total: 0,
            points: 1000000000,
            stake: 0,
            frames: 0,
            delay: 4,
            digit: 10,
            pos: 112,
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
        // vfx for adding to total+points "stack" + pset fireworks?

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
                        self.pos = 112;
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
                        self.pos = 76;
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
                if add >= self.points {
                    self.stake = self.points
                } else if self.points > self.stake + self.value {
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
                // not exactly 50/50 as it rounds float up halfway
                // flr() favors lower ints because it still generates a float
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
                    63 - 12 - self.last_stake.to_string().len() as i32,
                    50 + self.anim_time,
                    8,
                ),
                1 => pico8.print(
                    &format!("{}{}", "WIN +", self.last_stake),
                    63 - 10 - self.last_stake.to_string().len() as i32,
                    50 + self.anim_time,
                    11,
                ),
                // add text vfx
                _ => pico8.print("INCREASE STAKE AND PRESS X", 63 - 52, 63 - 8, 15),
            };
        }
        if self.anim_time == 30 {
            self.anim_time = 0;
            self.anim_num = self.stake_num;
        }

        pico8.print("STAKING CHALLENGE", 63 - 36, 4, 15);
        pico8.print("SEE HOW HIGH YOU CAN GO!", 63 - 48, 14, 9);
        pico8.rect(74, 28, 116, 46, 5);
        pico8.print(&format!("{:0>10}", self.stake), 76, 30, 7);
        pico8.print(&format!("{:>2}{}", self.digit, text), 76, 40, 9);
        pico8.print(&format!("{}{}", "COUNTER:", self.stake_num), 4, 44, 7);
        pico8.line(self.pos, 36, self.pos + 2, 36, 8);

        // match game difficulty and rename these
        pico8.print(&format!("{}{:0>10}", "POINTS:", self.points), 4, 24, 7);
        pico8.print(&format!("{}{}", "STACKS:", self.total), 4, 34, 7);
        pico8.print(&format!("{:0>10}{}", self.hiscore, " POINTS + "), 4, 122, 7);
        pico8.print(&format!("{:0>3}{}", self.stacks, " STACKS"), 85, 122, 7);
        pico8.print("PERSONAL BEST", 63 - 26, 112, 9);
        // if self.game_over {
        //     pico8.cls(15);
        //     // add text vfx
        //     pico8.print("PRESS X TO RESTART", 63 - 36, 50, 10);
        // }
    }
}

impl Game {
    // const WIDTH: i32 = 127;
    // const HEIGHT: i32 = 127;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::run::<Game>(assets).unwrap(); // no assets for this game
}
