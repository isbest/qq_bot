use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use proc_qq::Authentication::{QRCode, UinPassword};
use proc_qq::ClientBuilder;

mod sign_in;

/// 启动并使用为二维码登录
#[tokio::main]
async fn main() {
    // 初始化日志打印
    init_tracing_subscriber();
    // 使用builder创建
    ClientBuilder::new()
        .priority_session("session.token") // 默认使用session.token登录
        // .device(JsonFile("device.json")) // 设备默认值
        .authentication(QRCode) // 若不成功则使用二维码登录
        .build(vec![sign_in::module()]) // 您可以注册多个模块
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("rs_qq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                // 这里改成自己的crate名称
                .with_target("proc_qq_examples", Level::DEBUG),
        )
        .init();
}
