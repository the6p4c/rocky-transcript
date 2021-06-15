use std::collections::HashSet;
use std::io::{self, Write};
use rand::prelude::SliceRandom;

const PREV_CTX: usize = 3;

#[derive(Debug)]
struct Line<'a> {
    speaker: &'a str,
    lines: Vec<&'a str>,
}

fn print_line(line: &Line) {
    let line_text = line.lines.join(" ");
    println!("{}: {}", line.speaker, line_text);
}

fn main() {
    // speakers
    let transcript = include_str!("../transcript.txt");
    let mut script: Vec<Line> = vec![];

    let mut current_speaker = None;
    for line in transcript.split('\n') {
        let mut line_text = line.trim();

        let line_has_speaker = line.starts_with(|c: char| c.is_ascii_alphabetic());
        if line_has_speaker {
            let mut line_parts = line.split(':');

            let speaker = line_parts.next().expect("no name of speaker");
            current_speaker = Some(speaker);

            line_text = line_parts.next().expect("somehow no text?").trim();
        }

        if let Some(current_speaker) = current_speaker {
            let is_first = script.len() == 0;
            let need_new = if is_first {
                true
            } else {
                let prev_line_idx = script.len() - 1;
                let prev_line_same = script[prev_line_idx].speaker == current_speaker;

                !prev_line_same
            };

            if need_new {
                script.push(Line {
                    speaker: current_speaker,
                    lines: vec![line_text]
                });
            } else {
                let prev_line_idx = script.len() - 1;
                script[prev_line_idx].lines.push(line_text);
            }
        }
    }

    // speakers
    let speakers = transcript
        .split('\n')
        .filter(|line| line.starts_with(|c: char| c.is_ascii_alphabetic()))
        .map(|line| line.split(':').next().expect("no name of speaker"))
        .collect::<HashSet<&str>>();
    let speakers = speakers.iter().collect::<Vec<_>>();

    for (i, speaker) in speakers.iter().enumerate() {
        println!("{}: {}", i + 1, speaker);
    }

    // choice
    print!("choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: usize = choice.trim().parse().unwrap();

    assert!(choice <= speakers.len(), "invalid index");

    let desired_speaker = speakers[choice - 1];

    let speakers_lines = script
        .iter()
        .enumerate()
        .filter(|(_, line)| line.speaker == *desired_speaker)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let line_idx = &speakers_lines.choose(&mut rand::thread_rng());
    let line_idx = line_idx.expect("didn't find a random line");

    let skip_back = usize::min(*line_idx, PREV_CTX);
    for line in &script[line_idx - skip_back..*line_idx] {
        print_line(line);
    }

    {
        let mut bleh = String::new();
        io::stdin().read_line(&mut bleh).unwrap();
    }

    let actual_line = &script[*line_idx];
    print_line(actual_line);
}
