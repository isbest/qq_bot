use proc_qq::re_exports::reqwest::Url;
use proc_qq::re_exports::rq_engine::msg::elem::RQElem;
use proc_qq::re_exports::rs_qq::msg::elem::GroupImage;

use proc_qq::re_exports::{reqwest, serde_json};
use proc_qq::{
    event, module, MessageChainParseTrait, MessageContentTrait, MessageEvent,
    MessageSendToSourceTrait, Module,
};

use std::path::Path;
use xunfei_ocr::download::{new_run, XFConfig};
use xunfei_ocr::param::XFData;
use xunfei_ocr::response::XFResponse;
use xunfei_ocr::App;

/// 监听群消息
/// 使用event宏进行声明监听消息
/// 参数为rs-qq支持的任何一个类型的消息事件, 必须是引用.
/// 返回值为 anyhow::Result<bool>, Ok(true)为拦截事件, 不再向下一个监听器传递
#[event]
async fn sign_in(event: &MessageEvent) -> anyhow::Result<bool> {

    if !event.is_group_message() {
        return Ok(false);
    }

    let elements = event.elements();

    let mut group_image: Option<GroupImage> = None;

    for x in elements {
        match x {
            RQElem::GroupImage(gm) => {
                group_image = Some(gm);
                break;
            }
            _ => (),
        }
    }

    let content = event.message_content();

    if !content.contains("GroupImage") {
        return Ok(false);
    }

    if group_image.is_none() {
        return Ok(false);
    }

    let url = group_image.unwrap().url();

    let url = Url::parse(&url)?;
    let config = XFConfig::read_config(url).unwrap();
    let file_path = Path::new(&config.path).join(&config.file_name);

    new_run(&config.uri, &file_path, config.task_num)
        .await
        .unwrap();

    // 注意替换
    let app = App::new("app_id", "app_secret", "app_key");

    let data = XFData::new(app.app_id(), &file_path);
    let client = reqwest::Client::new();

    let res = client
        .post(app.build_url().unwrap())
        .header("Content-type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await?
        .text()
        .await?;

    let res = serde_json::from_str::<XFResponse>(res.as_str()).unwrap();

    match res.parse() {
        Ok(ocr_result) => {
            event
                .send_message_to_source(ocr_result.info().to_string().parse_message_chain())
                .await?;
            println!("{}", "发送完成");
            Ok(true)
        }
        Err(_e) => {
            println!("{}: {:#?}", "解析失败", &res.text());
            Ok(false)
        }
    }
}

pub(crate) fn module() -> Module {
    module!("sign_in", "签到模块", sign_in)
}
