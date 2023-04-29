use serde::{Serialize, Deserialize};
use rocket::{response::{self, Response, Responder}, Request, http::{Status, ContentType}, serde::json::Json};

pub struct JsonResponder<T> {
    pub response: T,
    pub status: Status
}

#[derive(Serialize, Deserialize)]
pub struct EmptyResponse {}

impl<T: Serialize> JsonResponder<T> {
    pub fn created(data: T) -> Self {
        JsonResponder::from(data, Status::Created)
    }

    pub fn ok(data: T) -> Self {
        JsonResponder::from(data, Status::Ok)
    }

    pub fn from(data: T, status: Status) -> Self {
        Self { response: data, status }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for JsonResponder<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let data = Json(self.response);
        Response::build_from(data.respond_to(req)?)
            .header(ContentType::JSON)
            .status(self.status)
            .ok()
    }
}
