use std::path::Path;
use tokio::time::Instant;
use url::Url;
use xunfei_ocr::App;
use crate::download::{new_run, XFConfig};
use crate::param::{ XFData};
use crate::response::XFResponse;

mod param;
mod response;
mod download;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = Url::parse("https://gchat.qpic.cn/gchatpic_new/1713143151/1070233003-2666728377-785A0EFF27D2004484C1E3A0CDB3AE7F/0?term=2").unwrap();
    let config = XFConfig::read_config(url).unwrap();
    let file_path = Path::new(&config.path).join(&config.file_name);
    let now = Instant::now();
    new_run(&config.uri, &file_path, config.task_num).await.unwrap();
    println!("elasped time: {}", now.elapsed().as_secs_f32());
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
        Ok(r) => {
            println!("{:?}", r.info());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(())
}