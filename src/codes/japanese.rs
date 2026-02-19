use crate::{Code, PhoneticAlphabet};

pub struct Japanese {
    codes: Vec<Code>,
}

impl Japanese {
    pub fn new() -> Self {
        Japanese {
            codes: phonetic_codes(),
        }
    }
}

impl PhoneticAlphabet for Japanese {
    fn into_entries(self) -> impl Iterator<Item = Code> {
        self.codes.into_iter()
    }
}

fn phonetic_codes() -> Vec<Code> {
    vec![
        Code::new('あ', "朝日のア"),
        Code::new('い', "いろはのイ"),
        Code::new('う', "上野のウ"),
        Code::new('え', "英語のエ"),
        Code::new('お', "大阪のオ"),
        Code::new('か', "為替のカ"),
        Code::new('き', "切手のキ"),
        Code::new('く', "クラブのク"),
        Code::new('け', "景色のケ"),
        Code::new('こ', "子どものコ"),
        Code::new('さ', "桜のサ"),
        Code::new('し', "新聞のシ"),
        Code::new('す', "すずめのス"),
        Code::new('せ', "世界のセ"),
        Code::new('そ', "そろばんのソ"),
        Code::new('た', "たばこのタ"),
        Code::new('ち', "ちどりのチ"),
        Code::new('つ', "つるかめのツ"),
        Code::new('て', "手紙のテ"),
        Code::new('と', "東京のト"),
        Code::new('な', "名古屋のナ"),
        Code::new('に', "日本の二"),
        Code::new('ぬ', "沼津のヌ"),
        Code::new('ね', "ねずみのネ"),
        Code::new('の', "野原のノ"),
        Code::new('は', "葉書のハ"),
        Code::new('ひ', "飛行機のヒ"),
        Code::new('ふ', "富士山のフ"),
        Code::new('へ', "平和のヘ"),
        Code::new('ほ', "保険のホ"),
        Code::new('ま', "マッチのマ"),
        Code::new('み', "三笠のミ"),
        Code::new('む', "無線のム"),
        Code::new('め', "明治のメ"),
        Code::new('も', "もみじのモ"),
        Code::new('や', "大和のヤ"),
        Code::new('ゆ', "弓矢のユ"),
        Code::new('よ', "吉野のヨ"),
        Code::new('ら', "ラジオのラ"),
        Code::new('り', "りんごのリ"),
        Code::new('る', "るすいのル"),
        Code::new('れ', "れんげのレ"),
        Code::new('ろ', "ローマのロ"),
        Code::new('わ', "わらびのワ"),
        Code::new('ゐ', "ゐどのヰ"),
        Code::new('ゑ', "かぎのあるヱ"),
        Code::new('を', "尾張のヲ"),
        Code::new('ん', "おしまいのン"),
        Code::new('0', "マル"),
        Code::new('1', "ヒト"),
        Code::new('2', "フタ"),
        Code::new('3', "サン"),
        Code::new('4', "ヨン"),
        Code::new('5', "ゴ"),
        Code::new('6', "ロク"),
        Code::new('7', "ナナ"),
        Code::new('8', "ハチ"),
        Code::new('9', "キュウ"),
    ]
}
