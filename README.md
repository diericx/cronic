# Cronic

Cronic is a simple tool for saving and observing events and their attributes on a server.

This was inspired when I shifted my backup scheduling from Airflow to Cron and wanted better and more accessible insight into the job results.

I also wanted to learn Rust, so while I'm sure other projects like this exist that was another motivator.

## Design

It is a REST server with a SQLite db that will record `events` which contain a `source` (string), `output` (string) and an error `code` (int).

An event's `source` is a unique identifier, `restic_backup_bookstack` for example.

A dashboard is presented on port `8000` that shows basic information for the most recent events by source.

## Usage

This is an example `curl` request for saving a new event.

```
curl -X POST -d 'source=restic_backup_bookstack' -d 'output=successful backup!' -d 'code=0' http://localhost:8000/new
```
