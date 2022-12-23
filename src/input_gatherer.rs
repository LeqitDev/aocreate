use scraper::{ElementRef, Selector, Html};

extern crate reqwest;

// TODO: get rid of unnecessary comments (mostly println! code :D)

pub async fn get_input(year: &str, day: &str) -> String {
    // Create client
    let client = reqwest::Client::new();
    // Go to the specific day of the project year
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    // println!("{}", url);
    // Set a Session cookie to authenticate the request
    // TODO: Change/require cookie (Cookie valid for 2 years)
    let resp = client.get(url).header("Cookie", "session=53616c7465645f5f1f9aaf0a4244fb69298b91519c47f6b151f4864168231b2d742ee7522080eb16632b601f238e5b127673983a45e3acf812e92c4fdabd391f").send().await.unwrap();
    assert!(resp.status().is_success());

    // read and return the content
    let input_string = resp.text().await.unwrap();

    return input_string;
}

pub async fn get_example(year: &str, day: &str) -> String {
    // Create client
    let client = reqwest::Client::new();
    // Go to the specific day of the project year
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    // println!("{}", url);
    // Set a Session cookie to authenticate the request
    // TODO: Change/require cookie (Cookie valid for 2 years)
    let response = client.get(url).header("Cookie", "session=53616c7465645f5f1f9aaf0a4244fb69298b91519c47f6b151f4864168231b2d742ee7522080eb16632b601f238e5b127673983a45e3acf812e92c4fdabd391f").send().await.unwrap();
    assert!(response.status().is_success());

    // get the site content
    let example_string = response.text().await.unwrap();

    // println!("{}", input_string);
    // parse the site content into html
    let document = Html::parse_document(example_string.as_str());

    // define the 'p' selector
    let p_selector = Selector::parse("p").unwrap();
    // define the 'pre > code' selector
    let pre_code_selectore = Selector::parse("pre > code").unwrap();
    // example-data element
    let mut example_element: Option<ElementRef> = None;

    for p_element in document.select(&p_selector) {
        // 'p' element has to end with a colon
        if !p_element.inner_html().ends_with(":") {
            continue;
        }
        // the 'p' element has to have elements following it
        if !p_element.has_siblings() {
            continue;
        }
        // println!("{:#?}", p_element.next_siblings().collect::<Vec<_>>());
        // TODO: duplicated code??
        if p_element.next_siblings().collect::<Vec<_>>().is_empty() {
            continue;
        }

        // define the next sibling of the 'p' element
        let next_sibling = p_element.next_siblings().nth(1).unwrap();

        // next sibling must have a children
        if next_sibling.children().collect::<Vec<_>>().len() < 1 {
            continue;
        }

        // println!("{:?}", next_sibling.children().nth(0).unwrap().value());

        // the children must be an element
        if !next_sibling.children().nth(0).unwrap().value().is_element() {
            continue;
        }

        let children_of_sibling = ElementRef::wrap(next_sibling.children().nth(0).unwrap()).unwrap();
        // println!("{}", children_of_sibling.inner_html());

        // does this specific element match the 'pre > code' selector than its the example code
        if pre_code_selectore.matches(&children_of_sibling) {
            // println!("{}", children_of_sibling.inner_html());
            example_element = Some(children_of_sibling);
            break;
        }
    }

    // println!("{}", &example_element.expect("Couldnt find any reliable example code.").inner_html());

    // return the example code if its found a fitting element
    return format!("{}", &example_element.expect("Couldnt find any reliable example code.").inner_html());
}