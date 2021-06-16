use std::collections::HashSet;
use std::io::{self, Write};
use rand::prelude::SliceRandom;

const PREV_CTX: usize = 3;

#[derive(Debug)]
struct Line<'a> {
    speakers: Vec<&'a str>,
    lines: Vec<&'a str>,
}

fn print_line(line: &Line) {
    let line_text = line.lines.join(" ");
    println!("`{}: {}", line.speakers.join(" "), line_text);
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    // speakers
    let transcript = include_str!("../transcript.txt");
    let lines = transcript.split('\n').into_iter();
    let mut script: Vec<Line> = vec![];

    let mut current_speaker = None;
    for line in lines.clone() {
        let mut line_text = line.trim();

        let line_has_speaker = line.starts_with(|c: char| c.is_ascii_alphabetic());
        if line_has_speaker {
            let mut line_parts = line.split(':');

            let speaker = line_parts.next().expect("no name of speaker");
            current_speaker = Some(speaker);

            line_text = line_parts.next().expect("somehow no text?").trim();
        }

        if let Some(current_speaker) = current_speaker {
            if line_has_speaker {
                script.push(Line {
                    speakers: current_speaker.split('\n').collect(),
                    lines: vec![line_text]
                });
            } else {
                let prev_line_idx = script.len() - 1;
                script[prev_line_idx].lines.push(line_text);
            }
        }
    }

    // speakers
    let speakers = lines
        .filter(|line| line.starts_with(|c: char| c.is_ascii_alphabetic()))
        .map(|line| line.split(':').next().expect("no name of speaker"))
        .map(|line| line.split(", ").collect::<Vec<&str>>())
	.collect::<Vec<Vec<&str>>>().into_iter().flatten()
	.collect::<HashSet<&str>>();
    let mut speakers = speakers.iter().collect::<Vec<_>>();
    speakers.sort();
    for (i, speaker) in speakers.iter().enumerate() {
        println!("{}: {}", i + 1, speaker);
    }

    // choice
    let desired_speaker;
    loop {
        print!("choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: Result<usize,_> = choice.trim().parse();
        match choice {
            Ok(n) => {if n <= speakers.len() && n != 0 {desired_speaker = speakers[n-1]; break;}}
	    Err(_) => {continue;}
        }
    }

    let speakers_lines = script
        .iter()
        .enumerate()
        .filter(|(_, line)| line.speakers.contains(desired_speaker))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    loop {
        let line_idx = &speakers_lines.choose(&mut rand::thread_rng());
        let line_idx = line_idx.expect("didn't find a random line");

        let skip_back = usize::min(*line_idx, PREV_CTX);
        for line in &script[line_idx - skip_back..*line_idx] {
            print_line(line);
        }
        let mut bleh = String::new();
        io::stdin().read_line(&mut bleh).unwrap();
        let actual_line = &script[*line_idx];
        print_line(actual_line);
        print!("enter to continue or any key to quit: " );
        io::stdout().flush()?;
        let mut restart = String::new();
        io::stdin().read_line(&mut restart)?;
        if restart.trim() != "" {
	    break;
	}
    }
    Ok(())
}
