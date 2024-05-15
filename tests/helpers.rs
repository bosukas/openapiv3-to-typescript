use openapiv3::OpenAPI;

pub fn fetch_oas_yaml(url: &str) -> OpenAPI {
    let text = reqwest::blocking::get(url).unwrap().text().unwrap();
    serde_yaml::from_str(&text).unwrap()
}
