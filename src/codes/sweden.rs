use crate::{Code, PhoneticAlphabet};

pub struct Sweden {
    codes: Vec<Code>,
}

impl Sweden {
    pub fn new() -> Self {
        Sweden {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Sweden {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Adam"),
        Code::new('\u{00C4}', "Ärlig"),
        Code::new('B', "Bertil"),
        Code::new('C', "Cesar"),
        Code::new('D', "David"),
        Code::new('E', "Erik"),
        Code::new('F', "Filip"),
        Code::new('G', "Gustav"),
        Code::new('H', "Helge"),
        Code::new('I', "Ivar"),
        Code::new('J', "Johan"),
        Code::new('K', "Kalle"),
        Code::new('L', "Ludvig"),
        Code::new('M', "Martin"),
        Code::new('N', "Niklas"),
        Code::new('O', "Olof"),
        Code::new('\u{00D6}', "Östen"),
        Code::new('P', "Petter"),
        Code::new('Q', "Qvintus"),
        Code::new('R', "Rudolf"),
        Code::new('S', "Sigurd"),
        Code::new('T', "Tore"),
        Code::new('U', "Urban"),
        Code::new('V', "Viktor"),
        Code::new('W', "Wilhelm"),
        Code::new('X', "Xerxes"),
        Code::new('Y', "Yngve"),
        Code::new('Z', "Zäta"),
    ]
}
