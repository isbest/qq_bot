use std::path::Path;
use proc_qq::{
    event, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait,
    module, Module,
};
use proc_qq::re_exports::{reqwest, serde_json};
use proc_qq::re_exports::reqwest::Url;
use proc_qq::re_exports::rs_qq::client::event::GroupMessageEvent;
use xunfei_ocr::App;
use xunfei_ocr::download::{new_run, XFConfig};
use xunfei_ocr::param::XFData;
use xunfei_ocr::response::XFResponse;

/// 监听群消息
/// 使用event宏进行声明监听消息
/// 参数为rs-qq支持的任何一个类型的消息事件, 必须是引用.
/// 返回值为 anyhow::Result<bool>, Ok(true)为拦截事件, 不再向下一个监听器传递
#[event]
async fn sign_in(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if !content.contains("GroupImage") {
        return Ok(false);
    }

    let content = content.replace("[GroupImage: ", "");
    let content = content.replace("]", "");

    let url = Url::parse(content.as_str()).unwrap();
    let config = XFConfig::read_config(url).unwrap();
    let file_path = Path::new(&config.path).join(&config.file_name);
    new_run(&config.uri, &file_path, config.task_num).await.unwrap();
    let app = App::new("3f8cb891", "MWIwZWI0OWJmYjhlMjk1OGFhYjFiYTk4", "bd83a0c62c484d9171264cee049a16a3");

    let data = XFData::new(app.app_id(), &file_path);

    let client = reqwest::Client::new();

    let res = client.post(app.build_url().unwrap())
        .header("Content-type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await?
        .text()
        .await?;

    let res = serde_json::from_str::<XFResponse>(res.as_str()).unwrap();

    match res.parse() {
        Ok(ocr_result) => {
            event.send_message_to_source(ocr_result.info().to_string().parse_message_chain()).await?;
            println!("{}", "发送完成");
            Ok(true)
        }
        Err(e) => {
            println!("{}: {}", "解析失败", e);
            Ok(false)
        }
    }
}

#[event]
async fn group_sign_in(_: &GroupMessageEvent) -> anyhow::Result<bool> {
    Ok(false)
}

pub(crate) fn module() -> Module {
    module!("sign_in", "签到模块", sign_in, group_sign_in)
}
