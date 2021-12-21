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
// type aliases to make life just a bit easier
