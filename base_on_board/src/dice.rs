pub enum PitchDice{
    S,
    SM,
    F,
    B,
    C,
}

pub fn make_pitch_dice() -> Vec<PitchDice>{
    let num_s = 2;
    let num_sm = 1;
    let num_f = 1;
    let num_b = 4;
    let num_c = 4;
    let mut pitch = Vec::new();
    for i in 0..num_s{
        pitch.push(PitchDice::S);
    }
    for i in 0..num_sm{
        pitch.push(PitchDice::SM);
    }
    for i in 0..num_b{
        pitch.push(PitchDice::B);
    }
    for i in 0..num_c{
        pitch.push(PitchDice::C);
    }
    pitch
}

pub fn get_pitch_dice(key : &str) -> Option<PitchDice>{
    match key{
        "s" => Some(PitchDice::S),
        "sm" => Some(PitchDice::SM),
        "f" => Some(PitchDice::F),
        "b" => Some(PitchDice::B),
        "c" => Some(PitchDice::C),
        _ => None,
    }
}
enum GSub{
    GF,
    G3,
    GA,
}
enum FSub{
    FO,
    F2,
    F3,
    FA,
}
pub enum CDice{
    G(GSub),
    PO,
    F(FSub),
    HT,
    HR,
}

pub fn make_c_dice() -> Vec<CDice>{
    let num_gf = 1;
    let num_g3 = 1;
    let num_ga = 1;
    let num_po = 1;
    let num_fo = 1;
    let num_f2 = 1;
    let num_f3 = 1;
    let num_fa = 1;
    let num_ht = 3;
    let num_hr = 1;
    
    let mut c_dice  = Vec::new();
    for i in 0..num_gf{
        c_dice.push(CDice::G(GSub::GF));
    }
    for i in 0..num_g3{
        c_dice.push(CDice::G(GSub::G3));
    }
    for i in 0..num_ga{
        c_dice.push(CDice::G(GSub::GA));
    }
    for i in 0..num_po{
        c_dice.push(CDice::PO);
    }
    for i in 0..num_fo{
        c_dice.push(CDice::F(FSub::FO));
    }
    for i in 0..num_f2{
        c_dice.push(CDice::F(FSub::F2));
    }
    for i in 0..num_f3{
        c_dice.push(CDice::F(FSub::F3));
    }
    for i in 0..num_fa{
        c_dice.push(CDice::F(FSub::FA));
    }
    for i in 0..num_ht{
        c_dice.push(CDice::HT);
    }
    for i in 0..num_hr{
        c_dice.push(CDice::HR);
    }
    c_dice
}

pub fn get_c_dice(key : &str) -> Option<CDice>{
    match key{
        "gf" => Some(CDice::G(GSub::GF)),
        "g3" => Some(CDice::G(GSub::G3)),
        "ga" => Some(CDice::G(GSub::GA)),
        "po" => Some(CDice::PO),
        "fo" => Some(CDice::F(FSub::FO)),
        "f2" => Some(CDice::F(FSub::F2)),
        "f3" => Some(CDice::F(FSub::F3)),
        "fa" => Some(CDice::F(FSub::FA)),
        "ht" => Some(CDice::HT),
        "hr" => Some(CDice::HR),        
        _ => None,
    }
}

enum HitSub{
    AD1,
    AD2,
}
pub enum HTDice{
    Hit(HitSub),
    D2,
    D3,
    T3,
}

pub fn make_ht_dice() -> Vec<HTDice>{
    let num_ad1 = 4;
    let num_ad2 = 5;
    let num_d2 = 1;
    let num_d3 = 1;
    let num_t3 = 1;
    let mut ht_dice = Vec::new();
    for i in 0..num_ad1{
        ht_dice.push(HTDice::Hit(HitSub::AD1));
    }    
    for i in 0..num_ad2{
        ht_dice.push(HTDice::Hit(HitSub::AD2));
    }
    for i in 0..num_d2{
        ht_dice.push(HTDice::D2);
    }
    for i in 0..num_d3{
        ht_dice.push(HTDice::D3);
    }
    for i in 0..num_t3{
        ht_dice.push(HTDice::T3);
    }
    ht_dice
}


pub fn get_ht_dice(key : &str) -> Option<HTDice>{
    match key{
        "ih" => Some(HTDice::Hit(HitSub::AD1)),
        "l1" => Some(HTDice::Hit(HitSub::AD1)),
        "l2" => Some(HTDice::Hit(HitSub::AD2)),
        "c1" => Some(HTDice::Hit(HitSub::AD1)),
        "c2" => Some(HTDice::Hit(HitSub::AD2)),
        "r1" => Some(HTDice::Hit(HitSub::AD1)),
        "r2" => Some(HTDice::Hit(HitSub::AD2)),
        "d2" => Some(HTDice::D2),
        "d3" => Some(HTDice::D3),
        "t3" => Some(HTDice::T3),        
        _ => None,
    }
}