use tonic::{Request, Status};
use utils::jwt::Payload;

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) => {
            let token = t.to_str().unwrap().split(" ").collect::<Vec<&str>>();
            let payload = Payload::verify_token(token[1]);
            match payload {
                Ok(claims) => {
                    if claims.is_valid() {
                        req.extensions_mut().insert(claims);
                        Ok(req)
                    } else {
                        Err(Status::permission_denied("Invalid token"))
                    }
                }
                Err(err) => Err(Status::permission_denied(err.to_string())),
            }
        }
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
