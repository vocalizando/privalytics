use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use crate::{get_args, get_cors_hostname};

pub struct CorsFairing;

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        let args = get_args();
        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            get_cors_hostname(&args.cors_hostname, &args.cors_protocol),
        ));
    }
}
