// modules this file will use
use std::sync::{Arc, Mutex};

use rocket::get;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request, State};

use oso::{Oso, PolarClass};

use crate::jobs::{Job, DB};

#[derive(Debug)]
struct User(String);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(user) = request.headers().get_one("user") {
            request::Outcome::Success(User(user.to_string()))
        } else {
            request::Outcome::Failure((Status::Forbidden, "Unknown User".to_owned()))
        }
    }
}

// Catch error whenever user tries to get a restricted route or document
#[catch(403)]
fn not_authorized(_: &Request) -> String {
    "Not Authorized!\n".to_string()
}

// Catch error whenever user inputs a non-valid URL
#[catch(404)]
fn not_found(_: &Request) -> String {
    "Not Found!\n".to_string()
}

// Get method to retrieve firm's jobs
#[get("/jobs/<id>")]
fn get_job(oso: State<OsoState>, user: User, id: usize) -> Result<Option<String>, Status> {
    if let Some(job) = DB.get(&id) {
// If user is allowed, get file/document
        if oso.is_allowed(user.0, "GET", job.clone()) {
            Ok(Some(format!("{}\n", job)))
// If they are not allowed throw 403 error
        } else {
            Err(Status::Forbidden)
        }
    } else {
        Ok(None)
    }
}

// Declare structure for oso state
struct OsoState {
    oso: Arc<Mutex<Oso>>,
}

// Define implementation type for oso state
impl OsoState {
    pub fn is_allowed(&self, actor: String, action: &str, resource: Job) -> bool {
        let mut guard = self.oso.lock().unwrap();
        guard
            .is_allowed(actor, action.to_string(), resource)
            .unwrap()
    }
}

//public function for oso
pub fn oso() -> Oso {
    let mut oso = Oso::new();

    oso.register_class(Job::get_polar_class()).unwrap();
// get jobs.polar file
    oso.load_file("jobs.polar").unwrap();

    oso
}

// public function for rocket
pub fn rocket(oso: Oso) -> rocket::Rocket {
    let oso_state = OsoState {
        oso: Arc::new(Mutex::new(oso)),
    };

    rocket::ignite()
        .mount("/", routes![get_job])
        .manage(oso_state)
        .register(catchers![not_authorized, not_found])
}

// allow rocket launch oso
pub fn run() {
    rocket(oso()).launch();
}

mod test {
    use super::{oso, rocket};
    use rocket::http::{Header, Status};
    use rocket::local::Client;

    #[test]
    fn get_job_no_rules() {
        let client = Client::new(rocket(oso())).expect("valid rocket instance");
        let response = client.get("/jobs/1").dispatch();
        assert_eq!(response.status(), Status::Forbidden);
    }

    #[test]
    fn get_job_first_rule() {
        let mut oso = oso();
        oso.load_str(
            "allow(actor: String, \"GET\", _job: Job) if actor.ends_with(\"@example.com\");",
        )
        .unwrap();
        let client = Client::new(rocket(oso)).expect("valid rocket instance");
        let mut request = client.get("/jobs/1");
        request.add_header(Header::new("user", "alice@example.com"));
        let ok_response = request.dispatch();
        assert_eq!(ok_response.status(), Status::Ok);
        let unauthorized_response = client.get("/jobs/1").dispatch();
        assert_eq!(unauthorized_response.status(), Status::Forbidden);
    }

    #[test]
    fn get_job_second_rule() {
        let mut oso = oso();
        oso.load_str(
            "allow(actor: String, \"GET\", job: Job) if job.submitted_by = actor;",
        )
        .unwrap();
        let client = Client::new(rocket(oso)).expect("valid rocket instance");
        let mut request = client.get("/jobs/1");
        request.add_header(Header::new("user", "alice@example.com"));
        let ok_response = request.dispatch();
        assert_eq!(ok_response.status(), Status::Ok);

        let mut bad_request = client.get("/jobs/3");
        bad_request.add_header(Header::new("user", "alice@example.com"));
        let unauthorized_response = bad_request.dispatch();
        assert_eq!(unauthorized_response.status(), Status::Forbidden);
    }
}
