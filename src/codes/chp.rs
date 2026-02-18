use crate::{Code, PhoneticAlphabet};

pub struct Chp {
    codes: Vec<Code>,
}

impl Chp {
    pub fn new() -> Self {
        Chp {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Chp {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Adam"),
        Code::new('B', "Baker"),
        Code::new('C', "Charlie"),
        Code::new('D', "David"),
        Code::new('E', "Edward"),
        Code::new('F', "Frank"),
        Code::new('G', "George"),
        Code::new('H', "Henry"),
        Code::new('I', "Ida"),
        Code::new('J', "John"),
        Code::new('K', "King"),
        Code::new('L', "Lewis"),
        Code::new('M', "Mary"),
        Code::new('N', "Nancy"),
        Code::new('O', "Otto"),
        Code::new('P', "Peter"),
        Code::new('Q', "Queen"),
        Code::new('R', "Robert"),
        Code::new('S', "Sugar"),
        Code::new('T', "Thomas"),
        Code::new('U', "Uniform"),
        Code::new('V', "Victor"),
        Code::new('W', "William"),
        Code::new('X', "X-ray"),
        Code::new('Y', "Young"),
        Code::new('Z', "Zebra"),
    ]
}
