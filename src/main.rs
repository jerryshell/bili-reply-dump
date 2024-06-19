use anyhow::Result;
use bili_reply_dump::model;
use reqwest::header;
use std::{env, time::Duration};
use tokio::{
    fs,
    io::{AsyncWriteExt, BufWriter},
    time::sleep,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let sessdata = match env::var("SESSDATA") {
        Ok(sessdata) => sessdata,
        Err(_) => {
            tracing::error!("SESSDATA not found");
            return Ok(());
        }
    };
    if sessdata.is_empty() {
        tracing::error!("SESSDATA is empty");
        return Ok(());
    }
    tracing::info!("{:#?}", sessdata);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        format!("SESSDATA={}", sessdata).parse().unwrap(),
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut list = vec![];
    let mut pn = 1;

    loop {
        tracing::info!("{:#?}", pn);

        let response_root = get_root(&client, pn).await;
        if response_root.is_err() {
            tracing::error!("{:#?}", response_root.unwrap_err());
            break;
        }

        match response_root.unwrap().data.list {
            None => break,
            Some(mut res_list) => list.append(&mut res_list),
        }
        pn += 1;
        sleep(Duration::from_millis(1000)).await;
    }
    // tracing::info!("{:#?}", list);

    let file = fs::File::create("replylist.json").await?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(serde_json::to_string_pretty(&list)?.as_bytes())
        .await?;
    writer.flush().await?;

    Ok(())
}

async fn get_root(client: &reqwest::Client, pn: i32) -> Result<model::reply_response::Root> {
    let url = format!("https://api.bilibili.com/x/v2/reply/up/fulllist?order=1&filter=-1&type=1&bvid=&pn={}&ps=20&charge_plus_filter=false", pn);
    tracing::info!("{:#?}", url);

    let res = client
        .get(url)
        .send()
        .await?
        .json::<model::reply_response::Root>()
        .await?;
    // tracing::info!("{:#?}", res);

    Ok(res)
}
