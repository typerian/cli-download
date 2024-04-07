use crate::errors;
use reqwest;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn auto(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);

    let mut get_stream_body = HashMap::new();
    get_stream_body.insert("url", url);
    get_stream_body.insert("vCodec", codec);
    get_stream_body.insert("vQuality", quality);
    get_stream_body.insert("aFormat", audioformat);

    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();

    if ttwatermark == true {
        get_stream_body.insert("isNoTTWatermark", inttwm);
    }
    if fullaudio == true {
        get_stream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        get_stream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        get_stream_body.insert("dubLang", idl);
    }

    let get_stream_url = &format!("https://{apiurl}/api/json");

    if debug {
        println!(" ");
        println!("{prefix} {}", "===[ debug ]===");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", get_stream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&get_stream_body).unwrap()
        );
        println!("{prefix} {}", "===[ debug ]===");
        println!(" ");
    }

    get_stream(prefix, &get_stream_url, get_stream_body, path);
}

pub fn audio(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);

    let mut get_stream_body = HashMap::new();
    get_stream_body.insert("isAudioOnly", "true");
    get_stream_body.insert("url", url);
    get_stream_body.insert("vCodec", codec);
    get_stream_body.insert("vQuality", quality);
    get_stream_body.insert("aFormat", audioformat);

    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();

    if ttwatermark == true {
        get_stream_body.insert("isNoTTWatermark", inttwm);
    }
    if fullaudio == true {
        get_stream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        get_stream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        get_stream_body.insert("dubLang", idl);
    }

    let get_stream_url = &format!("https://{apiurl}/api/json");

    if debug {
        println!(" ");
        println!("{prefix} {}", "===[ debug ]===");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", get_stream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&get_stream_body).unwrap()
        );
        println!("{prefix} {}", "===[ debug ]===");
        println!(" ");
    }

    get_stream(prefix, &get_stream_url, get_stream_body, path);
}

#[tokio::main]
async fn get_stream(prefix: &str, url: &str, body: HashMap<&str, &str>, path: &str) {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .json(&body)
        .send()
        .await;
    let formatted_response = response
        .expect("method not found in `Result<Response, Error>`")
        .text()
        .await
        .unwrap();

    let fmtd_res2: Value = serde_json::from_str(&formatted_response).unwrap();

    if fmtd_res2.get("status").unwrap() == "stream" {
        let stream_url = fmtd_res2.get("url").unwrap().to_string();

        let stream_url: &str = &stream_url[1..stream_url.len() - 1];

        let idk: std::result::Result<(), Box<dyn Error + Send + Sync>> =
            download_from_stream(prefix, &stream_url.to_string(), path).await;
        println!("{:?}", idk);
    } else {
        errors::create_end(
            &format!(
                "{} failed to get stream url. {}",
                prefix,
                fmtd_res2.get("text").unwrap()
            )
            .as_str(),
        );
    }
}

async fn download_from_stream(prefix: &str, url: &str, path: &str) -> Result<()> {
    println!("{} got stream url. starting download...", prefix);
    let response = reqwest::get(url.to_string()).await?;
    let file_name_1 = response
        .headers()
        .get("Content-Disposition")
        .unwrap()
        .to_str()
        .ok();
    let file_name_2 = file_name_1.unwrap().strip_prefix("attachment; filename=\"");
    let file_name_3 = file_name_2.unwrap().strip_suffix("\"").unwrap();
    let full_path = format!("{}/{}", path, file_name_3);
    let mut file = std::fs::File::create(format!("{path}/{file_name_3}"))?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    println!("{} completed dowload. saved ad {}", prefix, full_path);
    Ok(())
}
