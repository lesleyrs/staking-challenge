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
        pico8.set_title(Game::TITLE.to_owned());
        Self {
            total: 1,
            points: 0,
            stake: 0,
            frames: 0,
            delay: 4,
            digit: 1,
            pos: Game::FIRST_DIGIT,
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
            _ => unreachable!(),
        }
        if self.frames > self.delay {
            match self.digit {
                1 => {
                    if pico8.btn(Button::Left) {
                        self.digit = 10;
                        self.pos = Game::LAST_DIGIT;
                        self.frames = 0;
                    }
                }
                _ => {
                    if pico8.btn(Button::Left) {
                        self.digit -= 1;
                        self.pos -= 4;
                        self.frames = 0;
                    }
                }
            }
            match self.digit {
                10 => {
                    if pico8.btn(Button::Right) {
                        self.digit = 1;
                        self.pos = Game::FIRST_DIGIT;
                        self.frames = 0;
                    }
                }
                _ => {
                    if pico8.btn(Button::Right) {
                        self.digit += 1;
                        self.pos += 4;
                        self.frames = 0;
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
                self.frames = 0;
            }
            if pico8.btn(Button::Down) {
                // needed to avoid negative numbers while using i32
                if self.stake >= self.value {
                    let sub = self.stake.saturating_sub(self.value);
                    self.stake = sub
                } else if self.stake < self.value {
                    self.stake = 0
                }
                self.frames = 0;
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
                self.frames = 0;
            }
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
        // add seperate hiscore for each difficulty
        if self.total > self.stacks {
            self.stacks = self.total;
            self.hiscore = self.points
        } else if self.total == self.stacks && self.points > self.hiscore {
            self.hiscore = self.points
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
                    63 - 10 + self.anim_time,
                    8,
                ),
                1 => pico8.print(
                    &format!("{}{}", "WIN +", self.last_stake),
                    63 - 10 - self.last_stake.to_string().len() as i32,
                    63 - 10 + self.anim_time,
                    11,
                ),
                // add text blink vfx
                _ => pico8.print("INCREASE STAKE AND PRESS X", 63 - 52, 63 - 10 + 15, 9),
            };
        }
        if self.anim_time == 30 {
            self.anim_time = 0;
            self.anim_num = self.stake_num;
        }

        // 63 - (text length * 2) to center text
        pico8.print(
            &Game::TITLE.to_uppercase(),
            63 - (Game::TITLE.len() * 2) as i32,
            4,
            15,
        );
        pico8.print("SEE HOW HIGH YOU CAN GO!", 63 - 48, 14, 9);
        pico8.print(
            &format!("{}{}", "TOTAL TURNS:", self.stake_num),
            63 - (24 + self.stake_num.to_string().len() * 2) as i32,
            24,
            7,
        );
        pico8.rectfill(7, 33, 75, 49, 5);
        pico8.print(&format!("{}{}", "POINTS:", self.points), 8, 34, 7);
        pico8.print(&format!("{}{}", "STACKS:", self.total), 8, 44, 7);
        pico8.rectfill(Game::FIRST_DIGIT - 1, 33, 118, 49, 5);
        pico8.print(&format!("{:0>10}", self.stake), Game::FIRST_DIGIT, 34, 7);
        pico8.print(
            &format!("{:>2}{}", self.digit, text),
            Game::FIRST_DIGIT,
            44,
            9,
        );
        pico8.line(self.pos, 40, self.pos + 2, 40, 9);

        // match game difficulty and rename these
        pico8.print("PERSONAL BEST", 63 - 26, 108, 9);
        pico8.print(&format!("{:0>10}{}", self.hiscore, " POINTS + "), 4, 118, 7);
        pico8.print(&format!("{:0>3}{}", self.stacks, " STACKS"), 85, 118, 7);
        // if self.game_over {
        //     pico8.cls(15);
        //     // add text vfx
        //     pico8.print("PRESS X TO RESTART", 63 - 36, 50, 10);
        // }
    }
}

impl Game {
    const TITLE: &str = "Staking Challenge";
    const FIRST_DIGIT: i32 = 79;
    const LAST_DIGIT: i32 = 115;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::run::<Game>(assets).unwrap(); // no assets for this game
}
