use reqwest::Client;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() {
    let url = "https://www.hakyoff.com/";  // link da pagina
    println!("Crawling: {}", url);
    
    match fetch_html(url).await {
        Ok(html) => {
            let links = extract_links(&html);
            println!("\nEncontrados {} links:", links.len());
            
            for link in links {
                match check_link(&link).await {
                    Ok(status) => println!("{} -> {}", link, status),
                    Err(e) => println!("Erro ao verificar {}: {}", link, e),
                }
            }
        }
        Err(e) => println!("Erro ao baixar a página: {}", e),
    }
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            if link.starts_with("http") {
                links.push(link.to_string());
            }
        }
    }
    links
}

async fn check_link(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    Ok(response.status().to_string())
}
