use clap::Parser;
use csv::Writer;
use std::io::Error as IoError;
use std::io::{ErrorKind, Result as IoResult};
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use tracing::info;

#[derive(Parser, Debug, Clone)]
pub(crate) struct ExperimentCommand {
    /// Resolution
    #[arg(long)]
    resolution: u64,

    /// Command arguments
    #[arg(long)]
    command: String,

    /// Path to the output file
    #[arg(long)]
    output: PathBuf,
}

pub(crate) fn experiment(args: ExperimentCommand) -> IoResult<()> {
    let mut writer = Writer::from_path(args.output)?;
    writer.write_record(&["timestamp", "cpu", "memory"])?;

    let mut system = System::new_all();
    let start_time = time::Instant::now();

    let program = std::env::current_exe()?;
    let arguments: Vec<&str> = args.command.split_whitespace().collect();

    let mut child = Command::new(program)
        .args(&arguments)
        .spawn()
        .map_err(|e| IoError::new(ErrorKind::Other, e))?;

    info!("Experiment started!");

    let pid = Pid::from_u32(child.id());

    loop {
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::new().with_cpu().with_memory(),
        );

        match child.try_wait() {
            Ok(Some(status)) => {
                info!("Experiment ended with status {}!", status);
                break;
            }
            Err(_) => {
                panic!("Error while waiting for children!");
            }
            _ => {}
        }

        if let Some(_) = system.process(pid) {
            let (cpu_usage, memory_usage) = calculate_cpu_and_memory_usage(&system, pid);

            let elapsed_time = start_time.elapsed().as_secs();

            writer.write_record(&[
                elapsed_time.to_string(),
                cpu_usage.to_string(),
                memory_usage.to_string(),
            ])?;

            thread::sleep(time::Duration::from_secs(args.resolution));
        } else {
            info!("Experiment ended!");
            break;
        }
    }

    Ok(())
}

fn calculate_cpu_and_memory_usage(system: &System, pid: Pid) -> (f32, u64) {
    let mut total_cpu = 0.0;
    let mut total_memory = 0u64;

    for process in system.processes() {
        if process.1.pid() == pid {
            total_cpu += process.1.cpu_usage();
            total_memory += process.1.memory() / 1024;
        }

        if process.1.parent() == Some(pid) {
            let (cpu, memory) = calculate_cpu_and_memory_usage(system, process.1.pid());
            total_cpu += cpu;
            total_memory += memory;
        }
    }

    (total_cpu, total_memory)
}
