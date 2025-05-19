use trpl::Html;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    trpl::run(
        async {
            let url = args.get(1).unwrap();
            match page_title(url).await {
                None => println!("No title found"),
                Some(title) => println!("{}", title),
            }
        }
    )
}

async  fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let text = response.text().await;
    Html::parse(text.as_str())
        .select_first("title")
        .map(|t| t.inner_html())
}
