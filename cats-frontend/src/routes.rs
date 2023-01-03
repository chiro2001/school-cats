use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/user/:id")]
    UserInfo { id: u32 },
    #[at("/cat/:id")]
    CatInfo { id: u32 },
}