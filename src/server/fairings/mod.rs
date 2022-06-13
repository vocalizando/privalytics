use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::form::validate::Contains;
use rocket::http::Header;
use crate::Config;

pub struct CorsFairing {
    pub config: Config,
}

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let cors_value = if let Some(hostnames) = &self.config.cors_hostnames {
            let host = req.host().unwrap().to_string();
            if hostnames.contains(&host) {
                host
            } else {
                String::new()
            }
        } else {
            String::from("*")
        };

        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            cors_value
        ));
    }
}
