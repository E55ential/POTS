use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("ошибка запроса: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("неправильный http ответ: {0}")]
    BadResponse(String),
    #[error("некорректный URL: {0}")]
    UrlParseError(#[from] url::ParseError),
}

/// Результат проверки одной страницы
#[derive(Debug)]
struct PageResult {
    url: Url,
    status: Result<u16, Error>,
    links_found: usize,
}

/// Посещает страницу, извлекает все ссылки в том же домене
fn visit_page(
    client: &Client,
    url: &Url,
    base_domain: &str,
) -> Result<(PageResult, Vec<Url>), Error> {
    println!("Проверяем {}", url);
    let response = client.get(url.clone()).send()?;

    let status_code = response.status().as_u16();
    if !response.status().is_success() {
        let page_result = PageResult {
            url: url.clone(),
            status: Err(Error::BadResponse(status_code.to_string())),
            links_found: 0,
        };
        return Ok((page_result, Vec::new()));
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap(); // селектор для всех ссылок
    let mut link_urls = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base_url.join(href) {
                // Оставляем только ссылки внутри того же домена
                if absolute_url.host_str() == Some(base_domain) {
                    link_urls.push(absolute_url);
                }
            }
        }
    }

    let page_result = PageResult {
        url: url.clone(),
        status: Ok(status_code),
        links_found: link_urls.len(),
    };

    Ok((page_result, link_urls))
}

fn main() {
    let client = Client::new();
    // Задайте начальный URL (можно изменить на любой другой)
    let start_url = Url::parse("httpbin.org/links/3/1").unwrap();
    let base_domain = start_url.host_str().expect("Не удалось определить домен");

    let mut visited = HashSet::new();
    let mut to_visit = vec![start_url.clone()];
    let mut results = Vec::new();

    while let Some(url) = to_visit.pop() {
        if visited.contains(&url) {
            continue;
        }
        visited.insert(url.clone());

        match visit_page(&client, &url, base_domain) {
            Ok((page_result, links)) => {
                results.push(page_result);
                for link in links {
                    if !visited.contains(&link) {
                        to_visit.push(link);
                    }
                }
            }
            Err(e) => {
                results.push(PageResult {
                    url,
                    status: Err(e),
                    links_found: 0,
                });
            }
        }
    }

    // Вывод результатов
    println!("\n=== Результаты проверки ===");
    for result in results {
        match result.status {
            Ok(code) => println!("[{}] {} (ссылок: {})", code, result.url, result.links_found),
            Err(e) => println!("[ОШИБКА] {}: {}", result.url, e),
        }
    }
    println!("\nВсего проверено страниц: {}", visited.len());
}
