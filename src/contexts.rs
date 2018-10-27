use chargen::Character;

#[derive(Serialize)]
pub struct IndexContext {
    pub title: &'static str,
}

#[derive(Serialize)]
pub struct ValidateContextPass {
    pub title: &'static str,
    pub form_result: Character,
}

#[derive(Serialize)]
pub struct ValidateContextFail {
    pub title: &'static str,
    pub error_result: &'static str,
}

#[derive(Serialize)]
pub struct ErrorReturnContext<'a> {
    pub title: &'static str,
    pub error_result: &'a str,
}

#[derive(Serialize)]
pub struct ErrorContext {
    pub title: &'static str,
}
