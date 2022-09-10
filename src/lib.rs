use rusqlite::{Connection, Error, Result};

#[derive(Eq, PartialEq, Debug)]
pub struct Event {
    source: String,
    code: i32,
    output: String,
    date: String,
}

pub struct EventHandler {
    conn: Connection,
}

impl EventHandler {
    pub fn build(conn: Connection) -> Result<EventHandler, Error> {
        conn.execute(
            "CREATE TABLE event (
            id    INTEGER PRIMARY KEY,
            source TEXT NOT NULL,
            code INTEGER,
            date DATETIME,
            OUTPUT TEXT 
        )",
            (), // empty list of parameters.
        )?;

        Ok(EventHandler { conn })
    }

    pub fn save(self: &EventHandler, event: &Event) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO event(source, code, output, date) VALUES (?1, ?2, ?3, ?4)",
            (&event.source, &event.code, &event.output, &event.date),
        )?;
        Ok(())
    }

    pub fn get_all_events_by_source(
        self: &EventHandler,
        source: &str,
    ) -> Result<Vec<Event>, Error> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, source, code, output, date FROM event WHERE source = '{}'",
            source
        ))?;
        let mut rows = stmt.query([])?;

        let mut events: Vec<Event> = Vec::new();
        while let Some(row) = rows.next()? {
            events.push(Event {
                source: row.get(1)?,
                code: row.get(2)?,
                output: row.get(3)?,
                date: row.get(4)?,
            })
        }

        return Ok(events);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rand::{distributions::Alphanumeric, Rng};

    fn generate_events(source: &str, code: i32, n: i32) -> Vec<Event> {
        let mut events = Vec::new();
        for _ in 1..n {
            events.push(Event {
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
        let event_handler = EventHandler::build(conn).unwrap();
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
        let event_handler = EventHandler::build(conn).unwrap();
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
}
