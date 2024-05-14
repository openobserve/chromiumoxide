use std::path::Path;

use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::fetcher::{BrowserFetcher, BrowserFetcherOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::canonicalize("/root/chromiumoxide/download").await.expect("download test");
    // Fetcher browser
    let download_path = Path::new("/root/chromiumoxide/download");
    tokio::fs::create_dir_all(&download_path).await.expect("failed to create download path");
    let fetcher = BrowserFetcher::new(
        BrowserFetcherOptions::builder()
            .with_path(&download_path)
            .build().expect("Failed to build fetcher"),
    );
    let info = fetcher.fetch().await.expect("faild to fetch the binary");

    println!("executable path {:?}", info.executable_path);
    // Verify browser
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .chrome_executable(info.executable_path)
            .arg("--no-sandbox")
            .build().expect("Get the executable"),
    )
    .await.expect("failed to get the browser/handler");

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(h) => match h {
                    Ok(_) => continue,
                    Err(_) => break,
                },
                None => break,
            }
        }
    });

    let page = browser.new_page("about:blank").await.expect("new page failed");

    let sum: usize = page.evaluate("1 + 2").await?.into_value().expect("evaluate ");
    assert_eq!(sum, 3);
    println!("it worked!");

    browser.close().await?;
    handle.await?;
    Ok(())
}
