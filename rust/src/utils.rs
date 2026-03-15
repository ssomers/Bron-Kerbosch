use crossbeam_channel::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

// Source - https://stackoverflow.com/a/67834588
pub fn formatted(num: usize) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

pub fn parse_positive_int(value: &str) -> usize {
    let (numstr, factor) = if let Some(megas) = value.strip_suffix('M') {
        (megas, 1_000_000)
    } else if let Some(kilos) = value.strip_suffix('k') {
        (kilos, 1_000)
    } else {
        (value, 1)
    };
    let num: usize = numstr
        .parse()
        .unwrap_or_else(|err| panic!("{numstr} is not a positive integer ({err})"));
    num * factor
}

// Do something but warn if it takes too long.
pub fn do_timely<A: FnOnce() -> R, R>(action: A, description: String) -> R {
    fn ticker(end_rx: Receiver<()>, description: String) {
        let interval = Duration::from_secs(3);
        let mut warnings = 0;
        while matches!(
            end_rx.recv_timeout(interval),
            Err(RecvTimeoutError::Timeout)
        ) {
            warnings += 1;
            eprintln!(
                "  {} seconds in, {}",
                interval.as_secs() * warnings,
                description
            );
        }
    }

    let (ticker_tx, ticker_rx) = crossbeam_channel::bounded(0);
    thread::spawn(move || ticker(ticker_rx, description));
    let result = action();
    ticker_tx.send(()).unwrap();
    result
}
