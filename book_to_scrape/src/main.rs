use reqwest::blocking;
use select::{
    document::Document,
    predicate::{Class, Name},
};
use std::collections::HashMap;

pub fn parse_product(html: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    let document = Document::from(html);

    let mut articles = document.find(Name("article")).into_iter();
    while let Some(article) = articles.next() {
        let mut res = HashMap::new();
        res.insert(
            "title".to_string(),
            article
                .find(Name("img"))
                .next()
                .unwrap()
                .attr("alt")
                .unwrap()
                .to_string()
                .clone(),
        );
        res.insert(
            "price".to_string(),
            article
                .find(Class("price_color"))
                .next()
                .unwrap()
                .text()
                .clone(),
        );
        res.insert(
            "ratings".to_string(),
            article
                .find(Class("star-rating"))
                .next()
                .unwrap()
                .attr("class")
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .to_string()
                .clone(),
        );

        results.push(res);
    }

    return Ok(results);
}


pub fn scrape(halaman: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("> mengunduh halaman {}", halaman);
    let res = blocking::get(format!("https://books.toscrape.com/catalogue/page-{}.html", halaman))?;
    let body = res.text()?;
    let products = parse_product(&body).expect("opps, ada error");

    println!("> {} produk berhasil diambil", products.len());
    println!("> daftar produk:");

    for (index, product) in products.iter().enumerate() {
        println!("  - produk ke-{}", index + 1);
        println!("    - judul: {}", product.get("title").unwrap());
        println!("    - rating: {}", product.get("ratings").unwrap());
        println!("    - harga: {}", product.get("price").unwrap());

    }
    Ok(())
}

fn main() {
    for halaman in 1..51 {
        scrape(halaman).expect("opps, ada error");
    }
}
