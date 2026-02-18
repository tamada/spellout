use crate::{Code, PhoneticAlphabet};

pub struct English {
    codes: Vec<Code>,
}

impl English {
    pub fn new() -> Self {
        English {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for English {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Alfred"),
        Code::new('B', "Benjamin"),
        Code::new('C', "Charles"),
        Code::new('D', "David"),
        Code::new('E', "Edward"),
        Code::new('F', "Frederick"),
        Code::new('G', "George"),
        Code::new('H', "Harry"),
        Code::new('I', "Isaac"),
        Code::new('J', "Jack"),
        Code::new('K', "King"),
        Code::new('L', "London"),
        Code::new('M', "Mary"),
        Code::new('N', "Nellie"),
        Code::new('O', "Oliver"),
        Code::new('P', "Peter"),
        Code::new('Q', "Queen"),
        Code::new('R', "Robert"),
        Code::new('S', "Samuel"),
        Code::new('T', "Tommy"),
        Code::new('U', "Uncle"),
        Code::new('V', "Victor"),
        Code::new('W', "William"),
        Code::new('X', "X-ray"),
        Code::new('Y', "Yellow"),
        Code::new('Z', "Zebra"),
    ]
}
