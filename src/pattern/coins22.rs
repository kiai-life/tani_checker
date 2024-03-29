use crate::config;
use crate::pattern::{Credits, CreditsData, CreditsInfo, CreditsPE, CreditsPattern};
use anyhow::Result;
use fancy_regex;
use regex::Regex;

pub fn check(config: &config::Config) -> Result<Vec<Box<dyn Credits>>> {
  let mut 専門必修実験 = CreditsInfo::new("専門必修実験", CreditsPattern::Only(6));
  let mut 専門必修卒業研究 = CreditsInfo::new("専門必修卒業研究", CreditsPattern::Only(6));
  let mut 専門必修専門語学 = CreditsInfo::new("専門必修専門語学", CreditsPattern::Only(4));
  let mut 専門選択1 = CreditsInfo::new(
    "専門選択1（GB20, GB30, GB40から始まるもの）",
    CreditsPattern::Bottom(16),
  );
  let mut 専門選択2 = CreditsInfo::new(
    "専門選択2（GB2, GB3, GB4, GA4からはじまるものと特別演習）",
    CreditsPattern::Range(0, 18),
  );
  let mut 専門基礎必修 = CreditsInfo::new("専門選択必修", CreditsPattern::Only(26));
  let mut 専門基礎選択1 =
    CreditsInfo::new("専門基礎選択1（確率論など）", CreditsPattern::Bottom(8));
  let mut 専門基礎選択2 = CreditsInfo::new(
    "専門基礎選択2 (Computer Science in English)",
    CreditsPattern::Bottom(2),
  );
  let mut 専門基礎選択3 = CreditsInfo::new(
    "専門基礎選択3（GB1から始まるもの）",
    CreditsPattern::Bottom(4),
  );
  let mut 専門基礎選択4 = CreditsInfo::new(
    "専門基礎選択4（GA1から始まるもの）",
    CreditsPattern::Bottom(8),
  );
  let mut 基礎共通必修総合科目 = CreditsInfo::new("基礎共通必修総合科目", CreditsPattern::Only(2));
  let mut 基礎体育 = CreditsPE::new("基礎体育");
  let mut 応用体育 = CreditsPE::new("応用体育");
  let mut 基礎共通必修外国語 = CreditsInfo::new("基礎共通必修外国語", CreditsPattern::Only(4));
  let mut 基礎共通必修情報 = CreditsInfo::new("基礎共通必修情報", CreditsPattern::Only(4));
  let mut 基礎共通選択1 = CreditsInfo::new(
    "基礎共通選択1（総合科目・学士基盤科目）",
    CreditsPattern::Bottom(1),
  );
  let mut 基礎共通選択2 = CreditsInfo::new(
    "基礎共通選択2（国語・芸術など）",
    CreditsPattern::Range(0, 4),
  );
  let mut 基礎関連選択1 = CreditsInfo::new("基礎関連選択1（文系）", CreditsPattern::Bottom(6));
  let mut 基礎関連選択2 = CreditsInfo::new("基礎関連選択1（理系）", CreditsPattern::Top(4));
  for class in config.class.iter().filter(|c| c.get != Some(false)) {
    let class_name = &*class.name;
    match class_name {
      "ソフトウェアサイエンス実験" => 専門必修実験.add(3),
      "ソフトウェアサイエンス実験B" => 専門必修実験.add(3),
      "情報システム実験A" => 専門必修実験.add(3),
      "情報システム実験B" => 専門必修実験.add(3),
      "知能情報メディア実験A" => 専門必修実験.add(3),
      "知能情報メディア実験B" => 専門必修実験.add(3),
      "卒業研究A" => 専門必修卒業研究.add(3),
      "卒業研究B" => 専門必修卒業研究.add(3),
      "専門語学A" => 専門必修専門語学.add(3),
      "専門語学B" => 専門必修専門語学.add(3),
      "情報科学特別演習" => 専門選択2.add(class.credits),
      "線形代数A" => 専門基礎必修.add(2),
      "線形代数B" => 専門基礎必修.add(2),
      "微分積分A" => 専門基礎必修.add(2),
      "微分積分B" => 専門基礎必修.add(2),
      "情報数学A" => 専門基礎必修.add(2),
      "専門英語基礎" => 専門基礎必修.add(1),
      "プログラミング入門A" => 専門基礎必修.add(2),
      "プログラミング入門B" => 専門基礎必修.add(1),
      "コンピュータとプログラミング" => 専門基礎必修.add(3),
      "データ構造とアルゴリズム" => 専門基礎必修.add(3),
      "データ構造とアルゴリズム実験" => 専門基礎必修.add(2),
      "論理回路" => 専門基礎必修.add(2),
      "論理回路演習" => 専門基礎必修.add(2),
      "確率論" => 専門基礎選択1.add(class.credits),
      "統計学" => 専門基礎選択1.add(class.credits),
      "数値計算法" => 専門基礎選択1.add(class.credits),
      "論理と形式化" => 専門基礎選択1.add(class.credits),
      "論理システム" => 専門基礎選択1.add(class.credits),
      "論理システム演習" => 専門基礎選択1.add(class.credits),
      "Computer Science in English A" => 専門基礎選択2.add(class.credits),
      "Computer Science in English B" => 専門基礎選択2.add(class.credits),
      "学問への誘い" => 基礎共通必修総合科目.add(class.credits),
      "ファーストイヤーセミナー" => 基礎共通必修総合科目.add(1),
      _ => {
        let re1 = Regex::new("^GB(2|3|4)0.+$").unwrap();
        let re2 = Regex::new("^GB(2|3|4).+$").unwrap();
        let re3 = Regex::new("^情報特別演習.+$").unwrap();
        let re4 = Regex::new("^GB1.+$").unwrap();
        let re5 = Regex::new("^GA1.+$").unwrap();
        let re6 = Regex::new("^1.+$").unwrap();
        // 必修体育
        let re7 = Regex::new("^2[1-7]{1}.+$").unwrap();
        // 選択体育
        let re8 = Regex::new("^2.+$").unwrap();
        // 必修英語（雑）
        let re9 = Regex::new("^31.+$").unwrap();
        // 第二外国語（雑）
        let re10 = Regex::new("^3.+$").unwrap();
        // 芸術
        let re11 = Regex::new("^4.+$").unwrap();
        // 国語
        let re12 = Regex::new("^5.+$").unwrap();
        // 必修情報
        let re13 = Regex::new("^6.+$").unwrap();
        let re21 = fancy_regex::Regex::new("^(?!(9|E|F|GC|GE|H).+$)").unwrap();
        let re22 = Regex::new("^(E|F|GC|GE|H).+$").unwrap();
        if re1.is_match(&class.id) {
          専門選択1.add(class.credits)
        } else if re2.is_match(&class.id) || re3.is_match(class_name) {
          専門選択2.add(class.credits)
        } else if re4.is_match(&class.id) {
          専門基礎選択3.add(class.credits)
        } else if re5.is_match(&class.id) {
          専門基礎選択4.add(class.credits)
        } else if re6.is_match(&class.id) {
          基礎共通選択1.add(class.credits)
        } else if re7.is_match(&class.id) {
          if class.name.starts_with('基') {
            基礎体育.get()
          }
          if class.name.starts_with('応') {
            応用体育.get()
          }
        } else if re8.is_match(&class.id) {
          基礎共通選択2.add(class.credits)
        } else if re9.is_match(&class.id) {
          基礎共通必修外国語.add(class.credits)
        } else if re10.is_match(&class.id) || re11.is_match(&class.id) || re12.is_match(&class.id) {
          基礎共通選択2.add(class.credits)
        } else if re13.is_match(&class.id) {
          基礎共通必修情報.add(class.credits)
        } else if re21.is_match(&class.id)? {
          基礎関連選択1.add(class.credits)
        } else if re22.is_match(&class.id) {
          基礎関連選択2.add(class.credits)
        }
      }
    }
  }
  let mut lst: Vec<Box<dyn Credits>> = vec![];
  lst.push(Box::new(専門必修実験.clone()));
  lst.push(Box::new(専門必修卒業研究.clone()));
  lst.push(Box::new(専門必修専門語学.clone()));

  let 専門必修 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "専門必修".to_string(),
    credits_list: vec![
      Box::new(専門必修実験),
      Box::new(専門必修卒業研究),
      Box::new(専門必修専門語学),
    ],
    pattern: CreditsPattern::Only(16),
  };
  lst.push(Box::new(専門必修.clone()));

  lst.push(Box::new(専門選択1.clone()));
  lst.push(Box::new(専門選択2.clone()));

  let 専門選択 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "専門選択".to_string(),
    credits_list: vec![Box::new(専門選択1), Box::new(専門選択2)],
    pattern: CreditsPattern::Only(34),
  };
  lst.push(Box::new(専門選択.clone()));

  lst.push(Box::new(専門基礎必修.clone()));

  lst.push(Box::new(専門基礎選択1.clone()));
  lst.push(Box::new(専門基礎選択2.clone()));
  lst.push(Box::new(専門基礎選択3.clone()));
  lst.push(Box::new(専門基礎選択4.clone()));

  let 専門基礎選択 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "専門基礎選択".to_string(),
    credits_list: vec![
      Box::new(専門基礎選択1),
      Box::new(専門基礎選択2),
      Box::new(専門基礎選択3),
      Box::new(専門基礎選択4),
    ],
    pattern: CreditsPattern::Only(26),
  };
  lst.push(Box::new(専門基礎選択.clone()));

  lst.push(Box::new(基礎共通必修総合科目.clone()));
  lst.push(Box::new(基礎共通必修外国語.clone()));
  let 基礎共通必修体育 = CreditsData {
    msg_prefix: String::new(),
    name: "基礎共通必修体育".to_string(),
    credits_list: vec![Box::new(基礎体育), Box::new(応用体育)],
    pattern: CreditsPattern::Only(2),
  };
  lst.push(Box::new(基礎共通必修体育.clone()));
  lst.push(Box::new(基礎共通必修情報.clone()));

  let 基礎共通必修 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "基礎共通必修".to_string(),
    credits_list: vec![
      Box::new(基礎共通必修総合科目),
      Box::new(基礎共通必修体育),
      Box::new(基礎共通必修外国語),
      Box::new(基礎共通必修情報),
    ],
    pattern: CreditsPattern::Only(12),
  };
  lst.push(Box::new(基礎共通必修.clone()));

  lst.push(Box::new(基礎共通選択1.clone()));
  lst.push(Box::new(基礎共通選択2.clone()));

  let 基礎共通選択 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "基礎共通選択".to_string(),
    credits_list: vec![Box::new(基礎共通選択1), Box::new(基礎共通選択2)],
    pattern: CreditsPattern::Range(1, 5),
  };
  lst.push(Box::new(基礎共通選択.clone()));

  lst.push(Box::new(基礎関連選択1.clone()));
  lst.push(Box::new(基礎関連選択2.clone()));

  let 基礎関連選択 = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "基礎関連選択".to_string(),
    credits_list: vec![Box::new(基礎関連選択1), Box::new(基礎関連選択2)],
    pattern: CreditsPattern::Range(6, 10),
  };
  lst.push(Box::new(基礎関連選択.clone()));

  let 基礎選択: CreditsData = CreditsData {
    msg_prefix: "--------- ".to_string(),
    name: "基礎選択".to_string(),
    credits_list: vec![Box::new(基礎共通選択), Box::new(基礎関連選択)],
    pattern: CreditsPattern::Top(11),
  };
  //lst.push(Box::new(基礎選択.clone()));

  let 必修 = CreditsData {
    msg_prefix: "***** ".to_string(),
    name: "必修".to_string(),
    credits_list: vec![
      Box::new(専門必修),
      Box::new(専門基礎必修),
      Box::new(基礎共通必修),
    ],
    pattern: CreditsPattern::Only(54),
  };
  lst.push(Box::new(必修.clone()));

  let 選択 = CreditsData {
    msg_prefix: "***** ".to_string(),
    name: "選択".to_string(),
    credits_list: vec![
      Box::new(専門選択),
      Box::new(専門基礎選択),
      Box::new(基礎選択),
    ],
    pattern: CreditsPattern::Only(71),
  };
  lst.push(Box::new(選択.clone()));

  let 卒業 = CreditsData {
    msg_prefix: "!!!!!!!!!! ".to_string(),
    name: "全体".to_string(),
    credits_list: vec![Box::new(必修), Box::new(選択)],
    pattern: CreditsPattern::Only(125),
  };
  lst.push(Box::new(卒業));

  Ok(lst)
}
