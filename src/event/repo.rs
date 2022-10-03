use crate::event::Event;
use rusqlite::{params, Connection, Error, Result};
use std::collections::HashMap;

pub struct Repo {
    conn: Connection,
}

impl Repo {
    // Creates the table for events and returns a new Repo with the provided connection
    pub fn build(conn: Connection) -> Result<Repo, Error> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS event (
            id    INTEGER PRIMARY KEY,
            source TEXT NOT NULL,
            code INTEGER,
            date DATETIME,
            OUTPUT TEXT 
        )",
            params![], // empty list of parameters.
        )?;

        Ok(Repo { conn })
    }

    // Saves an Event to the database
    pub fn save(self: &Repo, event: &Event) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO event(source, code, output, date) VALUES (?1, ?2, ?3, ?4)",
            params![&event.source, &event.code, &event.output, &event.date],
        )?;
        Ok(())
    }

    // Queries the databse for a single Event with a given id
    pub fn get_event_by_id(self: &Repo, id: &u32) -> Result<Event, Error> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, source, code, output, date FROM event WHERE id = '{}'",
            id
        ))?;
        let mut rows = stmt.query(params![])?;

        match rows.next()? {
            Some(row) => Ok(Event {
                id: row.get(0)?,
                source: row.get(1)?,
                code: row.get(2)?,
                output: row.get(3)?,
                date: row.get(4)?,
            }),
            // TODO: Custom error?
            None => Err(Error::InvalidQuery),
        }
    }

    // Queries the databse for all Events with a given source
    pub fn get_all_events_by_source(self: &Repo, source: &str) -> Result<Vec<Event>, Error> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, source, code, output, date FROM event WHERE source = '{}'",
            source
        ))?;
        let mut rows = stmt.query(params![])?;

        let mut events: Vec<Event> = Vec::new();
        while let Some(row) = rows.next()? {
            events.push(Event {
                id: row.get(0)?,
                source: row.get(1)?,
                code: row.get(2)?,
                output: row.get(3)?,
                date: row.get(4)?,
            })
        }

        return Ok(events);
    }

    // Queries the databse for all Events and groups them by source
    pub fn get_all_events_grouped_by_source(
        self: &Repo,
        limit: i32,
    ) -> Result<HashMap<String, Vec<Event>>, Error> {
        let mut stmt = self.conn.prepare(&format!(
            "select id, source, code, output, date from (
    select id, source, code, output, date,
           row_number() over (partition by source order by date desc) as date_rank 
    from event) ranks
where date_rank <= {};",
            limit
        ))?;
        let mut rows = stmt.query(params![])?;

        let mut events: HashMap<String, Vec<Event>> = HashMap::new();
        while let Some(row) = rows.next()? {
            let event = Event {
                id: row.get(0)?,
                source: row.get(1)?,
                code: row.get(2)?,
                output: row.get(3)?,
                date: row.get(4)?,
            };

            if !events.contains_key(&event.source.clone()) {
                events.insert((&event).source.clone(), Vec::new());
            }
            match events.get_mut(&event.source.clone()) {
                Some(events_vec) => events_vec.push(event),
                None => (),
            }
        }

        return Ok(events);
    }

    pub fn get_sources(self: &Repo) -> Result<Vec<String>, Error> {
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT DISTINCT source FROM event",))?;
        let mut rows = stmt.query(params![])?;

        let mut sources: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            sources.push(row.get(0)?)
        }

        return Ok(sources);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rand::{distributions::Alphanumeric, Rng};

    fn generate_events(source: &str, code: i32, n: i32) -> Vec<Event> {
        let mut events = Vec::new();
        for i in 1..n {
            events.push(Event {
                id: i,
                code,
                source: source.to_string(),
                output: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect(),
                date: Utc::now().to_rfc2822(),
            });
        }
        events
    }

    #[test]
    fn save_and_recall_single_event_by_source_id() {
        let conn = Connection::open_in_memory().unwrap();
        let event_handler = Repo::build(conn).unwrap();
        let expected_output = generate_events("test_source", 1, 0);

        // Save expected output
        for event in &expected_output {
            event_handler.save(&event).unwrap();
        }

        let events: Vec<Event> = event_handler
            .get_all_events_by_source("test_source")
            .unwrap();

        let it = expected_output.iter().zip(events.iter());
        for (_i, (expected, actual)) in it.enumerate() {
            if actual != expected {
                panic!(
                    "Event does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                    expected, actual
                );
            }
        }
    }

    #[test]
    fn save_and_recall_multiple_events_by_source_id() {
        let conn = Connection::open_in_memory().unwrap();
        let event_handler = Repo::build(conn).unwrap();
        let expected_output = generate_events("test_source", 1, 2);
        let extra_events = generate_events("test_source_2", 1, 2);

        // Save expected output
        for event in &expected_output {
            event_handler.save(&event).unwrap();
        }

        // Save extra events
        for event in &extra_events {
            event_handler.save(&event).unwrap();
        }

        let events: Vec<Event> = event_handler
            .get_all_events_by_source("test_source")
            .unwrap();

        let it = expected_output.iter().zip(events.iter());
        for (_i, (expected, actual)) in it.enumerate() {
            if actual != expected {
                panic!(
                    "Event does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                    expected, actual
                );
            }
        }
    }

    #[test]
    fn get_sources() {
        let conn = Connection::open_in_memory().unwrap();
        let event_handler = Repo::build(conn).unwrap();
        let events_set_1 = generate_events("test_source", 1, 2);
        let events_set_2 = generate_events("test_source_2", 1, 2);
        let expected_output = vec!["test_source", "test_source_2"];

        // Save expected output
        for event in &events_set_1 {
            event_handler.save(&event).unwrap();
        }

        // Save extra events
        for event in &events_set_2 {
            event_handler.save(&event).unwrap();
        }

        let actual: Vec<String> = event_handler.get_sources().unwrap();

        if actual != expected_output {
            panic!(
                "Event does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                expected_output, actual
            );
        }
    }
}
