use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();

    let v: Value = serde_json::from_str(s.as_str()).unwrap();
    v.get("setting").unwrap().to_string()
}

async fn read_and_parse_2(file_name: &str) -> Result<String, &str> {
    let mut f = File::open(file_name).await.map_err(|_| "File not found")?;

    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.map_err(|_| "Error while reading file")?;
    let s = String::from_utf8(buf).map_err(|_| "Error while converting to string")?;

    let v: Value = serde_json::from_str(s.as_str()).map_err(|_| "Error while parsing JSON")?;
    match v.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err("Setting not found"),
    }
}

async fn read_and_parse_3(file_name: &str) -> anyhow::Result<String> {
    let mut f = File::open(file_name).await?;

    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;

    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err(anyhow::anyhow!("Setting not found")),
    }
}

async fn read_and_parse_4(file_name: &str) -> Result<String, ParseError> {
    let mut f = File::open(file_name).await?;

    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;

    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("setting") {
        Some(value) => Ok(value.to_string()),
        None => Err(ParseError::Unknown),
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("error while reading file")]
    Io(#[from] std::io::Error),
    #[error("error while converting file content to utf8")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("error while parsing file content")]
    Json(#[from] serde_json::Error),
    #[error("other error")]
    Unknown,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let setting = read_and_parse("configuration.json").await;
    println!("{}", setting);

    let setting = read_and_parse_3("configuration.json").await?;
    println!("{}", setting);

    let setting = read_and_parse_4("configuration.json").await;
    match setting {
        Ok(setting) => println!("{}", setting),
        Err(e) => match e {
            ParseError::Io(e) => println!("Io error: {}", e),
            ParseError::Utf8(e) => println!("Utf8 error: {}", e),
            ParseError::Json(e) => println!("Json error: {}", e),
            ParseError::Unknown => println!("Unknown error"),
        },
    }

    Ok(())
}
