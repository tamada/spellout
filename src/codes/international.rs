use crate::{Code, PhoneticAlphabet};

pub struct International {
    codes: Vec<Code>,
}

impl International {
    pub fn new() -> Self {
        International {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for International {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Amsterdam"),
        Code::new('B', "Baltimore"),
        Code::new('C', "Casablanca"),
        Code::new('D', "Danemark"),
        Code::new('E', "Edison"),
        Code::new('F', "Florida"),
        Code::new('G', "Gallipoli"),
        Code::new('H', "Havanna"),
        Code::new('I', "Italia"),
        Code::new('J', "Jerusalem"),
        Code::new('K', "Kilogram"),
        Code::new('L', "Liverpool"),
        Code::new('M', "Madagaskar"),
        Code::new('N', "New York"),
        Code::new('O', "Oslo"),
        Code::new('P', "Paris"),
        Code::new('Q', "Québec"),
        Code::new('R', "Roma"),
        Code::new('S', "Santiago"),
        Code::new('T', "Tripoli"),
        Code::new('U', "Uppsala"),
        Code::new('V', "Valencia"),
        Code::new('W', "Washington"),
        Code::new('X', "Xanthippe"),
        Code::new('Y', "Yokohama"),
        Code::new('Z', "Zürich"),
    ]
}
