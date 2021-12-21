use yew_router::Routable;
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Viewer,
    #[at("/add")]
    AddNew,
    #[at("/delete")]
    Delete,
    #[at("/edit")]
    EditContent,
    #[not_found]
    #[at("/404")]
    PageNotFound,
}
// type aliases to make life just a bit easier
