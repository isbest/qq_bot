#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Header<'a> {
    app_id: &'a str,
    status: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parameter<'a> {
    #[serde(borrow)]
    sf8e6aca1: Param<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Param<'a> {
    category: &'a str,
    result: Res<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Res<'a> {
    encoding: &'a str,
    compress: &'a str,
    format: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    sf8e6aca1_data_1: ImgData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ImgData {
    encoding: String,
    status: usize,
    image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XFData<'a> {
    #[serde(borrow)]
    header: Header<'a>,
    parameter: Parameter<'a>,
    payload: Payload,
}

impl<'a> XFData<'a> {
    pub fn new<P: 'a + AsRef<Path>>(app_id: &'a str, path: P) -> Self {
        XFData {
            header: Header::new(app_id),
            parameter: Parameter::new(),
            payload: Payload::new(path),
        }
    }
}

impl<'a> Header<'a> {
    fn new(app_id: &'a str) -> Self {
        Header {
            app_id,
            status: 3,
        }
    }
}

impl<'a> Parameter<'a> {
    fn new() -> Self {
        Parameter {
            sf8e6aca1: Param {
                category: "ch_en_public_cloud",
                result: Res::new(),
            }
        }
    }
}

impl<'a> Res<'a> {
    fn new() -> Self {
        Res {
            encoding: "utf8",
            compress: "raw",
            format: "json",
        }
    }
}

impl Payload {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Payload {
            sf8e6aca1_data_1: ImgData::new(path),
        }
    }
}

impl ImgData {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        if !Path::is_file(path.as_ref()) {
            panic!("please input a image path")
        }

        let img = match File::open(&path) {
            Ok(mut f) => {
                let mut buf: Vec<u8> = Vec::new();
                match f.read_to_end(&mut buf) {
                    Ok(_) => {
                        buf
                    }
                    Err(e) => {
                        panic!("read img fail with reason {}", e);
                    }
                }
            }
            Err(e) => {
                panic!("open image failed with reason {}", e);
            }
        };

        ImgData {
            encoding: String::from(path.as_ref().extension().unwrap().clone().to_str().unwrap()),
            status: 3,
            image: base64::encode(img),
        }
    }
}
