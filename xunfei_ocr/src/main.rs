use std::path::Path;
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
    let url = Url::parse("https://gchat.qpic.cn/gchatpic_new/1713143151/522985738-2655748070-A153D83E6DEC52A9E016B889DCBDB0DE/0?term=2").unwrap();
    let config = XFConfig::read_config(url).unwrap();
    let file_path = Path::new(&config.path).join(&config.file_name);
    new_run(&config.uri, &file_path, config.task_num).await.unwrap();

    let app = App::new("3f8cb891", "MWIwZWI0OWJmYjhlMjk1OGFhYjFiYTk4", "bd83a0c62c484d9171264cee049a16a3");
    let data = XFData::new(app.app_id(), &file_path);
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&data).unwrap();
    let res = client.post(app.build_url().unwrap())
        .header("Content-type", "application/json")
        .body(body)
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