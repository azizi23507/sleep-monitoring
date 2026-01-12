use tokio::time::{sleep, Duration};
use chrono::{Local, Timelike};
use std::process::Command;
use tracing as log;

/// ML Analysis Scheduler
/// 
/// This module handles the scheduled execution of the ML sleep quality analysis.
/// The analysis runs daily at 8:00 AM to process the previous night's sensor data.
///
/// The scheduler operates independently of the web server and continues running
/// in the background throughout the application lifecycle.

/// Starts the ML analysis scheduler
/// 
/// This function spawns a background task that runs indefinitely, checking
/// every minute whether it's time to trigger the ML analysis. When the current
/// time matches the scheduled time (8:00 AM), it executes the Python ML script.
///
/// The scheduler is non-blocking and runs in a separate Tokio task, ensuring
/// the web server remains responsive.
pub async fn start_ml_scheduler() {
    tokio::spawn(async {
        log::info!("ML Analysis Scheduler started - will run daily at 08:00");
        
        loop {
            // Get current local time
            let now = Local::now();
            let current_hour = now.hour();
            let current_minute = now.minute();
            
            // Check if it's 8:00 AM
            if current_hour == 8 && current_minute == 0 {
                log::info!("Triggering ML analysis at 08:00");
                run_ml_analysis().await;
                
                // Sleep for 2 minutes to avoid running multiple times in the same minute
                sleep(Duration::from_secs(120)).await;
            }
            
            // Check every 60 seconds
            sleep(Duration::from_secs(60)).await;
        }
    });
}

/// Executes the ML analysis Python script
///
/// This function calls the external Python script that performs sleep quality analysis.
/// The script reads sensor data from the database, runs the ML model, and writes
/// results back to the database.
///
/// The function runs asynchronously and logs the execution status for monitoring.
async fn run_ml_analysis() {
    log::info!("Starting ML sleep quality analysis...");
    
    // Path to the ML Python script
    // In Docker: /app/ml/sleep_score_ml.py
    // In development: ../ml/sleep_score_ml.py
    let ml_script_path = if std::path::Path::new("/app/ml/sleep_score_ml.py").exists() {
        "/app/ml/sleep_score_ml.py"
    } else {
        "../ml/sleep_score_ml.py"
    };
    
    // Execute the Python script
    match Command::new("python3")
        .arg(ml_script_path)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                log::info!("ML analysis completed successfully");
                log::debug!("ML output: {}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("ML analysis failed: {}", stderr);
            }
        }
        Err(e) => {
            log::error!("Failed to execute ML script: {}", e);
        }
    }
}

/// Alternative implementation: Schedule at specific time with better precision
///
/// This version calculates the exact time until the next 8:00 AM and sleeps
/// until then, which is more efficient than checking every minute.
#[allow(dead_code)]
pub async fn start_ml_scheduler_precise() {
    tokio::spawn(async {
        log::info!("ML Analysis Scheduler (precise mode) started");
        
        loop {
            // Calculate time until next 8:00 AM
            let now = Local::now();
            let mut next_run = now
                .date_naive()
                .and_hms_opt(8, 0, 0)
                .unwrap()
                .and_local_timezone(now.timezone())
                .unwrap();
            
            // If it's already past 8:00 today, schedule for tomorrow
            if next_run <= now {
                next_run = next_run + chrono::Duration::days(1);
            }
            
            let duration_until_next_run = (next_run - now).to_std().unwrap();
            
            log::info!(
                "Next ML analysis scheduled for: {}",
                next_run.format("%Y-%m-%d %H:%M:%S")
            );
            
            // Sleep until the scheduled time
            sleep(duration_until_next_run).await;
            
            // Run the analysis
            log::info!("Executing scheduled ML analysis");
            run_ml_analysis().await;
        }
    });
}
