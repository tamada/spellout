use crate::{Code, PhoneticAlphabet};

pub struct France {
    codes: Vec<Code>,
}

impl France {
    pub fn new() -> Self {
        France {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for France {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('A', "Anatole"),
        Code::new('B', "Berthe"),
        Code::new('C', "Célestine"),
        Code::new('D', "Désiré"),
        Code::new('E', "Eugène"),
        Code::new('F', "François"),
        Code::new('G', "Gaston"),
        Code::new('H', "Henri"),
        Code::new('I', "Irma"),
        Code::new('J', "Joseph"),
        Code::new('K', "Kléber"),
        Code::new('L', "Louis"),
        Code::new('M', "Marcel"),
        Code::new('N', "Nicolas"),
        Code::new('O', "Oscar"),
        Code::new('P', "Pierre"),
        Code::new('Q', "Quintal"),
        Code::new('R', "Raoul"),
        Code::new('S', "Suzanne"),
        Code::new('T', "Thérèse"),
        Code::new('U', "Ursule"),
        Code::new('V', "Victor"),
        Code::new('W', "William"),
        Code::new('X', "Xavier"),
        Code::new('Y', "Yvonne"),
        Code::new('Z', "Zoé"),
    ]
}
