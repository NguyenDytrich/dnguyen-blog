pub mod http;
pub mod model;

pub mod error {

    use std::error::Error;
    use std::fmt;

    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};
    use rocket::http::Status;

    #[derive(Debug)]
    /// Wrapper class for tokio_postgres errors
    pub struct DBError;

    impl fmt::Display for DBError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DBError.")
        }
    }
    impl Error for DBError {}
    impl<'r> Responder<'r, 'r> for DBError {
        fn respond_to(self, _: &Request) -> response::Result<'r> {
            return Response::build()
                .status(Status::BadRequest)
                .ok();
        }
    }
    impl From<tokio_postgres::Error> for DBError {
        fn from(_: tokio_postgres::Error) -> Self {
            return DBError {};
        }
    }
}

pub mod db {

    use std::error::Error;
    use tokio_postgres::NoTls;

    /// Open a connection to Postgres on a new task
    pub async fn spawn_connection(url: &str) -> Result<tokio_postgres::Client, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        return Ok(client);
    }
}

pub mod htmlify {
    use ammonia::clean;
    use pulldown_cmark::{Parser, Options, html::push_html};

    /// Takes a string formatted in Markdown and returns it as sanitized HTML
    pub fn transcribe(markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(markdown, options);

        // String buffer output
        let mut out = String::new();
        push_html(&mut out, parser);
        clean(&*out)
    }

    pub fn monthify(num: usize) -> Option<String> {
        let a = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        if num > a.len() {
            None
        } else {
            Some(a[num - 1].to_string())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_transcribes_md_as_html() {
            let input = "Hello world, [this is](http://www.google.com/) ~~a~~ *an* example.";
            let result = transcribe(input);
            let expected = "<p>Hello world, <a href=\"http://www.google.com/\" rel=\"noopener noreferrer\">this is</a> <del>a</del> <em>an</em> example.</p>\n";

            assert_eq!(expected, &result);
        }

        #[test]
        fn it_gets_a_month() {
            let a = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            for n in 1..=12 {
                assert_eq!(Some(a[n-1].to_string()), monthify(n));
            }
        }

        #[test]
        fn it_doesnt_get_month_out_of_bounds() {
            assert_eq!(None, monthify(13));
        }
    }
}
