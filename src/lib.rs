pub struct Event {
    pub source_id: String,
    pub output: String,
}

pub struct EventHandler {
    pub data_path: String,
}

impl EventHandler {
    pub fn save(self: &EventHandler, event: &Event) -> Result<(), &'static str> {
        return Ok(());
    }

    pub fn get_all_events_by_source_id(
        self: &EventHandler,
        source_id: String,
    ) -> Result<Vec<Event>, &'static str> {
        return Ok(vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_recall_single_event() {
        let event_handler = EventHandler {
            data_path: String::from(":memory:"),
        };
        let expected_output = vec![Event {
            source_id: String::from("test_source"),
            output: String::from("test_output"),
        }];

        event_handler
            .save(&Event {
                source_id: String::from("test_source"),
                output: String::from("test_output"),
            })
            .unwrap();

        let events: Vec<Event> = event_handler
            .get_all_events_by_source_id(String::from("test_source"))
            .unwrap();

        let it = expected_output.iter().zip(events.iter());
        for (i, (event_a, event_b)) in it.enumerate() {
            if event_a.source_id != event_b.source_id || event_a.output != event_b.output {
                panic!("Unexpected event");
            }
        }
    }
}
