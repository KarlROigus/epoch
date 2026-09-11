use epoch_common::{EntryResponse, ErrorResponse, StatusResponse, StopRequest, UpdateEntryRequest};
use serde::Deserialize;
use std::env;
use std::fs;
use std::process;

#[derive(Deserialize)]
struct Config {
    server_url: String,
    api_key: String,
}

fn load_config() -> Config {
    let home = env::var("HOME").expect("HOME not set");
    let path = format!("{}/.epoch/config.toml", home);
    let content = fs::read_to_string(&path).unwrap_or_else(|_| {
        eprintln!("Config not found at {}", path);
        eprintln!("Create it with:");
        eprintln!();
        eprintln!("  mkdir -p ~/.epoch");
        eprintln!("  cat > ~/.epoch/config.toml << 'EOF'");
        eprintln!("  server_url = \"http://localhost:3000\"");
        eprintln!("  api_key = \"your-secret-key\"");
        eprintln!("  EOF");
        process::exit(1);
    });
    toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Invalid config: {}", e);
        process::exit(1);
    })
}

fn request(
    config: &Config,
    method: reqwest::Method,
    path: &str,
) -> reqwest::blocking::RequestBuilder {
    let url = format!("{}{}", config.server_url, path);
    reqwest::blocking::Client::new()
        .request(method, &url)
        .header("Authorization", format!("Bearer {}", config.api_key))
}

fn format_duration(start: &chrono::NaiveDateTime, end: &chrono::NaiveDateTime) -> String {
    let duration = *end - *start;
    let total_secs = duration.num_seconds();
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

fn cmd_start(config: &Config) {
    let resp = request(config, reqwest::Method::POST, "/api/timer/start")
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    if resp.status() == 409 {
        eprintln!("A timer is already running. Stop it first with: epoch stop \"description\"");
        process::exit(1);
    }

    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().unwrap();
        eprintln!("Error: {}", err.error);
        process::exit(1);
    }

    let entry: EntryResponse = resp.json().unwrap();
    println!("Timer started at {}", entry.started_at.format("%H:%M:%S"));
}

fn cmd_stop(config: &Config, description: &str) {
    let resp = request(config, reqwest::Method::POST, "/api/timer/stop")
        .json(&StopRequest {
            description: description.to_string(),
        })
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    if resp.status() == 404 {
        eprintln!("No running timer to stop.");
        process::exit(1);
    }

    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().unwrap();
        eprintln!("Error: {}", err.error);
        process::exit(1);
    }

    let entry: EntryResponse = resp.json().unwrap();
    let duration = format_duration(&entry.started_at, &entry.stopped_at.unwrap());
    println!("Stopped: {} ({})", description, duration);
}

fn cmd_status(config: &Config) {
    let resp = request(config, reqwest::Method::GET, "/api/timer/status")
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    let status: StatusResponse = resp.json().unwrap();

    if !status.running {
        println!("No timer running.");
        return;
    }

    let entry = status.entry.unwrap();
    let now = chrono::Utc::now().naive_utc();
    let duration = format_duration(&entry.started_at, &now);
    println!("Running for {} (started at {})", duration, entry.started_at.format("%H:%M:%S"));
}

fn cmd_log(config: &Config) {
    let resp = request(config, reqwest::Method::GET, "/api/entries")
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    let entries: Vec<EntryResponse> = resp.json().unwrap();

    if entries.is_empty() {
        println!("No entries today.");
        return;
    }

    for entry in &entries {
        let desc = entry.description.as_deref().unwrap_or("(running)");
        let time = if let Some(end) = &entry.stopped_at {
            format!(
                "{} - {}  {}",
                entry.started_at.format("%H:%M"),
                end.format("%H:%M"),
                format_duration(&entry.started_at, end)
            )
        } else {
            let now = chrono::Utc::now().naive_utc();
            format!(
                "{} - ...    {}",
                entry.started_at.format("%H:%M"),
                format_duration(&entry.started_at, &now)
            )
        };
        println!("  {}  {}", time, desc);
    }
}

fn cmd_delete(config: &Config, id: &str) {
    let path = format!("/api/entries/{}", id);
    let resp = request(config, reqwest::Method::DELETE, &path)
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    if resp.status() == 404 {
        eprintln!("Entry not found.");
        process::exit(1);
    }

    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().unwrap();
        eprintln!("Error: {}", err.error);
        process::exit(1);
    }

    println!("Entry deleted.");
}

fn cmd_edit(config: &Config, id: &str, description: &str) {
    let path = format!("/api/entries/{}", id);
    let resp = request(config, reqwest::Method::PUT, &path)
        .json(&UpdateEntryRequest {
            description: Some(description.to_string()),
        })
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect: {}", e);
            process::exit(1);
        });

    if resp.status() == 404 {
        eprintln!("Entry not found.");
        process::exit(1);
    }

    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().unwrap();
        eprintln!("Error: {}", err.error);
        process::exit(1);
    }

    println!("Entry updated.");
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  epoch start                        Start a timer");
    eprintln!("  epoch stop \"description\"            Stop timer with description");
    eprintln!("  epoch status                       Show running timer");
    eprintln!("  epoch log                          Show today's entries");
    eprintln!("  epoch delete <id>                  Delete an entry");
    eprintln!("  epoch edit <id> --desc \"text\"       Edit an entry's description");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let config = load_config();

    match args[1].as_str() {
        "start" => cmd_start(&config),
        "stop" => {
            if args.len() < 3 {
                eprintln!("Usage: epoch stop \"description\"");
                process::exit(1);
            }
            cmd_stop(&config, &args[2]);
        }
        "status" => cmd_status(&config),
        "log" => cmd_log(&config),
        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: epoch delete <id>");
                process::exit(1);
            }
            cmd_delete(&config, &args[2]);
        }
        "edit" => {
            if args.len() < 5 || args[3] != "--desc" {
                eprintln!("Usage: epoch edit <id> --desc \"new description\"");
                process::exit(1);
            }
            cmd_edit(&config, &args[2], &args[4]);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
