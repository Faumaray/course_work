use yew_router::Routable;
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    App,
    #[not_found]
    #[at("/404")]
    PageNotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SecondRoute {
    #[at("/")]
    Index,
    #[at("/admin/add")]
    AddNew,
    #[at("/admin/delete")]
    Delete,
    #[at("/admin/edit/:typ/:part/:name_t")]
    EditContent {
        typ: crate::support::EditTypes,
        name_t: String,
        part: crate::support::EditTypes,
    },
    #[at("/:game")]
    IndexGame { game: String },
    #[at("/:game/:related_name")]
    IndexRelated { game: String, related_name: String },
}
// type aliases to make life just a bit easier
