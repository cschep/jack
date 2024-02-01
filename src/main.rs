use regex::Regex;
use std::process::Command;
use std::process::Stdio;
use std::str;

#[derive(Debug)]
struct Sink {
    active: bool,
    id: String,
}

fn main() {
    let headphones = get_sink("Razer");
    let speakers = get_sink("Starship");

    println!("sink for Razer: {:?}", headphones);
    println!("sink for speakers: {:?}", speakers);

    let new_active = if headphones.active {
        speakers.id
    } else {
        headphones.id
    };

    Command::new("wpctl")
        .arg("set-default")
        .arg(new_active)
        .spawn()
        .unwrap();
}

fn get_sink(name: &str) -> Sink {
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
        .arg(name)
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
        active,
        id: id.to_string(),
    }
}

