pub mod error {

    use std::error::Error;
    use std::fmt;

    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};
    use rocket::http::Status;

    #[derive(Debug)]
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

pub mod blog {

    use chrono::prelude::*;
    use std::fmt;

    pub struct BlogPost {
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        is_public: bool,
        delta: String,
        title: String
    }

    impl fmt::Display for BlogPost {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\"{}\" (created {})", self.title, self.created_at)
        }
    }
}
