/// filed of a u32 bit word
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 {(1 << bits) - 1}


pub enum Op {
    CMov, Load, Stor, ADD, MULT, DIV, NAND, HALT, Map, UnMap, Output, Input, LPro, LVal, FAIL
}

/// structure containing a parsed instruction
pub struct Dinst {
    pub op: Op,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub val: u32
}

impl Dinst {
    
    /// returns a u32 bit word based on its field from an instruction word

    pub fn geta(&mut self, instruction: &u32) {
        self.a = (instruction >> RA.lsb) & mask(RA.width)
    }
    pub fn getb(&mut self, instruction: &u32) {
        self.b = (instruction >> RB.lsb) & mask(RB.width)
    }
    pub fn getc(&mut self, instruction: &u32) {
        self.c = (instruction >> RC.lsb) & mask(RC.width)
    }
    pub fn getv(&mut self, instruction: &u32) {
        self.val = (instruction >> VL.lsb) & mask(VL.width)
    }
    pub fn geta2(&mut self, instruction: &u32) {
        self.a = (instruction >> RL.lsb) & mask(RL.width)
    }

    // returns the Op
    pub fn op(&mut self, instruction: &u32) {

        match (instruction >> OP.lsb) & mask(OP.width) {
            o if o == Op::CMov as u32 => {
                self.op = Op::CMov
            },
            o if o ==  Op::Load as u32 => {
                self.op = Op::Load
            },
            o if o == Op::Stor as u32 => {
                self.op = Op::Stor
            },
            o if o ==  Op::ADD as u32 => {
                self.op = Op::ADD
            },
            o if o == Op::MULT as u32 => {
                self.op = Op::MULT
            },
            o if o ==  Op::DIV as u32 => {
                self.op = Op::DIV
            },
            o if o == Op::NAND as u32 => {
                self.op = Op::NAND
            },
            o if o ==  Op::HALT as u32 => {
                self.op = Op::HALT
            },
            o if o == Op::Map as u32 => {
                self.op = Op::Map
            },
            o if o == Op::UnMap as u32 => {
                self.op = Op::UnMap
            },
            o if o ==  Op::Output as u32 => {
                self.op = Op::Output
            },
            o if o == Op::Input as u32 => {
                self.op = Op::Input
            },
            o if o ==  Op::LPro as u32 => {
                self.op = Op::LPro
            },
            o if o == Op::LVal as u32 => {
                self.op = Op::LVal
            },
            _ =>{self.op = Op::FAIL}
        } 
    }
}

