use crate::{Code, PhoneticAlphabet};

pub struct Nato {
    codes: Vec<Code>,
}

impl Nato {
    pub fn new() -> Self {
        Nato {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Nato {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Alpha"),
        Code::new('\u{00C4}', "Ärger"),
        Code::new('B', "Bravo"),
        Code::new('C', "Charlie"),
        Code::new('D', "Delta"),
        Code::new('E', "Echo"),
        Code::new('F', "Foxtrot"),
        Code::new('G', "Golf"),
        Code::new('H', "Hotel"),
        Code::new('I', "India"),
        Code::new('J', "Juliett"),
        Code::new('K', "Kilo"),
        Code::new('L', "Lima"),
        Code::new('M', "Mike"),
        Code::new('N', "November"),
        Code::new('O', "Oscar"),
        Code::new('\u{00D6}', "Öser"),
        Code::new('P', "Papa"),
        Code::new('Q', "Quebec"),
        Code::new('R', "Romeo"),
        Code::new('S', "Sierra"),
        Code::new('T', "Tango"),
        Code::new('U', "Uniform"),
        Code::new('\u{00DC}', "Übel"),
        Code::new('V', "Victor"),
        Code::new('W', "Whiskey"),
        Code::new('X', "X-ray"),
        Code::new('Y', "Yankee"),
        Code::new('Z', "Zulu"),
        Code::new('0', "Zero"),
        Code::new('1', "One"),
        Code::new('2', "Two"),
        Code::new('3', "Three"),
        Code::new('4', "Four"),
        Code::new('5', "Five"),
        Code::new('6', "Six"),
        Code::new('7', "Seven"),
        Code::new('8', "Eight"),
        Code::new('9', "Nine"),
    ]
}
