#![allow(dead_code)]

use std::fmt;
use std::fmt::Formatter;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Header<'a> {
    code: usize,
    message: &'a str,
    sid: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<'a> {
    #[serde(borrow)]
    result: Res<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Res<'a> {
    compress: &'a str,
    encoding: &'a str,
    format: &'a str,
    text: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XFResponse<'a> {
    #[serde(borrow)]
    header: Header<'a>,
    payload: Payload<'a>,
}

impl<'a> XFResponse<'a> {
    pub fn headers(&self) -> &Header {
        &self.header
    }

    pub fn response_body(&self) -> &Res {
        &self.payload.result
    }

    pub fn text(&self) -> Option<String> {
        match base64::decode(self.response_body().text) {
            Ok(buf) => {
                match String::from_utf8(buf) {
                    Ok(str) => Some(str),
                    Err(_) => None
                }
            }
            Err(e) => {
                eprintln!("parse response error. {}", e);
                None
            }
        }
    }

    pub fn parse(&self) -> Result<OcrResult, serde_json::Error> {
        if let Some(content) = self.text() {
            serde_json::from_str::<OcrResult>(content.as_str())
        } else {
            panic!("deserialize error");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcrResult {
    /// 附加信息
    category: String,
    /// 引擎版本号
    version: String,
    /// 页面集合
    pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    /// 文本行集合
    lines: Vec<Lines>,
    /// 正常返回 0 异常返回-1
    exception: isize,
    /// 图像的旋转角度
    angle: f64,
    /// 页面高度
    height: usize,
    /// 页面宽度
    width: usize,
}

/// 文本行
#[derive(Serialize, Deserialize, Debug)]
struct Lines {
    coord: [Coord; 4],
    exception: isize,
    words: Option<Vec<UnitWord>>,
    conf: f64,
    word_units: Option<Vec<UnitWord>>,
}

/// 座标
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

/// 单个单词
#[derive(Serialize, Deserialize, Debug)]
struct UnitWord {
    content: String,
    conf: f64,
    coord: [Coord; 4],
    center_point: Option<Coord>,
}

impl OcrResult {
    pub fn info(&self) -> SignInRecord {
        let mut time = None;
        let mut name = None;
        let mut s_id = None;
        for page in &self.pages {
            if page.exception == 0 {
                for line in &page.lines {
                    if line.exception == 0 {
                        if line.words.as_ref().is_none() {
                            continue;
                        }
                        for word in line.words.as_ref().unwrap() {
                            if word.content.contains("当前时间") {
                                time = Some(line.line_content());
                                continue;
                            }
                            if word.content.contains("姓名") {
                                name = Some(line.line_content());
                                continue;
                            }
                            if word.content.contains("学号") {
                                s_id = Some(line.line_content());
                                continue;
                            }
                        }
                    }
                }
            }
        }

        SignInRecord::from(name, s_id, time)
    }
}

impl Lines {
    fn line_content(&self) -> String {
        let mut content = String::new();

        for word in self.words.as_ref().unwrap() {
            if !content.contains("·") {
                content.push(' ');
            }
            content.push_str(word.content.as_str().trim());
        }

        String::from(content.trim())
    }
}


#[derive(Debug)]
pub struct SignInRecord {
    name: Option<String>,
    s_id: Option<usize>,
    time: Option<DateTime<Utc>>,
}


impl SignInRecord {
    fn from(name: Option<String>, s_id: Option<String>, time: Option<String>) -> Self {
        SignInRecord {
            name: if let Some(name) = name {
                Some(name.replace("·姓名：", ""))
            } else {
                None
            },
            s_id: if let Some(s_id) = s_id {
                let s_id = s_id.replace("·学号：", "");
                if let Ok(s_id) = usize::from_str_radix(s_id.as_str(), 10) {
                    Some(s_id)
                } else {
                    None
                }
            } else {
                None
            },
            time: if let Some(date) = time {
                let mut date = date.replace("当前时间：", "");
                // 东八区
                date.push_str(" +08:00");

                match DateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S %z") {
                    Ok(date) => Some(DateTime::<Utc>::from(date)),
                    Err(_) => None
                }
            } else {
                None
            },
        }
    }
}

impl fmt::Display for SignInRecord {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "姓名: {}\n学号: {}\n签到时间: {}", self.name.as_ref().unwrap(), self.s_id.as_ref().unwrap(), self.time.as_ref().unwrap().format("%Y-%m-%d").to_string()) // e.g. `2014-11-28T12:45:59.324310806Z`
    }
}