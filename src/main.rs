use reqwest;
use std::process::Command;
use std::time::Duration;
use tokio::time;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionState {
    Connected,
    Disconnected,
}

struct ConnectionMonitor {
    client: reqwest::Client,
    current_state: ConnectionState,
}

impl ConnectionMonitor {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            current_state: ConnectionState::Connected, // Assume connected initially
        }
    }

    async fn check_connection(&self) -> bool {
        match self.client.get("http://example.com").send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    fn notify_disconnected(&self) {
        // Use macOS say command
        let _ = Command::new("say")
            .arg("Internet connection lost")
            .spawn();

        // Show system notification
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("display notification \"Internet connection lost\" with title \"Connection Monitor\" sound name \"Basso\"")
            .spawn();

        println!("🔴 Internet connection lost!");
    }

    fn notify_reconnected(&self) {
        // Use macOS say command
        let _ = Command::new("say")
            .arg("Internet connection restored")
            .spawn();

        // Show system notification
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("display notification \"Internet connection restored\" with title \"Connection Monitor\" sound name \"Glass\"")
            .spawn();

        println!("🟢 Internet connection restored!");
    }

    async fn monitor(&mut self) {
        println!("🚀 Starting internet connection monitor...");
        println!("📡 Checking connection every minute");
        println!("Press Ctrl+C to stop\n");

        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            let is_connected = self.check_connection().await;
            let new_state = if is_connected {
                ConnectionState::Connected
            } else {
                ConnectionState::Disconnected
            };

            // Only notify on state changes
            match (self.current_state, new_state) {
                (ConnectionState::Connected, ConnectionState::Disconnected) => {
                    self.notify_disconnected();
                }
                (ConnectionState::Disconnected, ConnectionState::Connected) => {
                    self.notify_reconnected();
                }
                (ConnectionState::Connected, ConnectionState::Connected) => {
                    println!("✅ Connection OK - {}", chrono::Local::now().format("%H:%M:%S"));
                }
                (ConnectionState::Disconnected, ConnectionState::Disconnected) => {
                    println!("❌ Still disconnected - {}", chrono::Local::now().format("%H:%M:%S"));
                }
            }

            self.current_state = new_state;
        }
    }
}

#[tokio::main]
async fn main() {
    let mut monitor = ConnectionMonitor::new();
    monitor.monitor().await;
}