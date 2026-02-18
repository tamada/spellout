use crate::{Code, PhoneticAlphabet};

pub struct Eu {
    codes: Vec<Code>,
}

impl Eu {
    pub fn new() -> Self {
        Eu {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Eu {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Amerika"),
        Code::new('B', "Baltimore"),
        Code::new('C', "Canada"),
        Code::new('D', "Dänemark"),
        Code::new('E', "England"),
        Code::new('F', "Frankreich"),
        Code::new('G', "Guatemala"),
        Code::new('H', "Honolulu"),
        Code::new('I', "Italien"),
        Code::new('J', "Japan"),
        Code::new('K', "Kilowatt"),
        Code::new('L', "Luxemburg"),
        Code::new('M', "Mexico"),
        Code::new('N', "Norwegen"),
        Code::new('O', "Ontario"),
        Code::new('P', "Portugal"),
        Code::new('Q', "Québec"),
        Code::new('R', "Radio"),
        Code::new('S', "Santiago"),
        Code::new('T', "Texas"),
        Code::new('U', "Uruguay"),
        Code::new('V', "Venezuela"),
        Code::new('W', "Washington"),
        Code::new('X', "Xylophon"),
        Code::new('Y', "Yokohama"),
        Code::new('Z', "Zebra"),
    ]
}
