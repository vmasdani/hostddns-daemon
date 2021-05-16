use std::time::Duration;

use reqwest::StatusCode;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    loop {
        let update_url: &str = dotenv!("UPDATE_URL");
        println!("Dotenv contents: {}", update_url);

        match reqwest::get(update_url).await {
            Ok(resp) => {
                let code = resp.status();
                println!("[reqwest] Response code: {:?}", code);

                if code == StatusCode::OK {
                    println!("[reqwest] status OK");
                } else {
                    println!("[reqwest] status not OK");
                }
            }
            Err(e) => {
                println!("[reqwest] ERROR SENDING {:?}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
