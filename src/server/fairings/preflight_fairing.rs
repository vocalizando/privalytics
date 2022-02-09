use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Header, Method, Status};
use rocket::response::Body;
use crate::{get_args, get_cors_hostname};

pub struct PreflightFairing;

#[rocket::async_trait]
impl Fairing for PreflightFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let args = get_args();
        if req.method() == Method::Options {
            let empty_body = Body::default();

            res.set_status(Status::NoContent);
            res.set_header(Header::new(
                "Access-Control-Allow-Origin",
                get_cors_hostname(&args.cors_hostname, &args.cors_protocol),
            ));
            res.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "PUT, POST, GET, OPTIONS, DELETE",
            ));
            res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            res.set_header(Header::new("Access-Control-Max-Age", "86400"));
            res.set_streamed_body(empty_body);
        }
    }
}
