use crate::config;
use crate::pattern::{
  msg_from_credits_data, msg_from_credits_info, Credits, CreditsData, CreditsInfo, CreditsPattern,
};
use anyhow::Result;
use fancy_regex;
use regex::Regex;

pub fn check(config: &config::Config) -> Result<Vec<(String, String)>> {
  let mut 専門必修実験 = CreditsInfo {
    credits: 0.0,
    pattern: CreditsPattern::Only(6.0),
  };
  let mut 専門必修卒業研究 = CreditsInfo {
    // 卒研
    credits: 0.0,
    pattern: CreditsPattern::Only(6.0),
  };
  let mut 専門必修専門語学 = CreditsInfo {
    // 専門語学
    credits: 0.0,
    pattern: CreditsPattern::Only(4.0),
  };
  let mut 専門選択1 = 0.0;
  let mut 専門選択2 = 0.0;
  let mut 専門基礎必修 = 0;
  let mut 専門基礎選択1 = 0.0; // 確率論など
  let mut 専門基礎選択2 = 0.0; // Computer Science in English
  let mut 専門基礎選択3 = 0.0; // GB1から始まるもの
  let mut 専門基礎選択4 = 0.0; // GA1から始まるもの
  let mut 基礎共通必修総合科目 = 0;
  let mut 基礎共通必修体育 = 0.0;
  let mut 基礎共通必修外国語 = 0.0;
  let mut 基礎共通必修情報 = 0.0;
  let mut 基礎共通選択1 = 0.0; // 総合科目・学士基盤科目
  let mut 基礎共通選択2 = 0.0; // 体育・外国語・国語・芸術
  let mut 基礎関連選択1 = 0.0; // 文系科目
  let mut 基礎関連選択2 = 0.0; // 理系科目
  for class in config.class.iter() {
    let class_name = &*class.name;
    match class_name {
      "ソフトウェアサイエンス実験" => 専門必修実験.credits += 3.0,
      "ソフトウェアサイエンス実験B" => 専門必修実験.credits += 3.0,
      "情報システム実験A" => 専門必修実験.credits += 3.0,
      "情報システム実験B" => 専門必修実験.credits += 3.0,
      "知能情報メディア実験A" => 専門必修実験.credits += 3.0,
      "知能情報メディア実験B" => 専門必修実験.credits += 3.0,
      "卒業研究A" => 専門必修卒業研究.credits += 3.0,
      "卒業研究B" => 専門必修卒業研究.credits += 3.0,
      "専門語学A" => 専門必修専門語学.credits += 3.0,
      "専門語学B" => 専門必修専門語学.credits += 3.0,
      "情報科学特別演習" => 専門選択2 += class.credits,
      "線形代数A" => 専門基礎必修 += 2,
      "線形代数B" => 専門基礎必修 += 2,
      "微分積分A" => 専門基礎必修 += 2,
      "微分積分B" => 専門基礎必修 += 2,
      "情報数学A" => 専門基礎必修 += 2,
      "専門英語基礎" => 専門基礎必修 += 1,
      "プログラミング入門A" => 専門基礎必修 += 2,
      "プログラミング入門B" => 専門基礎必修 += 1,
      "コンピュータとプログラミング" => 専門基礎必修 += 3,
      "データ構造とアルゴリズム" => 専門基礎必修 += 3,
      "データ構造とアルゴリズム実験" => 専門基礎必修 += 2,
      "論理回路" => 専門基礎必修 += 2,
      "論理回路演習" => 専門基礎必修 += 2,
      "確率論" => 専門基礎選択1 += class.credits,
      "統計学" => 専門基礎選択1 += class.credits,
      "数値計算法" => 専門基礎選択1 += class.credits,
      "論理と形式化" => 専門基礎選択1 += class.credits,
      "論理システム" => 専門基礎選択1 += class.credits,
      "論理システム演習" => 専門基礎選択1 += class.credits,
      "Computer Science in English A" => 専門基礎選択2 += class.credits,
      "Computer Science in English B" => 専門基礎選択2 += class.credits,
      "学問への誘い" => 基礎共通必修総合科目 += 1,
      "ファーストイヤーセミナー" => 基礎共通必修総合科目 += 1,
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
          専門選択1 += class.credits
        } else if re2.is_match(&class.id) || re3.is_match(class_name) {
          専門選択2 += class.credits
        } else if re4.is_match(&class.id) {
          専門基礎選択3 += class.credits
        } else if re5.is_match(&class.id) {
          専門基礎選択4 += class.credits
        } else if re6.is_match(&class.id) {
          基礎共通選択1 += class.credits
        } else if re7.is_match(&class.id) {
          基礎共通必修体育 += class.credits
        } else if re8.is_match(&class.id) {
          基礎共通選択2 += class.credits
        } else if re9.is_match(&class.id) {
          基礎共通必修外国語 += class.credits
        } else if re10.is_match(&class.id) || re11.is_match(&class.id) || re12.is_match(&class.id) {
          基礎共通選択2 += class.credits
        } else if re13.is_match(&class.id) {
          基礎共通必修情報 += class.credits
        } else if re21.is_match(&class.id)? {
          基礎関連選択1 += class.credits
        } else if re22.is_match(&class.id) {
          基礎関連選択2 += class.credits
        }
      }
    }
  }
  let mut lst = vec![];
  lst.push(("専門必修実験".to_string(), {
    msg_from_credits_info(&専門必修実験)
    //if 専門必修実験.0 > 6 {
    //  専門必修実験.1 = Some(専門必修実験.0 - 6);
    //  format!("{}({})/{}", 6, 専門必修実験.0, 6)
    //} else {
    //  format!("{}/{}", 専門必修実験.0, 6)
    //}
  }));
  lst.push((
    "専門必修卒業研究".to_string(),
    msg_from_credits_info(&専門必修卒業研究)
    //if 専門必修卒業研究.0 > 6 {
    //  専門必修卒業研究.1 = Some(専門必修卒業研究.0 - 6);
    //  format!("{}({})/{}", 6, 専門必修卒業研究.0, 6)
    //} else {
    //  format!("{}/{}", 専門必修卒業研究.0, 6)
    //},
  ));
  lst.push((
    "専門必修専門語学".to_string(),
    msg_from_credits_info(&専門必修専門語学)
    //if 専門必修専門語学.0 > 4 {
    //  専門必修専門語学.1 = Some(専門必修専門語学.0 - 4);
    //  format!("{}({})/{}", 4, 専門必修専門語学.0, 4)
    //} else {
    //  format!("{}/{}", 専門必修専門語学.0, 4)
    //},
  ));
  let 専門必修 = CreditsData {
    credits_list: vec![
      Credits::CreditsInfo(専門必修実験),
      Credits::CreditsInfo(専門必修卒業研究),
      Credits::CreditsInfo(専門必修専門語学),
    ],
    pattern: CreditsPattern::Only(16.0),
  };
  lst.push((
    "----------- 専門必修".to_string(),
    msg_from_credits_data(&専門必修), //format!("{}/{}", 専門必修, 16),
  ));
  lst.push((
    "専門選択1（GB20, GB30, GB40から始まるもの）".to_string(),
    format!("{}/{}", 専門選択1, "16～"),
  ));
  lst.push((
    "専門選択2（GB2, GB3, GB4, GA4からはじまるものと特別演習）".to_string(),
    format!("{}/{}", 専門選択2, "0～18"),
  ));
  let 専門選択 = (専門選択1 + 専門選択2) as i32;
  lst.push((
    "----------- 専門選択".to_string(),
    format!("{}/{}", 専門選択, 34),
  ));
  lst.push((
    "専門基礎必修".to_string(),
    format!("{}/{}", 専門基礎必修, 26),
  ));
  lst.push((
    "----------- 専門基礎必修".to_string(),
    format!("{}/{}", 専門基礎必修, 26),
  ));
  lst.push((
    "専門基礎選択1（確率論など）".to_string(),
    format!("{}/{}", 専門基礎選択1, "8～"),
  ));
  lst.push((
    "専門基礎選択2 (Computer Science in English)".to_string(),
    format!("{}/{}", 専門基礎選択2, "2～"),
  ));
  lst.push((
    "専門基礎選択3（GB1から始まるもの）".to_string(),
    format!("{}/{}", 専門基礎選択3, "4～"),
  ));
  lst.push((
    "専門基礎選択4（GA1から始まるもの）".to_string(),
    format!("{}/{}", 専門基礎選択4, "8～"),
  ));
  let 専門基礎選択 = (専門基礎選択1 + 専門基礎選択2 + 専門基礎選択3 + 専門基礎選択4) as i32;
  lst.push((
    "----------- 専門基礎選択".to_string(),
    format!("{}/{}", 専門基礎選択, 26),
  ));
  lst.push((
    "基礎共通必修総合科目".to_string(),
    format!("{}/{}", 基礎共通必修総合科目, "2"),
  ));
  lst.push((
    "基礎共通必修体育".to_string(),
    format!("{}/{}", 基礎共通必修体育, "2"),
  ));
  lst.push((
    "基礎共通必修外国語（英語）".to_string(),
    format!("{}/{}", 基礎共通必修外国語, "4"),
  ));
  lst.push((
    "基礎共通必修情報".to_string(),
    format!("{}/{}", 基礎共通必修情報, "4"),
  ));
  let 基礎共通必修 = 基礎共通必修総合科目
    + (基礎共通必修体育 as i32)
    + (基礎共通必修外国語 as i32)
    + (基礎共通必修情報 as i32);
  lst.push((
    "----------- 基礎共通必修".to_string(),
    format!("{}/{}", 基礎共通必修, 12),
  ));
  lst.push((
    "基礎共通選択1（総合科目・学士基盤科目）".to_string(),
    format!("{}/{}", 基礎共通選択1, "1～"),
  ));
  lst.push((
    "基礎共通選択2（国語・芸術など）".to_string(),
    format!("{}/{}", 基礎共通選択2, "0～4"),
  ));
  let 基礎共通選択 = (基礎共通選択1 + 基礎共通選択2) as i32;
  lst.push((
    "----------- 基礎共通選択".to_string(),
    format!("{}/{}", 基礎共通選択, "1～5"),
  ));
  lst.push((
    "基礎関連選択1（文系）".to_string(),
    format!("{}/{}", 基礎関連選択1, "6～"),
  ));
  lst.push((
    "基礎関連選択2（理系）".to_string(),
    format!("{}/{}", 基礎関連選択2, "0～4"),
  ));
  let 基礎関連選択 = (基礎関連選択1 + 基礎関連選択2) as i32;
  lst.push((
    "----------- 基礎関連選択".to_string(),
    format!("{}/{}", 基礎関連選択, "6～10"),
  ));
  let 必修 = 専門必修 + 専門基礎必修 + 基礎共通必修;
  lst.push(("*** 必修".to_string(), format!("{}/{}", 必修, "54")));
  let 選択: i32 = 専門選択 + 専門基礎選択 + 基礎共通選択 + 基礎関連選択;
  lst.push(("*** 選択".to_string(), format!("{}/{}", 選択, "71")));
  lst.push((
    "!!!!! 全体合計 !!!!!".to_string(),
    format!("{}/{}", 必修 + 選択, "125"),
  ));
  Ok(lst)
}
