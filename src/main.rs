use reqwest;
use select::document::Document;
use select::predicate::Name;
use tokio;
use core::time;
use std::collections::HashSet;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url_list = &mut vec![String::from("https://www.rust-lang.org/")];
    loop{
        let client = reqwest::Client::builder().build().unwrap();
        let current_url = url_list.pop().unwrap();
        println!("Current URLs: {}", current_url);
        
        if current_url.find("http") == Some(0) && current_url.find(".pdf") != Some(0) {
            let content = client.get(current_url).send().await?.text().await?;
            let document = Document::from(content.as_str());
            let href_list = document.find(Name("a")).filter_map(|n| n.attr("href")).map(str::to_string).collect::<HashSet<String>>();
            println!("URLs: {:#?}", href_list);
            for href in href_list{
                if href.find("http") == Some(0) && href.find(".pdf") != Some(0){
                    url_list.push(href);
                }
            }
        }

        if url_list.len() == 0{
            break
        }
        
        sleep(time::Duration::from_millis(10000));
    }
    Ok(())
    
}