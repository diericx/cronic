use chrono::Utc;
use cronic::Event;
use cronic::EventHandler;
use rocket::form::Form;
use rocket::State;
use rusqlite::Connection;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct UserInput {
    source: String,
    output: String,
    code: i32,
}

struct EventHandlerState {
    event_handler_mutex: Mutex<cronic::EventHandler>,
}

#[post("/new", data = "<user_input>")]
fn new_event(
    event_handler_state: &State<EventHandlerState>,
    user_input: Form<UserInput>,
) -> String {
    let event_handler = event_handler_state.event_handler_mutex.lock().unwrap();

    event_handler
        .save(&Event {
            source: user_input.source.clone(),
            output: user_input.output.clone(),
            code: user_input.code,
            date: Utc::now().to_rfc2822(),
        })
        .unwrap();

    // TODO: return new id
    format!("{}", 0)
}

#[get("/")]
fn index(event_handler_state: &State<EventHandlerState>) -> String {
    let event_handler = event_handler_state.event_handler_mutex.lock().unwrap();
    let sources = event_handler.get_sources().unwrap();

    let mut content = String::from("");
    for source in sources {
        content += &format!("{}\n", source);
    }
    content
}

#[launch]
fn rocket() -> _ {
    let db_path = "/tmp/cronic.db";
    let conn = Connection::open(db_path).unwrap();
    let event_handler = EventHandler::build(conn).unwrap();

    rocket::build()
        .manage(EventHandlerState {
            event_handler_mutex: Mutex::new(event_handler),
        })
        .mount("/", routes![index, new_event])
}
