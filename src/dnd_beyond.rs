use non_empty_string::NonEmptyString;

const SPELL_URL: &str = "https://www.dndbeyond.com/spells?page=";

fn page_url(page: usize) -> String {
    format!("{}{}", SPELL_URL, page)
}

async fn page_html(page: usize) -> Option<NonEmptyString> {
    let url = page_url(page);
    let browser = headless_chrome::Browser::default().ok()?;
    let tab = browser.new_tab().ok()?;
    tab.navigate_to(&url).ok()?;
    tab.wait_until_navigated().ok()?;
    NonEmptyString::new(tab.get_content().ok()?).ok()
}

pub fn main() {
    let runner = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runner.block_on(page_html(3));
    println!("{:?}", result);
}
