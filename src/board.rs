#[derive(Debug)]
#[derive(Copy,Clone)]
pub struct PossField{
    pub fields: [Option<bool>;9],
    pub amount: i32
}

#[derive(Copy,Clone)]
pub struct Board {
    pub fields: [Option<u32>;81]
}

#[derive(Copy,Clone)]
pub struct BoardPoss {
    pub fields: [PossField ;81]
}

impl Board {
    pub fn new() -> Board{
        Board{
            fields:[None;81],
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
 
impl BoardPoss {
    pub fn new() -> BoardPoss{
        BoardPoss{
            fields:[PossField::new() ;81]
        }
    }
}

impl Default for BoardPoss {
    fn default() -> Self {
        Self::new()
    }
}

impl PossField {
    pub fn get(self, idx: usize) -> Option<bool>{
        self.fields[idx-1]
    }
    pub fn set(&mut self, idx: usize, val: bool){
        if idx >9{
            print!("")
        }
        self.fields[idx-1] = Some(val);
        PossField::calc_amount(self);
    }
    pub fn new() -> PossField{
        PossField { fields: [None;9],amount: 0}
    }
    pub fn combine(&mut self, other: PossField){
        for i in 0..9{
            match (self.fields[i] ,other.fields[i]){
                (Some(_),Some(_)) => self.fields[i] = Some(self.fields[i].unwrap() && other.fields[i].unwrap()),
                (None,Some(_))=> self.fields[i] = other.fields[i],
                _ => (),
            }
        }
        PossField::calc_amount(self);
    }
    fn calc_amount(&mut self){
        self.amount = 0;
        for i in 0..9{
            match self.fields[i]{
                None => (),
                _ => self.amount += 1,
            }
        }
    }
}

impl Default for PossField {
    fn default() -> Self {
        Self::new()
    }
}