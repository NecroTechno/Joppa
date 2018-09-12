#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket::request::Form;
use rocket_contrib::Template;

mod chargen;
mod contexts;

use chargen::chargen;

#[derive(Debug, FromForm)]
pub struct FormInput<> {
    #[form(field = "text")]
    text: String,
}

#[post("/validate", data = "<validate>")]
fn validate<>(validate: Result<Form<FormInput<>>, Option<String>>) -> Template {
    match validate {
        Ok(form) => {
            let result =  chargen(&form.get().text);
            match result {
                Some(x) => {
                    let context = &contexts::ValidateContextPass {
                        title: "Joppa",
                        form_result: x,
                    };
                    Template::render("validate", &context)
                },
                None => {
                    let context = &contexts::ValidateContextFail{
                        title: "Joppa",
                    };
                    Template::render("error", &context)
                },
            }
        },
        Err(Some(f)) => {
            let result: &str = &f;
            let context = &contexts::ErrorReturnContext{
                title: "Joppa",
                error_result: result,
            };
            Template::render("error_return", &context)
        },
        Err(None) => {
            let context = &contexts::ErrorContext{
                title: "Joppa",
            };
            Template::render("error", &context)
        },
    }
}

#[get("/")]
fn index() -> Template {
    let context = &contexts::IndexContext{
        title: "Joppa",
    };
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, validate]).attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
