use crate::{Code, PhoneticAlphabet};

pub struct Uk {
    codes: Vec<Code>,
}

impl Uk {
    pub fn new() -> Self {
        Uk {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Uk {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Able"),
        Code::new('B', "Baker"),
        Code::new('C', "Charlie"),
        Code::new('D', "Dog"),
        Code::new('E', "Easy"),
        Code::new('F', "Fox"),
        Code::new('G', "George"),
        Code::new('H', "How"),
        Code::new('I', "Item"),
        Code::new('J', "Jig"),
        Code::new('K', "King"),
        Code::new('L', "Love"),
        Code::new('M', "Mike"),
        Code::new('N', "Nan"),
        Code::new('O', "Oboe"),
        Code::new('P', "Peter"),
        Code::new('Q', "Queen"),
        Code::new('R', "Roger"),
        Code::new('S', "Samuel"),
        Code::new('T', "Tare"),
        Code::new('U', "Uncle"),
        Code::new('V', "Victor"),
        Code::new('W', "William"),
        Code::new('X', "X-ray"),
        Code::new('Y', "Yoke"),
        Code::new('Z', "Zebra"),
    ]
}
