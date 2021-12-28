use yew_router::Routable;
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Viewer,
    #[at("/add")]
    AddNew,
    #[at("/delete")]
    Delete,
    #[at("/edit/:typ/:part/:name_t")]
    EditContent {
        typ: crate::pages::viewer::Edit,
        name_t: String,
        part: crate::pages::viewer::Edit,
    },
    #[not_found]
    #[at("/404")]
    PageNotFound,
}

pub async fn fetch_post(body: String, path: &str) -> Result<middleware::Response, reqwasm::Error> {
    let res: Result<middleware::Response, reqwasm::Error> = reqwasm::http::Request::post(path)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
pub async fn fetch_get(body: String, path: &str) -> Result<middleware::Response, reqwasm::Error> {
    let res: Result<middleware::Response, reqwasm::Error> = reqwasm::http::Request::get(path)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
// type aliases to make life just a bit easier
