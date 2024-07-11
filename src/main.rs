use std::error::Error;
use std::{f32, iter};
use std::fmt::Write;
use std::fs::read_to_string;
use uname_rs::Uname;
use std::str::FromStr;

fn get_os_name() -> Result<String, Box<dyn Error>> {
    let file = read_to_string("/etc/os-release")?;
    for line in file.lines() {
        // We use split_once because its faster than just split.
        let data = line.split_once('=').ok_or("Error splitting line.")?;
        if data.0 == "PRETTY_NAME" { 
            return Ok(data.1.trim_matches('"').to_string());
        }
    }
    
    Err("No PRETTY_NAME found in your release file.".into())
}

fn get_ram_usage() -> Result<String, Box<dyn Error>> {
    // What we need here is MemTotal and MemAvailable
    // With that we can calculate the used memory
    
    let mut total_memory = -1;
    let mut available_memory = -1;
    
    let file = read_to_string("/proc/meminfo")?;
    for line in file.lines() {
        let data = line.trim().split_once(':').ok_or("meow")?;
        match data.0 { 
            "MemTotal" => 
                total_memory = i32::from_str(data.1.replace("kB", "").trim())?,
            
            "MemAvailable" =>
                available_memory = i32::from_str(data.1.replace("kB", "").trim())?,
            
            _ => {
                // This can save quite a bit of time 
                // We quickly bail if both available memory have been found
                // Skipping the entire rest of the file, since we read like by line.
                if total_memory != -1 && available_memory != -1 { 
                    break;
                }
                
                continue;
            }
        }     
    }
    
    let used_memory = total_memory - available_memory;
    
    // 1 MiB = 1024 KiB
    Ok(format!("{}MiB / {}MiB ", used_memory / 1024, total_memory / 1024))
}

fn get_user_data(uname: &Uname) -> Result<String, Box<dyn Error>> {
    Ok(format!("{}@{}", std::env::var("USER")?, uname.nodename))
}  

fn get_shell() -> Result<String, Box<dyn Error>> {
    Ok(std::env::var("SHELL")?.split('/').last().ok_or("Cannot get the shell name.")?.to_string())
}

fn get_uptime() -> Result<String, Box<dyn Error>> {
    let mut result: String = String::new();
    
    let file = read_to_string("/proc/uptime")?;
    let sec = f32::from_str(file.split_once(char::is_whitespace).ok_or("Can't get uptime data.")?.0)?;
    if sec < 60.0 {
        return Ok(format!("{} Seconds", sec.floor()));
    }
    
    let total_minutes = sec / 60.0;
    let total_hours = total_minutes / 60.0;
    
    let mins = total_minutes % 60.0;
    if total_hours >= 1.0 {
        let hr_str = if total_hours < 2.0 { "Hour" } else { "Hours" };
        write!(result, "{} {hr_str} ", total_hours.floor())?;
    }    
    
    if mins >= 1.0 {
        let min_str = if mins < 2.0 { "Minute" } else { "Minutes" };
        write!(result, "{} {min_str}", mins.floor())?;
    }
    
    Ok(result.to_string())
}

fn print_cl(text: String, colour: i32) -> String {
    format!("\x1b[38;5;{colour}m{text}\x1b[0m")
}

fn main() {
    let text_colour = 219;
    let uname = Uname::new().expect("Failed to get UNAME.");

    let system_data= vec![
        get_user_data(&uname).ok().map(|user| format!("\x1b[38;5;212m{user}\x1b[0m")),
        
        "".to_string().into(),
        
        get_os_name().ok().map(|name| print_cl(format!("OS: {name}"), text_colour)),
        Some(print_cl(format!("KR: {}", uname.release), text_colour)),
        get_uptime().ok().map(|up| print_cl(format!("UP: {up}"), text_colour)),
        get_shell().ok().map(|shell| print_cl(format!("SH: {shell}"), text_colour)),
        get_ram_usage().ok().map(|ram| print_cl(format!("ME: {ram}"), text_colour))
    ].into_iter().flatten().map(Some).chain(iter::repeat(None));
    
    // Colour Data
    let c1 = "\x1b[38;5;112m";
    let c2 = "\x1b[38;5;196m";
    let c3 = "\x1b[0m";
    let ascii_data: [String; 9] = [
        format!("{c1}   (    ((     "),
        format!("{c1} ((  (((  ((   "),
        format!("{c2} #%#{c1}({c2}###{c1}({c2}###   "),
        format!("{c2}##{c1}(({c2}##{c1}({c2}##{c1}({c2}#%#  "),
        format!("{c2}##%#####%####  "),
        format!("{c2} #########%#   "),
        format!("{c2}  ###%#####    "),
        format!("{c2}    ###%#      "),
        format!("{c2}      #        {c3}")
    ];

    for (data, ascii) in system_data.zip(ascii_data) {
        if let Some(data_str) = data {
            println!("{ascii} {data_str}")
        } else {
            println!("{ascii}")
        }
    }
}
