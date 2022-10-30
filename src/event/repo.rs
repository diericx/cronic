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
    pub fn save(self: &Repo, events: &Vec<Event>) -> Result<(), Error> {
        for event in events {
            self.conn.execute(
                "INSERT INTO event(source, code, output, date) VALUES (?1, ?2, ?3, ?4)",
                params![&event.source, &event.code, &event.output, &event.date],
            )?;
        }
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

    fn generate_events(source: &str, start: i32, end: i32) -> Vec<Event> {
        let mut events = Vec::new();
        for i in start..end {
            events.push(Event {
                id: i,
                code: 1,
                source: source.to_string(),
                output: format!("output_{}_{}", source, i),
                date: Utc::now().to_rfc2822(),
            });
        }
        events
    }

    #[test]
    fn save_events_and_query_by_source_id() {
        let event_handler = Repo::build(Connection::open_in_memory().unwrap()).unwrap();
        let events_set_1 = generate_events("set_1", 1, 4);
        let events_set_2 = generate_events("set_2", 1, 4);

        // Save expected output
        event_handler.save(&events_set_1).unwrap();
        event_handler.save(&events_set_2).unwrap();

        let events: Vec<Event> = event_handler.get_all_events_by_source("set_1").unwrap();

        let it = events_set_1.iter().zip(events.iter());
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
    fn query_events_by_source() {
        let event_handler = Repo::build(Connection::open_in_memory().unwrap()).unwrap();
        let events_set_1 = generate_events("test_source", 1, 3);
        let events_set_2 = generate_events("test_source_2", 1, 3);
        let expected_output = vec!["test_source", "test_source_2"];

        // Save events
        event_handler.save(&events_set_1).unwrap();
        event_handler.save(&events_set_2).unwrap();

        let actual: Vec<String> = event_handler.get_sources().unwrap();

        if actual != expected_output {
            panic!(
                "Event does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                expected_output, actual
            );
        }
    }

    #[test]
    fn get_event_by_id() {
        let event_handler = Repo::build(Connection::open_in_memory().unwrap()).unwrap();
        let events_set_1 = generate_events("set_1", 1, 2);

        // Save events
        event_handler.save(&events_set_1).unwrap();

        let event: Event = event_handler.get_event_by_id(&1).unwrap();
        if event != events_set_1[0] {
            panic!(
                "Event does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                event, events_set_1[0]
            );
        }
    }

    #[test]
    fn get_all_events_grouped_by_source() {
        let event_handler = Repo::build(Connection::open_in_memory().unwrap()).unwrap();
        let events_set_1 = generate_events("set_1", 1, 3);
        let events_set_2 = generate_events("set_2", 3, 5);
        let mut expected_output: HashMap<String, Vec<Event>> = HashMap::new();
        expected_output.insert(String::from("set_1"), generate_events("set_1", 1, 3));
        expected_output.insert(String::from("set_2"), generate_events("set_2", 3, 5));

        // Save events
        event_handler.save(&events_set_1).unwrap();
        event_handler.save(&events_set_2).unwrap();

        let actual = event_handler.get_all_events_grouped_by_source(99).unwrap();

        if actual != expected_output {
            panic!(
                "Actual does not match expected output.\n Expected:\n{:?} \n\n Actual:\n{:?}",
                actual, expected_output
            );
        }
    }
}
