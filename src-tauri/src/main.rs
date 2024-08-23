

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{io::{Read, Write as _}, net::{TcpStream, ToSocketAddrs}, vec};
use sysinfo::System;
use std::time::{Duration, Instant};

#[tauri::command]
fn show_memory() -> f64{
    let mut sys = System::new_all();
    sys.refresh_all();

   
    let used: f64 = sys.used_memory() as f64;
    let total: f64 = sys.total_memory() as f64;
    let result : f64 = (used / total) * 100.0 ;
   
    return result.round();
}

#[tauri::command]
fn show_disk() -> f64{
    let mut sys = System::new_all();
    sys.refresh_all();

   
    let used: f64 = sys.used_swap() as f64;
    let total: f64 = sys.total_swap() as f64;
    let result : f64 = (used / total) * 100.0 ;
   
    return result.round();
}

#[tauri::command]
fn show_cpu() -> Vec<Vec<String>>{
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut result:Vec<Vec<String>> = vec![];
    let mut name: Vec<String> = vec![];
    let mut used: Vec<String> = vec![];


    for cpu in sys.cpus() {
        name.push(cpu.name().to_string());
        let mut cpu_usage : String = cpu.cpu_usage().to_string();
        if cpu.cpu_usage().to_string() == "NaN"{
            cpu_usage = "0".to_string();
        }
        used.push(cpu_usage);
    }
    result.push(name);
    result.push(used);

    return result;
}

#[tauri::command]
fn show_process() -> Vec<Vec<String>>{
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut result:Vec<Vec<String>> = vec![];

    for (pid, process) in sys.processes() {
        let mut tmp: Vec<String> = vec![];
        tmp.push(pid.to_string());
        tmp.push(process.name().to_string());
        tmp.push(format!("{:.2}",process.cpu_usage()).to_string());
        tmp.push((process.memory() / 1024).to_string());
        tmp.push(process.start_time().to_string());
        // tmp.push(format!("{:?}",process.exe().unwrap()));
        result.push(tmp);
    }   
    return result;
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_memory,show_disk,show_cpu,show_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
