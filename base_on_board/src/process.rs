pub struct BallCount{
    ball : u32,
    strike : u32,
}
impl BallCount{
    pub fn new() -> Self{
        Self{ ball : 0, strike : 0}
    }
}

pub struct Base{
    first : bool,
    second : bool,
    third : bool,
}
impl Base{
    pub fn new() -> Self{
        Self{ first : false, second : false, third : false}
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
}

impl Inning{
    pub fn new() -> Self{
        Self{ hit_1 : 0, hit_2 : 0, hit_3 : 0, homerun : 0, error : 0, bb : 0, runs : 0, ks : 0}
    }
}
pub struct Games{
    teamA : Vec<Option<Inning>>,
    teamB : Vec<Option<Inning>>,
}

impl Games{
    pub fn new() -> Self{
        
    }
}