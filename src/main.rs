use cloudflare_r2_rs::r2::R2Manager;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;
use std::error::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let mut binary = Vec::new();
    match convert_image_to_binary("./data/sample.jpg").await {
        Ok(buffer) => {
            binary = buffer;
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    println!("binary data: {:?}", binary);
    // Set environment variables
    // Declaration and initialization of static variable
    static BUCKET_NAME: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_URI_ENDPOINT: OnceCell<String> = OnceCell::new();
    static API_TOKENS_ACCESS_KEY_ID: OnceCell<String> = OnceCell::new();
    static API_TOKENS_SECRET_ACCESS_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = BUCKET_NAME.set(env::var("BUCKET_NAME").expect("KEY not found in .env file."));
    let _ = CLOUDFLARE_URI_ENDPOINT
        .set(env::var("CLOUDFLARE_URI_ENDPOINT").expect("KEY not found in .env file."));
    let _ = API_TOKENS_ACCESS_KEY_ID
        .set(env::var("API_TOKENS_ACCESS_KEY_ID").expect("KEY not found in .env file."));
    let _ = API_TOKENS_SECRET_ACCESS_KEY
        .set(env::var("API_TOKENS_SECRET_ACCESS_KEY").expect("KEY not found in .env file."));
    //インスタンスの作成
    let r2_manager = R2Manager::new(
        //Bucket Name
        BUCKET_NAME.get().unwrap(),
        //Cloudflare URI endpoint
        CLOUDFLARE_URI_ENDPOINT.get().unwrap(),
        //API Token's Access Key ID
        API_TOKENS_ACCESS_KEY_ID.get().unwrap(),
        //API Token's Secret Access Key
        API_TOKENS_SECRET_ACCESS_KEY.get().unwrap(),
    )
    .await;
    // upload
    r2_manager
        .upload(
            "hello21.jpg",
            &binary[..],
            Some("max-age=60"),
            Some("image/jpeg"),
        )
        .await;

    // get
    // let bytes = r2_manager.get("sample.jpeg").await.unwrap();

    // delete
    // r2_manager.delete("hello10.jpg").await;
}

async fn convert_image_to_binary(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    //read file
    let mut file = File::open(file_path).await?;
    //read binary
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}
