use scraper::{ElementRef, Selector, Html};

extern crate reqwest;

pub async fn get_input(year: &str, day: &str) -> String {
    let client = reqwest::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    println!("{}", url);
    let resp = client.get(url).header("Cookie", "session=53616c7465645f5f1f9aaf0a4244fb69298b91519c47f6b151f4864168231b2d742ee7522080eb16632b601f238e5b127673983a45e3acf812e92c4fdabd391f").send().await.unwrap();
    assert!(resp.status().is_success());

    let input_string = resp.text().await.unwrap();

    return input_string;
}

pub async fn get_example(year: &str, day: &str) -> String {
    let client = reqwest::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    println!("{}", url);
    let response = client.get(url).header("Cookie", "session=53616c7465645f5f1f9aaf0a4244fb69298b91519c47f6b151f4864168231b2d742ee7522080eb16632b601f238e5b127673983a45e3acf812e92c4fdabd391f").send().await.unwrap();
    assert!(response.status().is_success());

    let input_string = response.text().await.unwrap();

    // println!("{}", input_string);
    let document = Html::parse_document(input_string.as_str());

    let p_selector = Selector::parse("p").unwrap();
    let pre_code_selectore = Selector::parse("pre > code").unwrap();
    let mut example_element: Option<ElementRef> = None;

    for p_element in document.select(&p_selector) {
        if !p_element.inner_html().ends_with(":") {
            continue;
        }
        if !p_element.has_siblings() {
            continue;
        }
        // println!("{:#?}", p_element.next_siblings().collect::<Vec<_>>());
        if p_element.next_siblings().collect::<Vec<_>>().is_empty() {
            continue;
        }

        let next_sibling = p_element.next_siblings().nth(1).unwrap();

        if next_sibling.children().collect::<Vec<_>>().len() < 1 {
            continue;
        }

        // println!("{:?}", next_sibling.children().nth(0).unwrap().value());

        if !next_sibling.children().nth(0).unwrap().value().is_element() {
            continue;
        }

        let children_of_sibling = ElementRef::wrap(next_sibling.children().nth(0).unwrap()).unwrap();
        // println!("{}", children_of_sibling.inner_html());

        if pre_code_selectore.matches(&children_of_sibling) {
            // println!("{}", children_of_sibling.inner_html());
            example_element = Some(children_of_sibling);
            break;
        }
    }

    // println!("{}", &example_element.expect("Couldnt find any reliable example code.").inner_html());

    return format!("{}", &example_element.expect("Couldnt find any reliable example code.").inner_html());
}