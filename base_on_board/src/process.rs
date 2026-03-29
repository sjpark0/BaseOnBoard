use crate::dice::{CDice, Dice, FSub, GSub, HTDice, HitSub, PitchDice, get_c_dice, get_ht_dice, get_pitch_dice};

pub struct BallCount{
    ball : u32,
    strike : u32,
}
impl BallCount{
    pub fn new() -> Self{
        Self{ ball : 0, strike : 0}
    }
    pub fn initialize(&mut self){
        self.ball = 0;
        self.strike = 0;
    }
}

#[derive(Debug)]
pub struct Base(Vec<bool>);
impl Base{
    pub fn new() -> Self{
        Self(vec![false, false, false])
    }
    pub fn initialize(&mut self){
        self.0[0] = false;
        self.0[1] = false;
        self.0[2] = false;
    }
}

pub struct Inning{
    hit_1 : u32,
    hit_2 : u32,
    hit_3 : u32,    
    homerun : u32,
    error : u32,
    bb : u32,
    runs : u32,
    ks : u32,
    outs : u32,
}

impl Inning{
    pub fn new() -> Self{
        Self{ hit_1 : 0, hit_2 : 0, hit_3 : 0, homerun : 0, error : 0, bb : 0, runs : 0, ks : 0, outs : 0}
    }
    pub fn initialize(&mut self){
        self.hit_1 = 0;
        self.hit_2 = 0;
        self.hit_3 = 0;
        self.homerun = 0;
        self.error = 0;
        self.bb = 0;
        self.runs = 0;
        self.ks = 0;
        self.outs = 0;
    }
}

pub struct Games{
    teams : Vec<Vec<Inning>>,
    total_runs : Vec<u32>,
    ball_count : BallCount,
    base : Base,
    which_team : usize,
    which_dice : Dice,
}

