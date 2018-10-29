/// The main analysis
use std::collections::HashMap;

use super::avanza;

#[derive(Default)]
pub struct Stats {
    types: HashMap<String, f64>,
}

impl Stats {
    pub fn format(&self) -> String {
        let total: f64 = self.types.values().sum();
        let lines = self
            .types
            .iter()
            .map(|(name, value)| {
                format!(
                    "{:13} {:>13.1}  {:>4.1}%",
                    name,
                    value,
                    100. * value / total
                )
            }).collect::<Vec<String>>()
            .join("\n");
        format!(
            "Your portfolio consists of\n{}\n{:13} {:>13.1}  sek.",
            lines, "Total", total
        )
    }

    fn track(&mut self, name: &str, value: f64) {
        *self.types.entry(name.to_string()).or_default() += value;
    }
}

pub fn calculate_stats(auth: &super::Auth) -> Result<Stats, serde_json::Value> {
    let mut talk = avanza::Talk::new(&auth).map_err(|e| serde_json::json!(e))?;
    let positions = talk.command(&["getpositions"])?;
    let mut stats = Stats::default();
    let mut not_supported = Vec::new();
    for category in positions["instrumentPositions"].as_array().unwrap() {
        match category["instrumentType"].as_str().unwrap() {
            "STOCK" => {
                for position in category["positions"].as_array().unwrap() {
                    let value = position["value"].as_f64().unwrap();
                    stats.track("Aktier", value);
                }
            }
            "FUND" => {
                for position in category["positions"].as_array().unwrap() {
                    let value = position["value"].as_f64().unwrap();
                    let orderbookid = position["orderbookId"].as_str().unwrap();
                    let instrument = talk.command(&["getinstrument", "FUND", orderbookid])?;
                    stats.track(instrument["type"].as_str().unwrap(), value);
                }
            }
            instrument_type => not_supported.push(instrument_type),
        }
    }
    if !not_supported.is_empty() {
        println!(
            "NOTE: The application only counts STOCK and FUND instruments, not {}.",
            not_supported.join(", ")
        );
    }
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formating() {
        let mut stats = Stats::default();
        stats.track("Test", 10004.);
        stats.track("Test", 1231.23);
        stats.track("Test", 5243.51);
        stats.track("Bla", 3424.);
        stats.track("Bla", 23464.);
        stats.track("Bla", 32.45);
        stats.track("Blu", 0.);
        let format = stats.format();
        println!("Format example:\n{}", format);
    }
}
