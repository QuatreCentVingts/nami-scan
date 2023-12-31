
use std::io::*;

use colored::Colorize;

pub fn debug(content: String){
    println!("{} {} {}{}", "[", "DEBUG".truecolor(136, 227, 242), "] - ", content.trim_end());
}

pub fn error(content: String){
    println!("{} {} {}{}", "[", "ERROR".red(), "] - ", content.trim_end());
}

pub fn info(content: String){
    println!("{} {} {}{}", "[", "INFO".green(), "] - ", content.trim_end());
}