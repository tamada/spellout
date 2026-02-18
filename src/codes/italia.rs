use crate::{Code, PhoneticAlphabet};

pub struct Italia {
    codes: Vec<Code>,
}

impl Italia {
    pub fn new() -> Self {
        Italia {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Italia {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Ancona"),
        Code::new('B', "Bari"),
        Code::new('C', "Como"),
        Code::new('D', "Domodossola"),
        Code::new('E', "Empoli"),
        Code::new('F', "Firenze"),
        Code::new('G', "Genova"),
        Code::new('H', "Hotel"),
        Code::new('I', "Imola"),
        Code::new('J', "Juventus"),
        Code::new('K', "Kilometro"),
        Code::new('L', "Livorno"),
        Code::new('M', "Milano"),
        Code::new('N', "Napoli"),
        Code::new('O', "Otranto"),
        Code::new('P', "Pisa"),
        Code::new('Q', "Quadro"),
        Code::new('R', "Romeo"),
        Code::new('S', "Savona"),
        Code::new('T', "Torino"),
        Code::new('U', "Udine"),
        Code::new('V', "Venezia"),
        Code::new('W', "Vu Doppia"),
        Code::new('X', "Xilofono"),
        Code::new('Y', "Ipsilon"),
        Code::new('Z', "Zara"),
    ]
}
