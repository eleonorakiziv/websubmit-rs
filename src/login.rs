use crate::config::Config;
use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub(crate) fn login(config: State<Config>) -> Template {
    let mut ctx = HashMap::new();
    ctx.insert("CLASS_ID", config.class.clone());
    ctx.insert("parent", String::from("layout"));
    Template::render("login", &ctx)
}
