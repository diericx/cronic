# Cronic

Cronic is a simple tool for saving and observing events on a remote server.

For example, if you have a cron job that runs periodically you can use a simple curl at the end of the job to capture the output.
It will be written to disk for later viewing. 
This can be helpful when running cron jobs whose output is critical (e.g. backups).

## Design

A REST server with a SQLite db that will record `events` which contain a `source` (string), `output` (string) and an error `code` (int).

An event's `source` is a unique identifier, `restic_backup_bookstack` for example.

A dashboard is presented on `0.0.0.0:8000` that shows basic information for the most recent events by source.

## Usage

### Storing events with bash

An example of a bash script storing a new event can be found in the `examples/` directory.

### Docker Compose

```
cronic:
  container_name: cronic
  image: diericx/cronic:latest
  restart: always
  environment:
    - DB_PATH=/config/cronic_db.sqlite
  volumes:
    - cronic:/config
    - customized_templates:/templates # OPTIONAL
  ports:
    - 80
```

## Screenshots

![](./content/dashboard.png)
![](./content/event.png)

## Development

### Mac cross compilation notes

On Mac it seems we need to install a `musl` linker and ensure that the scripts know about it.

Look into `musl-cross` and ensure [these steps are followed](https://github.com/rust-lang/backtrace-rs/issues/34), specifically:

```
TARGET_CC=x86_64-linux-musl-gcc cargo build ...
```
