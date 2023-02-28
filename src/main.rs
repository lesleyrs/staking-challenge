use runty8::{rnd, App, Button};

struct Game {
    stacks: u8,
    points: i32,
    stake: i32,
    frames: i32,
    delay: i32,
    digit: u8,
    pos: i32,
    value: i32,
    max_points: i32,
    max_stacks: u8,
    rng: u8,
    game_over: bool,
    anim_time: i32,
    last_stake: i32,
    stake_num: i32,
    anim_num: i32,
    investing: bool,
    invested: i32,
    income: i32,
    msg_timer: u8,
    show_msg: bool,
    arrow_pos: i32,
    lose_msg: bool,
    start_timer: u8,
    chall_points: i32,
    chall_turns: i32,
    limitless: bool,
}

impl App for Game {
    fn init(pico8: &mut runty8::Pico8) -> Self {
        pico8.set_title(Game::TITLE.to_owned());
        Self {
            stacks: 0,
            points: 0,
            stake: 0,
            frames: 0,
            delay: 4,
            digit: 1,
            pos: Game::FIRST_DIGIT,
            value: 0,
            max_points: 0,
            max_stacks: 0,
            rng: 2, // useful for showing a message without titlescreen for matching _
            game_over: true,
            last_stake: 0,
            anim_time: 0,
            stake_num: 0,
            anim_num: 0,
            investing: false,
            invested: 0,
            income: 0,
            msg_timer: 0,
            show_msg: false,
            arrow_pos: 65,
            lose_msg: false,
            start_timer: 0,
            chall_points: 0,
            chall_turns: 0,
            limitless: false,
        }
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        self.frames += 1;
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
            if !self.game_over
                && self.start_timer == 10
                && self.anim_num == self.stake_num
                && !self.investing
                && self.stake != 0
                && pico8.btn(Button::Cross)
            {
                // not exactly 50/50 as it rounds float up halfway
                // flr() favors lower ints because it still generates a float
                self.rng = rnd(1.0).round() as u8;
                if self.rng == 1 {
                    let (mut a, b) = self.points.overflowing_add(self.stake);
                    if b {
                        self.stacks += 1;
                        a -= i32::MIN;
                        // +1 is needed because overflowing takes 1 to reach 0
                        a += 1;
                    }
                    self.points = a;
                }
                if self.rng == 0 {
                    self.points -= self.stake;
                }
                if self.points == 0 && self.stacks == 0 && self.invested == 0 {
                    self.lose_msg = true;
                }
                self.last_stake = self.stake;
                self.stake = 0;
                self.stake_num += 1;
                self.frames = 0;
            }
        }
        // avoid staking by setting stake to 0 before setting investing to false
        if !self.game_over
            && self.anim_num == self.stake_num
            && self.investing
            && pico8.btnp(Button::Cross)
        {
            match self.invested {
                0 => {
                    if self.stake < self.points || self.stacks > 0 {
                        // add actual income
                        let stake = self.stake;
                        self.invested += stake;
                        self.points -= stake;
                        self.stake = 0;
                        self.investing = false;
                    }
                }
                _ => {
                    let (mut a, b) = self.points.overflowing_add(self.invested);
                    if b {
                        self.stacks += 1;
                        a -= i32::MIN;
                        // +1 is needed because overflowing takes 1 to reach 0
                        a += 1;
                    }
                    self.points = a;
                    self.invested = 0;
                }
            }
        }
        // input is not really disabled here for adding to stake but can't increase from 0
        if self.game_over {
            match self.arrow_pos {
                Game::SELECT_FIRST => {
                    if pico8.btnp(Button::Up) {
                        self.arrow_pos = Game::SELECT_LAST;
                    }
                }
                _ => {
                    if pico8.btnp(Button::Up) {
                        self.arrow_pos -= 10;
                    }
                }
            }
            match self.arrow_pos {
                Game::SELECT_LAST => {
                    if pico8.btnp(Button::Down) {
                        self.arrow_pos = Game::SELECT_FIRST;
                    }
                }
                _ => {
                    if pico8.btnp(Button::Down) {
                        self.arrow_pos += 10;
                    }
                }
            }
        }
        if !self.lose_msg && self.start_timer < 10 {
            self.start_timer += 1;
        }
        if self.game_over && pico8.btnp(Button::Cross) && self.anim_num == self.stake_num {
            self.game_over = false;
            self.stacks = 0;
            self.digit = 1;
            self.pos = Game::FIRST_DIGIT;
            self.invested = 0;
            self.income = 0;
            self.stake_num = 0;
            self.anim_num = 0;
            self.lose_msg = false;
            self.start_timer = 0;

            // add challenges based on difficulty
            match self.arrow_pos {
                Game::SELECT_FIRST => {
                    self.points = 1000000000;
                }
                75 => {
                    self.points = 1000000;
                }
                85 => {
                    self.points = 1000;
                }
                Game::SELECT_LAST => {
                    self.limitless = true;
                    self.points = 25;
                }
                _ => unreachable!(),
            }
        }
        if pico8.btnp(Button::Circle) && !self.game_over {
            self.investing = !self.investing;
        }
        if self.points == 0 {
            if self.stacks > 0 {
                self.stacks -= 1;
                self.points = i32::MAX;
            } else if self.invested > 0 {
                self.points = self.invested;
                self.invested = 0;
            } else {
                self.limitless = false;
                self.game_over = true;
            }
        }
        if self.limitless {
            if self.stacks > self.max_stacks {
                self.max_stacks = self.stacks;
                self.max_points = self.points
            } else if self.stacks == self.max_stacks && self.points > self.max_points {
                self.max_points = self.points
            }
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
        if pico8.btnp(Button::Cross)
            && self.investing
            && self.invested == 0
            && self.stacks == 0
            && self.stake == self.points
            && !self.show_msg
        {
            self.show_msg = true;
        }
        if self.show_msg {
            self.msg_timer += 1;
            pico8.print("TOO MUCH!", Game::FIRST_DIGIT, 43, 7);
            if self.msg_timer == 30 {
                self.msg_timer = 0;
                self.show_msg = false;
            }
        }
        if self.lose_msg {
            pico8.print("GAME OVER!", Game::FIRST_DIGIT, 53, 8);
        }
        if self.anim_num != self.stake_num && self.stake_num > 0 {
            self.anim_time += 1;
            match self.rng {
                0 => pico8.print(
                    &format!("{}{}", "LOSE -", self.last_stake),
                    63 - 12 - self.last_stake.to_string().len() as i32,
                    63 + self.anim_time,
                    8,
                ),
                1 => pico8.print(
                    &format!("{}{}", "WIN +", self.last_stake),
                    63 - 10 - self.last_stake.to_string().len() as i32,
                    63 + self.anim_time,
                    11,
                ),
                _ => unreachable!(),
            };
        }
        if self.anim_time == 30 {
            self.anim_time = 0;
            self.anim_num = self.stake_num;
        }

        // 63 - (text length * 2) to center text horizontically
        // 63 - 3 to center text vertically

        // set this to current challenge instead of title?
        pico8.print(
            &Game::TITLE.to_uppercase(),
            63 - (Game::TITLE.len() * 2) as i32,
            4,
            15,
        );
        pico8.print(
            &format!("{}{}", "TOTAL TURNS:", self.stake_num),
            63 - (24 + self.stake_num.to_string().len() * 2) as i32,
            14,
            7,
        );
        match self.investing {
            false => {
                pico8.rectfill(7, 23, 75, 39, 1);
                pico8.rectfill(7, 42, 75, 58, 5);
            }
            true => {
                pico8.rectfill(7, 23, 75, 39, 5);
                pico8.rectfill(7, 42, 75, 58, 1);
            }
        }
        pico8.print(&format!("{}{}", "POINTS:", self.points), 8, 24, 7);
        pico8.print(&format!("{}{}", "STACKS:", self.stacks), 8, 34, 7);
        pico8.rectfill(Game::FIRST_DIGIT - 1, 23, 118, 39, 5);
        pico8.print(&format!("{:0>10}", self.stake), Game::FIRST_DIGIT, 24, 7);
        pico8.print(
            &format!("{:>2}{}", self.digit, text),
            Game::FIRST_DIGIT,
            34,
            9,
        );
        pico8.line(self.pos, 30, self.pos + 2, 30, 9);
        pico8.print(&format!("{}{}", "INVEST:", self.invested), 8, 43, 7);
        pico8.print(&format!("{}{}", "INCOME:", self.income), 8, 53, 7);

        pico8.print("LIMITLESS PERSONAL BEST", 63 - 46, 108, 9);
        pico8.print(
            &format!("{:0>10}{}", self.max_points, " POINTS + "),
            4,
            118,
            7,
        );
        pico8.print(&format!("{:0>3}{}", self.max_stacks, " STACKS"), 85, 118, 7);
        if self.game_over && self.anim_num == self.stake_num {
            pico8.rectfill(20, 61, 107, 104, 5);
            pico8.print("EASY", 63 - 8, 65, 7);
            pico8.print("MEDIUM", 63 - 12, 75, 7);
            pico8.print("HARD", 63 - 8, 85, 7);
            pico8.print("LIMITLESS", 63 - 18, 95, 7);
            pico8.pset(38, self.arrow_pos, 9);
            pico8.pset(38, self.arrow_pos + 1, 9);
            pico8.pset(38, self.arrow_pos + 2, 9);
            pico8.pset(38, self.arrow_pos + 3, 9);
            pico8.pset(38, self.arrow_pos + 4, 9);
            pico8.pset(39, self.arrow_pos + 1, 9);
            pico8.pset(39, self.arrow_pos + 2, 9);
            pico8.pset(39, self.arrow_pos + 3, 9);
            pico8.pset(40, self.arrow_pos + 2, 9);
        }
    }
}

impl Game {
    const TITLE: &str = "Staking Challenge";
    const FIRST_DIGIT: i32 = 79;
    const LAST_DIGIT: i32 = 115;
    const SELECT_FIRST: i32 = 65;
    const SELECT_LAST: i32 = 95;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::run::<Game>(assets).unwrap(); // no assets for this game
}
