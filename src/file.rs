use crate::shell::Shell;
use log::{debug, trace};
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::vec::Vec;

// pub fn copy_once(shell: &Shell, target_path: &str) -> io::Result<()> {
//     let uuid = fs::read_to_string(Path::new("shells/uuid.txt"))?;
//     let uuid = uuid.trim_end();
// }

pub fn copy_for(shell: &Shell, target_path: &str) -> io::Result<()> {
    let uuid = fs::read_to_string(Path::new("shells/uuid.txt"))?;
    let uuid = uuid.trim_end();

    let semver = env!("CARGO_PKG_VERSION");

    let marker_fst = format!("# Start of jmp-rs block: {} {}", semver, uuid);
    let marker_lst = format!("# End of jmp-rs block: {} {}", semver, uuid);

    // Step 1: create a copy of the original target file
    let file_from_path = target_path;
    let file_into_path = format!("{}.orig", target_path);

    debug!("file_from_path: {}", file_from_path);
    debug!("file_into_path: {}", file_into_path);

    fs::copy(&file_from_path, &file_into_path)?;

    // Step 2: append a section of our shell code to target file
    let shell_name = match shell {
        Shell::ZSH => "jmp-rs.zsh",
    };
    let shell_path = format!("shells/{}", shell_name);

    let file_from_path = match Path::new(&shell_path).to_str() {
        Some(path) => path,
        None => panic!("shell name is not UTF-8, this never happens"),
    };
    let file_into_path = target_path;

    let reader = File::open(&file_from_path)?;
    let mut writer =
        OpenOptions::new().append(true).open(&file_into_path)?;
    writeln!(writer, "{}", marker_fst)?;
    for line in io::BufReader::new(reader).lines() {
        let line = line?;

        writeln!(writer, "{}", line)?;
    }
    writeln!(writer, "{}", marker_lst)?;

    writer.flush()?;

    Ok(())
}

pub fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    trace!("Enter read_lines({})", file_path);

    let file = File::open(file_path)?;

    let mut lines = Vec::new();
    for (number, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;

        if line.is_empty() {
            trace!("line {}: empty", number);
            continue;
        }

        let parts: Vec<&str> = line.trim_start().splitn(2, '#').collect();

        if parts[0].is_empty() && parts.len() > 1 {
            trace!("line {}: comment-only", number);
            continue;
        } else if parts[0].is_empty() {
            trace!("line {}: whitespace-only", number);
            continue;
        } else {
            trace!("line {}: has content!", number);
            lines.push(String::from(parts[0]));
        }
    }

    trace!("Leave read_lines({}) (Ok)", file_path);

    Ok(lines)
}
