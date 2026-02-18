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
        Code::new('あ', "朝日"),
        Code::new('い', "いろは"),
        Code::new('う', "上野"),
        Code::new('え', "英語"),
        Code::new('お', "大阪"),
        Code::new('か', "為替"),
        Code::new('き', "切手"),
        Code::new('く', "クラブ"),
        Code::new('け', "景色"),
        Code::new('こ', "子ども"),
        Code::new('さ', "桜"),
        Code::new('し', "新聞"),
        Code::new('す', "すずめ"),
        Code::new('せ', "世界"),
        Code::new('そ', "そろばん"),
        Code::new('た', "たばこ"),
        Code::new('ち', "ちどり"),
        Code::new('つ', "つるかめ"),
        Code::new('て', "手紙"),
        Code::new('と', "東京"),
        Code::new('な', "名古屋"),
        Code::new('に', "日本"),
        Code::new('ぬ', "沼津"),
        Code::new('ね', "ねずみ"),
        Code::new('の', "野原"),
        Code::new('は', "葉書"),
        Code::new('ひ', "飛行機"),
        Code::new('ふ', "富士山"),
        Code::new('へ', "平和"),
        Code::new('ほ', "保険"),
        Code::new('ま', "マッチ"),
        Code::new('み', "三笠"),
        Code::new('む', "無線"),
        Code::new('め', "明治"),
        Code::new('も', "もみじ"),
        Code::new('や', "大和"),
        Code::new('ゆ', "弓矢"),
        Code::new('よ', "吉野"),
        Code::new('ら', "ラジオ"),
        Code::new('り', "りんご"),
        Code::new('る', "るすい"),
        Code::new('れ', "れんげ"),
        Code::new('ろ', "ローマ"),
        Code::new('わ', "わらび"),
        Code::new('ゐ', "ゐど"),
        Code::new('ゑ', "かぎのあるゑ"),
        Code::new('を', "尾張"),
        Code::new('ん', "おしまい"),
    ]
}
