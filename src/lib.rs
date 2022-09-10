use rusqlite::{Connection, Error, Result};

pub struct Event {
    id: i32,
    source: String,
    code: i32,
    output: String,
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
            OUTPUT TEXT 
        )",
            (), // empty list of parameters.
        )?;

        Ok(EventHandler { conn })
    }
    pub fn save(self: &EventHandler, event: &Event) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO event(source, code, output) VALUES (?1, ?2, ?3)",
            (&event.source, &event.code, &event.output),
        )?;
        Ok(())
    }

    pub fn get_all_events_by_source(
        self: &EventHandler,
        source: String,
    ) -> Result<Vec<Event>, Error> {
        let mut events: Vec<Event> = Vec::new();
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, source, code, output FROM event WHERE source = '{}'",
            source
        ))?;
        let event_iter = stmt.query_map([], |row| {
            Ok(Event {
                id: row.get(0)?,
                source: row.get(1)?,
                code: row.get(2)?,
                output: row.get(3)?,
            })
        })?;
        for event in event_iter {
            events.push(event.unwrap())
        }
        return Ok(events);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_recall_single_event() {
        let conn = Connection::open_in_memory().unwrap_or_else(|e| {
            panic!("Error building event handler: {e}");
        });
        let event_handler = EventHandler::build(conn).unwrap();
        let expected_output = vec![Event {
            id: 0,
            source: String::from("test_source"),
            code: 1,
            output: String::from("test_output"),
        }];

        event_handler
            .save(&Event {
                id: 0,
                source: String::from("test_source"),
                output: String::from("test_output"),
                code: 1,
            })
            .unwrap();

        let events: Vec<Event> = event_handler
            .get_all_events_by_source(String::from("test_source"))
            .unwrap();

        let it = expected_output.iter().zip(events.iter());
        for (_i, (event_a, event_b)) in it.enumerate() {
            if event_a.source != event_b.source || event_a.output != event_b.output {
                panic!("Unexpected event");
            }
        }
    }
}
