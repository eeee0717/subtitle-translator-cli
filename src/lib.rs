#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
pub mod formatter;
pub mod handler;
pub mod mock;
pub mod openai;
pub mod parse;
pub mod subtitle_combiner;
pub mod subtitle_extractor;
pub mod text_splitter;
pub mod translator;
const GROUP_SIZE: usize = 5;

lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let tera = match tera::Tera::new("src/templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
#[cfg(test)]
mod test {
    #[test]
    fn test_tera() {
        let mut context = tera::Context::new();
        context.insert("source_lang", "en");
        context.insert("target_lang", "ja");
        match crate::TEMPLATES.render("prompt.txt", &context) {
            Ok(s) => println!("{:?}", s),
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = std::error::Error::source(&e);
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
    }
}
