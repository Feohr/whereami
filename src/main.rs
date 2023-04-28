use std::env::consts::ARCH;
use sys_info;
use itertools::Itertools;
use colored::Colorize;

// Get timeval in seconds
fn get_timeval(tv_sec: f64, tv_usec: f64) -> f64 {
    tv_sec + ((1.0 / 1_000_000.0) * tv_usec)
}

// Byte sizes.
const BYTES: [f64; 6] = [
    1024.,                    // 1  MB
    1_048_574.,               // 1  GB
    1_073_741_824.,           // 1  TB
    1_099_511_627_776.,       // 1  PB
    1_125_899_906_842_624.,   // 1  EB
    1_125_899_906_842_624_0., // 10 EB is the limit
];

// To get right data suffix and size
fn get_data_unit(data: u64) -> String {
    match data as f64 {
        data if data < BYTES[0] => return format!("{}\tKB", data),
        data if data >= BYTES[0] && data < BYTES[1] => {
            return format!("{:.2}\tMB", data / BYTES[0])
        }
        data if data >= BYTES[1] && data < BYTES[2] => {
            return format!("{:.2}\tGB", data / BYTES[1])
        }
        data if data >= BYTES[2] && data < BYTES[3] => {
            return format!("{:.2}\tTB", data / BYTES[2])
        }
        data if data >= BYTES[3] && data < BYTES[4] => {
            return format!("{:.2}\tPB", data / BYTES[3])
        }
        data if data >= BYTES[4] && data < BYTES[5] => {
            return format!("{:.2}\tEB", data / BYTES[4])
        }
        // I don't think we're going to come across more than 10 EB data anytime soon
        _ => return "Cannot infer data size".to_string(),
    }
}

// Returns ANSI info
fn get_ansi_color(ansi: Option<String>) -> Option<String> {
    if let Some(ansi) = ansi {
        return Some(format!("\x1b[{ansi}m███████\x1b[0m"));
    }
    None
}

// Returns string in blue color
fn get_string_blue(string: Option<String>) -> Option<String> {
    if let Some(string) = string {
        return Some(format!("{}", string.blue().italic()));
    }
    None
}

// Sends back release info
fn get_release_info(name: &str, info: Option<String>) -> String {
    if let Some(info) = info {
        return format!("\t{}\t{}\n", name, info);
    }
    // This is just for insurance
    // zip in "get_release_note()" should've removed all None values
    String::new()
}

// To get release note data in format
#[cfg(target_os = "linux")]
fn get_release_note() -> Result<String, sys_info::Error> {
    let release_notes = sys_info::linux_os_release()?;
    let mut output = String::from("Release notes:\n");

    // Creating a vec to iterate over.
    // I now realize how ridiculous this is but nevermind.
    let info_vec: Vec<Option<String>> = vec![
        Some(String::new()), // Empty string for sub-header
        release_notes.id,
        release_notes.id_like,
        release_notes.pretty_name,
        release_notes.version,
        release_notes.version_id,
        release_notes.version_codename,
        get_ansi_color(release_notes.ansi_color),
        release_notes.logo,
        release_notes.cpe_name,
        release_notes.build_id,
        release_notes.variant,
        release_notes.variant_id,
        Some(String::new()), // Empty string for sub-header
        get_string_blue(release_notes.home_url),
        get_string_blue(release_notes.documentation_url),
        get_string_blue(release_notes.support_url),
        get_string_blue(release_notes.bug_report_url),
        get_string_blue(release_notes.privacy_policy_url),
    ];

    // Could I have been more clever about this? yes.
    // Will I hear any of the complaints? no. Shut up.
    let info_vec_str: Vec<&str> = vec![
        "Distribution:",
        "\tID:\t",
        "\tID type:",
        "\tName:\t",
        "\tVersion:",
        "\tVersion ID:",
        "\tVersion Code:",
        "\tANSI color:",
        "\tLogo:\t",
        "\tCPE name:",
        "\tBuild ID:",
        "\tVariant:",
        "\tVariant ID:",
        "URL:",
        "\tHome:\t",
        "\tDocumentation:",
        "\tSupport:",
        "\tBug report:",
        "\tPrivacy policy:",
    ];

    // Applying zip and iterating over it
    for (info_name, info_item) in info_vec_str.iter().zip(info_vec) {
        output.push_str(get_release_info(info_name, info_item).as_str());
    }

    Ok(output)
}

// sys_info crate is only set up to get linux release note info at the moment
#[cfg(not(target_os = "linux"))]
fn get_release_note() -> Result<String, sys_info::Error> {
    Ok("Release notes:\t\t\tNo notes".to_string())
}