impl Games{
    fn bb_advance(&mut self) -> bool{
        let mut is_finish = false;
        if self.base.0[0] == false{
            self.base.0[0] = true;
        }
        else{
            if self.base.0[1] == false{
                self.base.0[1] = true;
            }
            else{
                if self.base.0[2] == false{
                    self.base.0[2] = true;
                }
                else{
                    self.teams[self.which_team].last_mut().unwrap().runs += 1;
                    self.total_runs[self.which_team] += 1;
                    if self.which_team == 1 && self.teams[self.which_team].len() >= 9{
                        is_finish = true;
                    }
                }
            }
        }
        is_finish
    }
    fn hit_advance(&mut self, batter_adv : usize, runner_adv : usize) -> bool{
        let mut is_finish = false;
        for i in (0..self.base.0.len()).rev(){
            if self.base.0[i]{
                if i + runner_adv >= self.base.0.len(){
                    self.teams[self.which_team].last_mut().unwrap().runs += 1;
                    self.total_runs[self.which_team] += 1;
                }
                else{
                    self.base.0[i + runner_adv] = true;
                }
                self.base.0[i] = false;
            }
        }
        if self.which_team == 1 && self.teams[self.which_team].len() >= 9 && self.total_runs[1] > self.total_runs[0]{
            is_finish = true;
        }
        self.base.0[batter_adv - 1] = true;

        is_finish
    }
    fn next_inning(&mut self) -> bool{        
        let mut is_finish = false;
        self.ball_count.initialize();
        self.base.initialize();

        if self.teams[0].len() < 9{
            self.which_team = (self.which_team + 1) % 2;
            self.teams[self.which_team].push(Inning::new());
        }
        else if self.teams[1].len() == 15{
            is_finish = true;
        }
        else{
            if self.which_team == 0 && self.total_runs[1] > self.total_runs[0]{
                is_finish = true;
            }
            else if self.which_team == 1 && self.total_runs[0] != self.total_runs[1]{
                is_finish = true;
            }
            else{
                self.which_team = (self.which_team + 1) % 2;
                self.teams[self.which_team].push(Inning::new());                
            }
            
        }
        is_finish
    }
    pub fn new() -> Self{
        let teams = vec![Vec::new(), Vec::new()];
        let init_runs = vec![0, 0];
        Self{ teams : teams, total_runs : init_runs, which_team : 0, ball_count : BallCount::new(), base : Base::new(), which_dice : Dice::PitchDice}        
    }
    pub fn set_process(&mut self, command : &str) -> bool{
        let mut is_finish = false;
        if let None = self.teams[self.which_team].last(){
            self.teams[self.which_team].push(Inning::new());
        }
        println!("{:?}", self.which_dice);
        match self.which_dice{
            Dice::PitchDice =>{
                if let Some(pitch) = get_pitch_dice(command){
                    match pitch{
                        PitchDice::B => self.ball_count.ball += 1,
                        PitchDice::F => if self.ball_count.strike < 2 {self.ball_count.strike += 1},
                        PitchDice::SM => self.ball_count.strike += 1,
                        PitchDice::S => self.ball_count.strike += 1,
                        PitchDice::C => {
                            self.ball_count.initialize();
                            self.which_dice = Dice::CDice;
                        },
                    }
                    if self.ball_count.ball == 4{
                        self.ball_count.initialize();                        
                        self.which_dice = Dice::PitchDice;
                        self.teams[self.which_team].last_mut().unwrap().bb += 1;
                        is_finish = self.bb_advance();
                    }
                    else if self.ball_count.strike == 3{
                        self.ball_count.initialize();                        
                        self.which_dice = Dice::PitchDice;
                        self.teams[self.which_team].last_mut().unwrap().ks += 1;
                        self.teams[self.which_team].last_mut().unwrap().outs += 1;
                    }
                }
            },
            Dice::CDice => {
                if let Some(c_dice) = get_c_dice(command){
                    match c_dice{
                        CDice::G(g_sub) =>{
                            self.which_dice = Dice::PitchDice;
                            self.teams[self.which_team].last_mut().unwrap().outs+=1;
                            if self.teams[self.which_team].last_mut().unwrap().outs != 3{
                                match g_sub{
                                    GSub::G3 => if self.base.0[2] {
                                        self.base.0[2] = false;
                                        self.total_runs[self.which_team] += 1;
                                        self.teams[self.which_team].last_mut().unwrap().runs += 1;
                                        if self.which_team == 1 && self.teams[self.which_team].len() >= 9 && self.total_runs[1] > self.total_runs[0]{
                                            is_finish = true;
                                        }                                        
                                    },
                                    GSub::GA => {
                                        is_finish = self.hit_advance(0, 1);
                                    }
                                    _ => (),
                                }
                            }
                        },
                        CDice::F(f_sub) => {
                            self.which_dice = Dice::PitchDice;
                            self.teams[self.which_team].last_mut().unwrap().outs+=1;
                            if self.teams[self.which_team].last_mut().unwrap().outs != 3{
                                match f_sub{
                                    FSub::F2 => {
                                        if self.base.0[2] {
                                            self.base.0[2] = false;
                                            self.total_runs[self.which_team] += 1;
                                            self.teams[self.which_team].last_mut().unwrap().runs += 1;
                                            if self.which_team == 1 && self.teams[self.which_team].len() >= 9 && self.total_runs[1] > self.total_runs[0]{
                                                is_finish = true;
                                            }
                                        }
                                        if self.base.0[1] {
                                            self.base.0[1] = false;
                                            self.base.0[2] = true;
                                        }
                                    },
                                    FSub::F3 => if self.base.0[2] {
                                        self.base.0[2] = false;
                                        self.total_runs[self.which_team] += 1;
                                        self.teams[self.which_team].last_mut().unwrap().runs += 1;
                                        if self.which_team == 1 && self.teams[self.which_team].len() >= 9 && self.total_runs[1] > self.total_runs[0]{
                                            is_finish = true;
                                        }                                        
                                    },
                                    FSub::FA => {
                                        is_finish = self.hit_advance(0, 1);
                                    }
                                    _ => (),
                                }

                            }
                        },
                        CDice::PO =>{
                            self.which_dice = Dice::PitchDice;
                            self.teams[self.which_team].last_mut().unwrap().outs+=1;
                        },
                        CDice::HR => {
                            self.which_dice = Dice::PitchDice;
                            self.teams[self.which_team].last_mut().unwrap().homerun+=1;                    
                            self.hit_advance(4, 4);
                        },
                        CDice::HT => self.which_dice = Dice::HitDice,
                    }
                }                            
            },
            Dice::HitDice => {
                self.which_dice = Dice::PitchDice;                            
                if let Some(hit_dice) = get_ht_dice(command){
                    match hit_dice{
                        HTDice::Hit(adv) => {
                            self.teams[self.which_team].last_mut().unwrap().hit_1 += 1;
                            match adv{
                                HitSub::AD1 => {
                                    is_finish = self.hit_advance(1, 1);
                                },
                                HitSub::AD2 => {
                                    is_finish = self.hit_advance(1, 2);
                                },
                            }
                        },
                        HTDice::D2 => {
                            self.teams[self.which_team].last_mut().unwrap().hit_2 += 1;
                            is_finish = self.hit_advance(2, 2);
                        },
                        HTDice::D3 => {
                            self.teams[self.which_team].last_mut().unwrap().hit_2 += 1;
                            is_finish = self.hit_advance(2, 3);
                        },
                        HTDice::T3 => {
                            self.teams[self.which_team].last_mut().unwrap().hit_3 += 1;
                            is_finish = self.hit_advance(3, 3);
                        },                                                
                    }
                }
            },
        }
        
        if self.teams[self.which_team].last_mut().unwrap().outs == 3{
            self.print();
            is_finish = self.next_inning();                        
        }

        is_finish

    }
    pub fn print(&self){
        print!("     ");
        for i in 0..self.teams[0].len(){
            print!("{} ", i+1);
        }
        println!("R  H  E  B");
        print!("teamA ");
        for inn in self.teams[0].iter(){
            print!("{} ", inn.runs);
        }
        let total_h = self.teams[0].iter().fold(0, |acc, inn| acc + inn.hit_1 + inn.hit_2 + inn.hit_3 + inn.homerun);
        let total_e = self.teams[0].iter().fold(0, |acc, inn| acc + inn.error);
        let total_b = self.teams[0].iter().fold(0, |acc, inn| acc + inn.bb);
        println!("{}\t{}\t{}\t{}", self.total_runs[0], total_h, total_e, total_b);

        print!("teamB ");
        for inn in self.teams[1].iter(){
            print!("{} ", inn.runs);
        }
        let total_h = self.teams[1].iter().fold(0, |acc, inn| acc + inn.hit_1 + inn.hit_2 + inn.hit_3 + inn.homerun);
        let total_e = self.teams[1].iter().fold(0, |acc, inn| acc + inn.error);
        let total_b = self.teams[1].iter().fold(0, |acc, inn| acc + inn.bb);
        println!("{}\t{}\t{}\t{}", self.total_runs[1], total_h, total_e, total_b);
    }
}