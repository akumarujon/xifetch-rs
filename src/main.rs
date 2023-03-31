use std::process::Command;

mod color;
mod config;

fn main() {
    config::config();
    let fetch = color::colorize(print_fetch());

    println!("{}", fetch);
}

fn get_os_name() -> Option<String> {
    let output = Command::new("uname").arg("-o").output().ok()?;
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn get_kernel_version() -> Option<String> {
    let output = Command::new("uname").arg("-r").output().ok()?;
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn get_host_name() -> Option<String> {
    let output = Command::new("hostname").output().ok()?;
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn get_uptime() -> Option<String> {
    let output = Command::new("uptime").output().ok()?;
    let uptime_str = String::from_utf8(output.stdout).ok()?.trim().to_string();
    let parts: Vec<&str> = uptime_str.split(' ').collect();
    Some(parts[0..2].join(" "))
}

fn get_cpu_info() -> Option<String> {
    let output = Command::new("lscpu").output().ok()?;
    let cpu_info_str = String::from_utf8(output.stdout).ok()?;
    let model_name_line = cpu_info_str
        .lines()
        .find(|line| line.contains("Model name"))?;
    let model_name = model_name_line.split(':').nth(1)?.trim();
    let cpu_cores_line = cpu_info_str.lines().find(|line| line.contains("CPU(s)"))?;
    let cpu_cores = cpu_cores_line.split(':').nth(1)?.trim();
    Some(format!("{} ({} cores)", model_name, cpu_cores))
}

fn get_memory_info() -> Option<String> {
    let output = Command::new("free").arg("-h").output().ok()?;
    let memory_info_str = String::from_utf8(output.stdout).ok()?;
    let lines: Vec<&str> = memory_info_str.lines().collect();
    let mem_line = lines.iter().find(|line| line.contains("Mem:"))?;
    let mem_parts: Vec<&str> = mem_line.split_whitespace().collect();
    let total_mem = mem_parts[1];
    let used_mem = mem_parts[2];
    Some(format!("{}  / {} ", used_mem, total_mem))
}

fn print_fetch() -> String {
    let os_name = get_os_name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = get_kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = get_host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime = get_uptime().unwrap_or_else(|| "Unknown".to_string());
    let cpu_info = get_cpu_info().unwrap_or_else(|| "Unknown".to_string());
    let memory_info = get_memory_info().unwrap_or_else(|| "Unknown".to_string());
    let shell_name = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());

    let mut result: String = String::new();

    result.push_str(format!("OS: {}\n", os_name).as_str());
    result.push_str(format!("Shell: {}\n", shell_name).as_str());
    result.push_str(format!("Kernel: {}\n", kernel_version).as_str());
    result.push_str(format!("Host: {}\n", host_name).as_str());
    result.push_str(format!("Uptime: {}\n", uptime).as_str());
    result.push_str(format!("CPU: {}\n", cpu_info).as_str());
    result.push_str(format!("Memory: {}\n", memory_info).as_str());

    return result;
}