// Get memory information
fn get_mem_info(mem_info: &sys_info::MemInfo) -> String {
    format!(
        "Memory info:\n\
        \tMemory:\n\
        \t\tTotal:\t\t{ttl_ram}\n\
        \t\tUsed:\t\t{used_ram}\n\
        \t\tAvailable:\t{avail_ram}\n\
        \t\tFree:\t\t{free_ram}\n\
        \tSwap:\n\
        \t\tTotal swap:\t{ttl_swp}\n\
        \t\tFree swap:\t{free_swp}\n\
        \n\
        \tBuffer:\t\t\t{buff}\n\
        \tCache:\t\t\t{cache}",
        ttl_ram = get_data_unit(mem_info.total),
        used_ram = get_data_unit(mem_info.total - mem_info.avail),
        avail_ram = get_data_unit(mem_info.avail),
        free_ram = get_data_unit(mem_info.free),
        ttl_swp = get_data_unit(mem_info.swap_total),
        free_swp = get_data_unit(mem_info.swap_free),
        buff = get_data_unit(mem_info.buffers),
        cache = get_data_unit(mem_info.cached)
    )
}

// Get disk information
fn get_disk_info(disk_info: &sys_info::DiskInfo) -> String {
    format!(
        "Disk info:\n\
        \tTotal:\t\t\t{}\n\
        \tFree:\t\t\t{}\n",
        get_data_unit(disk_info.total),
        get_data_unit(disk_info.free)
    )
}

// Help note
fn help_note() -> String {
    format!(
        "{}\n\
        \twhereami [options]\n\n\
        Simple program to get quick info on the current machine\n\n\
        {}\n\
        \t-d,\t--disk\t\t\tget hard-disk information\n\
        \t-r,\t--release-notes\t\tget the OS release notes\n\
        \t-m,\t--memory\t\tget ram and swap memory information\n\
        \t-h,\t--help\t\t\tdisplay this help\n\n\
        {}:\n\
        \tTotal\t\t\t\tThe total size of RAM\n\
        \tUsed\t\t\t\tThe total RAM in use\n\
        \tAvailable\t\t\tThe total available memory\n\
        \tFree\t\t\t\tThe total free memory\n\
        \tTotal swap\t\t\tThe total swap memory\n\
        \tFree swap\t\t\tThe total unused swap memory\n\
        \tBuffer\t\t\t\tThe total buffer memory\n\
        \tCache\t\t\t\tThe total cache memory\n\n\
        {}:\n\
        \tTotal\t\t\t\tThe total size of the disk\n\
        \tFree\t\t\t\tTotal available disk space\n",
        "Usage".green().bold(),
        "Options".green().bold(),
        "AVAILABLE INFO IN MEMORY INFO".bold().underline(),
        "AVAILABLE INFO IN DISK INFO".bold().underline(),
    )
}

// Main function to get info
fn get_sys_info() -> Result<(), sys_info::Error> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0); // To remove the first argument which is itself

    let boottime = sys_info::boottime()?;
    let cpu_num = sys_info::cpu_num()?;
    let cpu_speed = sys_info::cpu_speed()?;
    let disk_info = sys_info::disk_info()?;
    let hostname = sys_info::hostname()?;
    let mem_info = sys_info::mem_info()?;
    let os_release = sys_info::os_release()?;
    let os_type = sys_info::os_type()?;
    let release_notes = get_release_note()?;

    // If there are arguments
    if args.len() <= 0 {
        return Ok(println!(
            "Architecture:\t\t\t{}\n\
            Boot time:\t\t\t{:.6?} seconds\n\
            CPU count:\t\t\t{}\n\
            CPU speed:\t\t\t{} MHz\n\
            Host:\t\t\t\t{}\n\
            {}\
            OS type:\t\t\t{}\n\
            Release version:\t\t{}\n\
            {}\
            {}",
            ARCH.bright_yellow(),
            get_timeval(boottime.tv_sec as f64, boottime.tv_usec as f64),
            cpu_num,
            cpu_speed,
            hostname.bright_white(),
            get_disk_info(&disk_info),
            os_type,
            os_release,
            release_notes,
            get_mem_info(&mem_info)
        ))
    }

    // To remove duplicate arguments
    args = args.into_iter().unique().collect();
    for arg in args {
        // options
        match arg.as_str() {
            "-d" | "--disk"          => print!("{}",    get_disk_info(&disk_info)),
            "-r" | "--release-notes" => print!("{}",    release_notes),
            "-m" | "--memory"        => println!("{}",  get_mem_info(&mem_info)),
            "-h" | "--help"          => print!("{}",    help_note()),
            _ => {
                print!(
                    "Invalid option \'{}\'\nrunning \'whereami --help\' \
                    for more information\n\n{}",
                    arg,
                    help_note()
                    );
                break;
            }
        }
    }

    return Ok(());
}

fn main() {
    match get_sys_info() {
        Ok(_) => (),
        Err(err) => println!("{}", format!("\n\n\n{}", err).red().bold()),
    }
}
