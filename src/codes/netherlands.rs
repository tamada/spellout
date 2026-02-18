use crate::{Code, PhoneticAlphabet};

pub struct Netherlands {
    codes: Vec<Code>,
}

impl Netherlands {
    pub fn new() -> Self {
        Netherlands {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Netherlands {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Anna"),
        Code::new('B', "Bernard"),
        Code::new('C', "Cornelis"),
        Code::new('D', "Dirk"),
        Code::new('E', "Eduard"),
        Code::new('F', "Ferdinand"),
        Code::new('G', "Gerard"),
        Code::new('H', "Hendrik"),
        Code::new('I', "Izaak"),
        Code::new('J', "Jan"),
        Code::new('K', "Karel"),
        Code::new('L', "Lodewijk"),
        Code::new('M', "Marie"),
        Code::new('N', "Nico"),
        Code::new('O', "Otto"),
        Code::new('P', "Pieter"),
        Code::new('Q', "Quotiënt"),
        Code::new('R', "Rudolf"),
        Code::new('S', "Simon"),
        Code::new('T', "Teunis"),
        Code::new('U', "Utrecht"),
        Code::new('V', "Victor"),
        Code::new('W', "Willem"),
        Code::new('X', "Xanthippe"),
        Code::new('Y', "Ypsilon"),
        Code::new('\u{0132}', "IJmuiden"),
        Code::new('Z', "Zaandam"),
    ]
}
