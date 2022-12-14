use reqwest::header;
use reqwest::Client;
use polars::prelude::*;
use polars::df;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::{thread, time};
use std::mem::size_of_val;
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[tokio::getPrice]
async fn getPrice(symbol : String,minute : String) -> Result<(), Box<dyn std::error::Error>> {
    //获取数据所有SWAP的永续期货的symbol列表
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.okx.com".parse().unwrap());
    headers.insert("timeout", "10000".parse().unwrap());
    headers.insert("x-cdn", "https://static.okx.com".parse().unwrap());
    headers.insert("devid", "499c08cd-54fc-4eeb-aa94-a9a80c8454ae".parse().unwrap());
    headers.insert("accept-language", "zh-CN".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.87 Safari/537.36 SE 2.X MetaSr 1.0".parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("x-utc", "8".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("app-type", "web".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("referer", "https://www.okx.com/trade-swap/btc-usdt-swap".parse().unwrap());
    headers.insert(header::COOKIE, "locale=zh-CN; __cf_bm=exuZfzqk82n51EBCWC67ItP0HbhsjWF7.Ep.nzqYh0o-1666888648-0-Ad3IgHDl2o+MFevz5XKZ6PxNorkJ4h2rTyG2rCxxIcoM5lXeYL4Qp/1lBn3VzDPic6ziVZ0Yu/yPK0bEgmyqU5A=; defaultLocale=zh-CN; _ga_G0EKWWQGTZ=GS1.1.1666888981.1.0.1666888982.0.0.0; _tt_enable_cookie=1; _ttp=ef8b4490-aaf1-4148-a7df-4c86b66fd10e; OptanonAlertBoxClosed=2022-10-27T16:43:12.596Z; OptanonConsent=isGpcEnabled=0^&datestamp=Fri+Oct+28+2022+00^%^3A43^%^3A12+GMT^%^2B0800+(^%^E4^%^B8^%^AD^%^E5^%^9B^%^BD^%^E6^%^A0^%^87^%^E5^%^87^%^86^%^E6^%^97^%^B6^%^E9^%^97^%^B4)^&version=202208.1.0^&isIABGlobal=false^&hosts=^&consentId=a484ba32-079a-4f16-87a8-8de05d9a2eb0^&interactionCount=1^&landingPath=NotLandingPage^&groups=C0004^%^3A1^%^2CC0002^%^3A1^%^2CC0001^%^3A1; _ga=GA1.2.1624319042.1666888982; _gid=GA1.2.2022772432.1666888993; _gat_UA-35324627-3=1; amp_56bf9d=y_gFbJyJAGXrOHiys2PkMT...1ggd571t9.1ggd58600.4.0.4".parse().unwrap());

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get("https://www.okx.com/priapi/v5/market/candles?instId="+symbol+"&bar="+minute+"&after=&limit=1500&t=1666889034145")
        .headers(headers)
        .send().await?.text().await?;
    //println!("{}", res);
    //保存symbol到json文件
    fs::write(".\\price.json",res).unwrap();
    let ten_millis = time::Duration::from_millis(1);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    let f = File::open("./price.json").unwrap();
    let values:serde_json::Value = serde_json::from_reader(f).unwrap();

    println!("整个字符串：{:?}",size_of_val(&values["data"]));

    let priceList=values["data"].as_array().unwrap();
    println!("整个字符串：{:?}",priceList.len());

    println!("整个字符串：{:?}",priceList.iter());

    for price in priceList.iter() {

        println!("symbol {}", price["instId"]);

    }


    //println!("data:{}",values["data"][0]["instId"]);
    //let file = Cursor::new(res.as_str());
    //let df_01 = JsonReader::new(file)
    //   .with_batch_size(3)      //  每次读取数据批次大小
    //   .finish();



    //println!("{:?}", df_01);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //获取数据所有SWAP的永续期货的symbol列表
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.okx.com".parse().unwrap());
    headers.insert("timeout", "10000".parse().unwrap());
    headers.insert("x-cdn", "https://static.okx.com".parse().unwrap());
    headers.insert("devid", "499c08cd-54fc-4eeb-aa94-a9a80c8454ae".parse().unwrap());
    headers.insert("accept-language", "zh-CN".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.87 Safari/537.36 SE 2.X MetaSr 1.0".parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("x-utc", "8".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("app-type", "web".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("referer", "https://www.okx.com/trade-swap/btc-usdt-swap".parse().unwrap());
    headers.insert(header::COOKIE, "locale=zh-CN; __cf_bm=exuZfzqk82n51EBCWC67ItP0HbhsjWF7.Ep.nzqYh0o-1666888648-0-Ad3IgHDl2o+MFevz5XKZ6PxNorkJ4h2rTyG2rCxxIcoM5lXeYL4Qp/1lBn3VzDPic6ziVZ0Yu/yPK0bEgmyqU5A=; defaultLocale=zh-CN; _ga_G0EKWWQGTZ=GS1.1.1666888981.1.0.1666888982.0.0.0; _tt_enable_cookie=1; _ttp=ef8b4490-aaf1-4148-a7df-4c86b66fd10e; OptanonAlertBoxClosed=2022-10-27T16:43:12.596Z; OptanonConsent=isGpcEnabled=0^&datestamp=Fri+Oct+28+2022+00^%^3A43^%^3A12+GMT^%^2B0800+(^%^E4^%^B8^%^AD^%^E5^%^9B^%^BD^%^E6^%^A0^%^87^%^E5^%^87^%^86^%^E6^%^97^%^B6^%^E9^%^97^%^B4)^&version=202208.1.0^&isIABGlobal=false^&hosts=^&consentId=a484ba32-079a-4f16-87a8-8de05d9a2eb0^&interactionCount=1^&landingPath=NotLandingPage^&groups=C0004^%^3A1^%^2CC0002^%^3A1^%^2CC0001^%^3A1; _ga=GA1.2.1624319042.1666888982; _gid=GA1.2.2022772432.1666888993; _gat_UA-35324627-3=1; amp_56bf9d=y_gFbJyJAGXrOHiys2PkMT...1ggd571t9.1ggd58600.4.0.4".parse().unwrap());

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get("https://www.okx.com/priapi/v5/market/sort-tickers?t=1666889034145&order=desc&instType=SWAP&sortField=u8change")
        .headers(headers)
        .send().await?.text().await?;
    //println!("{}", res);
    //保存symbol到json文件
    fs::write(".\\symbol.json",res).unwrap();
    let ten_millis = time::Duration::from_millis(1);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    let f = File::open("./symbol.json").unwrap();
    let values:serde_json::Value = serde_json::from_reader(f).unwrap();

    println!("整个字符串：{:?}",size_of_val(&values["data"]));

    let symbolList=values["data"].as_array().unwrap();
    println!("整个字符串：{:?}",symbolList.len());

    println!("整个字符串：{:?}",symbolList.iter());

    for symbol in symbolList.iter() {

        println!("symbol {}", symbol["instId"]);

    }


    //println!("data:{}",values["data"][0]["instId"]);
    //let file = Cursor::new(res.as_str());
    //let df_01 = JsonReader::new(file)
     //   .with_batch_size(3)      //  每次读取数据批次大小
     //   .finish();



    //println!("{:?}", df_01);
    Ok(())
}