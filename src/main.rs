use regex::Regex;
use std::process::Command;
use std::process::Stdio;
use std::str;

#[derive(Debug)]
struct Sink {
    name: String,
    active: bool,
    id: String,
}

fn main() {
    let headphones = get_sink("Razer", "headphones");
    let speakers = get_sink("Starship", "speakers");

    // println!("sink for Razer: {:?}", headphones);
    // println!("sink for speakers: {:?}", speakers);

    let new_active = if headphones.active {
        speakers
    } else {
        headphones
    };

    println!("settings {} as active!", new_active.name);

    Command::new("wpctl")
        .arg("set-default")
        .arg(new_active.id)
        .spawn()
        .unwrap();
}

fn get_sink(device_name: &str, name: &str) -> Sink {
    let ps_child = Command::new("wpctl")
        .arg("status")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_one = Command::new("grep")
        .arg("-A5")
        .arg("-m 1")
        .arg("Sinks")
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("grep")
        .arg(device_name)
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();

    let line = String::from_utf8(output.stdout)
        .unwrap()
        .replace("│", "")
        .trim()
        .to_string();
    let active = line.starts_with("*");

    let re = Regex::new(r"[0-9]+").unwrap();
    let Some(caps) = re.captures(&line) else {
        panic!("now you have two problems");
    };
    let id = &caps[0];

    Sink {
        name: name.to_string(),
        active,
        id: id.to_string(),
    }
}
