# Exploring Tokio
Simple app to explore rust async runtime `tokio` that uses `broadcast` to synchronize asynchronous tasks with a common set of `events`

Start -> TaskAFinished -> TaskBFinished -> TaskCFinished -> Complete

## Run the app
```shell
cargo run
```

## Examples
Examples are the same found in the `tokio` documentation for `broadcast`, `mpsc`, `oneshot` and `watch`.

Run the examples as follow

```shell
cargo run --example EXAMPLE_NAME
```

## Logs
The app does not print to the console but uses log and syslog.

### Journal
More modern Linux version system logs are sent to the journal and can be monitored using the following command
```shell
journalctl -f
```

### Syslog
Older versions of linux uses syslog and can be monitored as follow
```shell
tail -f /var/log/syslog
```