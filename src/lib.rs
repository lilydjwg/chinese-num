//! Convert a decimal number to its Chinese form.
//!
//! [![Build Status](https://travis-ci.org/lilydjwg/chinese-num.svg)](https://travis-ci.org/lilydjwg/chinese-num)
//! [![Crates.io Version](https://img.shields.io/crates/v/chinese-num.svg)](https://crates.io/crates/chinese-num)
//! [![GitHub stars](https://img.shields.io/github/stars/lilydjwg/chinese-num.svg?style=social&label=Star)](https://github.com/lilydjwg/chinese-num)
//!
//!
//! # Examples
//!
//! ```
//! let s = chinese_num::to_chinese_num("121").unwrap();
//! assert_eq!(s, "一百二十一");
//! ```
//!
//! ```
//! let s = chinese_num::to_chinese_num("1004000007000500").unwrap();
//! assert_eq!(s, "一千零四万亿零七百万零五百");
//! ```
//!
//! ```
//! let s = chinese_num::to_chinese_num("123000520").unwrap();
//! assert_eq!(s, "一亿二千三百万零五百二十");
//! ```
//!
//! ```
//! let s = chinese_num::to_chinese_num("1234070000123780000087006786520988800000").unwrap();
//! assert_eq!(s, "一千二百三十四万零七百亿零一十二万三千七百八十亿零八千七百亿六千七百八十六万五千二百零九亿八千八百八十万");
//! ```
//!
//! If the given string is not a number, or begins with "0", `None` is returned:
//!
//! ```
//! let s = chinese_num::to_chinese_num("不是数字");
//! assert!(s.is_none());
//! ```
//!
//! ```
//! let s = chinese_num::to_chinese_num("020");
//! assert!(s.is_none());
//! ```
//!
//! The algorithm is taken from here:
//! http://zhuanlan.zhihu.com/iobject/20370983.

const DIGITS: &'static str = "零一二三四五六七八九";
const TENS_NAME: &'static str = "个十百千";
const UNIT_RANK: &'static str = "个十百千万亿";

fn digit_pos_to_name(pos: usize) -> char {
  if pos == 0 {
    '个'
  } else if pos % 8 == 0 {
    '亿'
  } else if pos % 4 == 0 {
    '万'
  } else {
    TENS_NAME.chars().nth(pos % 4).unwrap()
  }
}

struct ResultS (String, bool, char);

fn append_digit(result: ResultS, tuple: (usize, char)) -> ResultS {
  let (digit, this_unit) = tuple;
  let ResultS(mut result, pending_zero, last_unit) = result;
  let this_str = DIGITS.chars().nth(digit).unwrap();
  if digit == 0 {
    if UNIT_RANK.find(last_unit).unwrap() > UNIT_RANK.find(this_unit).unwrap() {
      ResultS(result, true, last_unit)
    } else {
      result.push(this_unit);
      ResultS(result, false, this_unit)
    }
  } else {
    if pending_zero {
      result.push('零');
    }
    result.push(this_str);
    result.push(this_unit);
    ResultS(result, false, this_unit)
  }
}

pub fn to_chinese_num<N: AsRef<str>>(n: N) -> Option<String> {
  let n = n.as_ref();

  // special cases
  if n == "0" {
    return Some("零".to_owned());
  }

  // non-digit found, nothing, leading zeros
  if !n.chars().all(|x| x.is_digit(10)) || n.len() == 0
    || n.chars().nth(0).unwrap() == '0' {
    return None;
  }

  let v = n.as_bytes().iter().rev().enumerate().map(
    |(i, c)| ((c - '0' as u8) as usize, digit_pos_to_name(i)))
    .rev().fold(ResultS(String::new(), false, '个'), append_digit);

  let mut r = v.0;
  if r.chars().last().unwrap() == '个' {
    r.pop();;
  }
  if r.starts_with("一十") {
    r.remove(0);
  }
  Some(r)
}

pub trait ToChineseNum {
  fn to_chinese_num(&self) -> Option<String>;
}

impl ToChineseNum for usize {
  /// # Examples
  ///
  /// ```
  /// use chinese_num::ToChineseNum;
  ///
  /// assert_eq!(20.to_chinese_num(), Some(String::from("二十")));
  /// ```
  fn to_chinese_num(&self) -> Option<String> {
    to_chinese_num(self.to_string())
  }
}

#[test]
fn empty_number() {
  let s = to_chinese_num("");
  assert!(s.is_none());
}

#[test]
fn num_0() {
  let s = to_chinese_num("0").unwrap();
  assert_eq!(s, "零");
}

#[test]
fn num_1() {
  let s = to_chinese_num("1").unwrap();
  assert_eq!(s, "一");
}

#[test]
fn num_10() {
  let s = to_chinese_num("10").unwrap();
  assert_eq!(s, "十");
}

#[test]
fn num_12() {
  let s = to_chinese_num("12").unwrap();
  assert_eq!(s, "十二");
}

#[test]
fn num_20() {
  let s = to_chinese_num("20").unwrap();
  assert_eq!(s, "二十");
}
