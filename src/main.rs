use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://books.toscrape.com/").send().await?;

    let html_content = res.text().await?;
    let document = Html::parse_document(&html_content);

    let product_selector = Selector::parse("article.product_pod").unwrap();
    let products = document.select(&product_selector);

    for product in products {
        let title = product
            .select(&Selector::parse("h3 a").unwrap())
            .next()
            .map(|a| a.text().collect::<String>())
            .unwrap_or_else(|| String::from("未知標題"));

        let image_url = product
            .select(&Selector::parse("img.thumbnail").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(|src| format!("https://books.toscrape.com/{}", src))
            .unwrap_or_else(|| String::from("未知圖片URL"));

        let price = product
            .select(&Selector::parse(".price_color").unwrap())
            .next()
            .map(|p| p.text().collect::<String>())
            .unwrap_or_else(|| String::from("未知價格"));

        println!("標題: {:?}", title);
        println!("圖片URL: {:?}", image_url);
        println!("價格: {:?}", price);
        println!("---");
    }

    Ok(())
}
