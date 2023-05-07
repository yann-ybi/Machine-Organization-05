mod machine;
use crate::machine::machine::UM;
use rum::rumload;

fn main() {

    let input = std::env::args().nth(1);
    match rumload::load(input.as_deref()) {
        Some(o) => {
            let mut machine = UM::new(o);
            loop {
                machine.disassemble();
            }
        }
        None => {}
    }
    
}

