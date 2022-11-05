use chrono::Utc;
use cronic::event::Event;
use cronic::event::Repo;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use rusqlite::Connection;
use std::env;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

struct EventRepoState {
    event_repo_mutex: Mutex<Repo>,
}

#[get("/")]
fn index(event_repo_state: &State<EventRepoState>) -> Template {
    let event_repo = event_repo_state.event_repo_mutex.lock().unwrap();
    let events_by_source = event_repo.get_all_events_grouped_by_source(10).unwrap();

    Template::render(
        "events/index",
        context! {
            events_by_source,
        },
    )
}

#[get("/<id>")]
fn event(event_repo_state: &State<EventRepoState>, id: u32) -> Template {
    let event_repo = event_repo_state.event_repo_mutex.lock().unwrap();
    let event = event_repo.get_event_by_id(&id).unwrap();

    Template::render(
        "events/show",
        context! {
            event,
        },
    )
}

#[post("/new", format = "json", data = "<event>")]
fn new_event(event_repo_state: &State<EventRepoState>, event: Json<Event>) -> String {
    let event_repo = event_repo_state.event_repo_mutex.lock().unwrap();

    event_repo
        .save(&vec![Event {
            id: 0,
            source: event.source.clone(),
            output: event.output.clone(),
            code: event.code,
            date: Utc::now().to_rfc2822(),
        }])
        .unwrap();

    // TODO: return new id
    format!("{}", 0)
}

#[get("/<source>")]
fn source(event_repo_state: &State<EventRepoState>, source: String) -> Template {
    let event_repo = event_repo_state.event_repo_mutex.lock().unwrap();
    let events = event_repo.get_all_events_by_source(&source).unwrap();

    Template::render(
        "sources/show",
        context! {
            events,
            source
        },
    )
}

#[launch]
fn rocket() -> _ {
    let default_db_path = "/tmp/cronic.db";

    let db_path = match env::var_os("DB_PATH") {
        Some(v) => v.into_string().unwrap(),
        None => default_db_path.clone().to_string(),
    };

    let conn = Connection::open(db_path).unwrap();
    let event_repo = Repo::build(conn).unwrap();

    rocket::build()
        .manage(EventRepoState {
            event_repo_mutex: Mutex::new(event_repo),
        })
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/events", routes![event, new_event])
        .mount("/sources", routes![source])
}
