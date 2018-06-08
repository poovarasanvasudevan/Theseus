// Writes the "/cfg/grub_test.cfg" file

extern crate getopts;

use getopts::Options;
use std::fs;
use std::io::Write;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file path, e.g., \"/my/dir/grub.cfg\"", "OUTPUT_PATH");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage("cargo run -- ", opts);
        process::exit(0);
    }

    // Require input directory 
    let input_directory = match matches.free.len() {
        0 => {
            eprintln!("No input directory");
            process::exit(-1);
        },
        1 => matches.free[0].clone(), 
        _ => { 
            eprintln!("Too many arguments entered");
            process::exit(-1);
        },
    };
    
    let grub_cfg_string = create_grub_cfg_string(input_directory);
    
    // Write to file 
    if matches.opt_present("o") {
        let output_file_path = match matches.opt_str("o") {
            Some(s) => s, 
            None    => process::exit(-1)
        };
        write_to_file(grub_cfg_string, output_file_path);
    }
    // Write to stdout by default
    else {
        println!("{}", grub_cfg_string);
    }

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] INPUT_DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}

fn create_grub_cfg_string(input_directory: String) -> String {
    // Creates string to write to grub.cfg file by looking through all files in input_directory
    let mut to_file = String::new();
    
    let mut path_to_exe = std::env::current_exe().unwrap_or(std::path::PathBuf::new());
    // go up three directories to remove the "target/<build_mode>/name"
    path_to_exe.pop(); path_to_exe.pop(); path_to_exe.pop();

    to_file.push_str("### This file has been autogenerated, do not manually modify it!\n");
    to_file.push_str(&format!("### Generated by program: \"{}\"\n", path_to_exe.display()));
    to_file.push_str(&format!("### Input directory: \"{}\"\n\n", &input_directory));
    to_file.push_str("set timeout=0\n");
    to_file.push_str("set default=0\n\n");
    to_file.push_str("menuentry \"Theseus OS\" {\n");
    to_file.push_str("\tmultiboot2 /boot/kernel.bin \n");

    to_file.push_str(&format!("\tmodule2 /modules/{0:25}\t\t{1:20}\n", "__k_nano_core.sym", "__k_nano_core"));
    
    if let Ok(paths) = fs::read_dir(input_directory) {
        for path in paths {
            if let Ok(path) = path {
                let cur_path = path.path().clone();
                if let Some(file_name) = cur_path.file_name() {
                if let Some(file_stem) = cur_path.file_stem() {
                    if let Some(file_name_str) = file_name.to_str() {
                    if let Some(file_stem_str) = file_stem.to_str() {
                        if file_name_str.contains("nano_core") || !file_name_str.starts_with("__") {
                            continue;
                        }
                        to_file.push_str(&format!("\tmodule2 /modules/{0:25}\t\t{1:20}\n", file_name_str, file_stem_str));
                    }
                    } 
                } 
                } 
            } 
        } 
    }
    else{
        eprintln!("Invalid directory");
        process::exit(-1);
    }

    to_file.push_str("\n\tboot\n}\n");
    to_file
}

fn write_to_file(to_file: String, output_file_path: String) {
    if let Ok(mut file) = fs::File::create(output_file_path) {
        if file.write(to_file.as_bytes()).is_ok(){ process::exit(0); }
    }
}