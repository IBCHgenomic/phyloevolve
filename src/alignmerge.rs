use crate::astruct::Alignment;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*

Author Gaurav Sablok
SLB Potsdam
Date 2025-2-3

splicing the alignment and then merging the merging. It will merge
only those sites across all the alignments which you have selected.

*/

pub fn alignmergeall(
    path: &str,
    start: &str,
    end: &str,
    mergestr: &str,
) -> Result<String, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not found");
    let pathread = BufReader::new(pathopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in pathread.lines() {
        let line = i.expect("file not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line)
        }
    }
    let mut allseq: Vec<Alignment> = Vec::new();
    for i in 0..header.len() {
        allseq.push(Alignment {
            head: header[i].clone(),
            seq: sequence[i].clone(),
            length: sequence[i].len(),
        });
    }

    let mut splicedal: Vec<Alignment> = Vec::new();

    let alstart: usize = start.parse::<usize>().unwrap();
    let alend: usize = end.parse::<usize>().unwrap();
    for i in allseq.iter() {
        splicedal.push(Alignment {
            head: i.head.clone(),
            seq: i.seq[alstart..alend].to_string(),
            length: i.seq.len(),
        });
    }

    let mut spliced_header: Vec<String> = Vec::new();
    let mut spliced_seq: Vec<String> = Vec::new();
    for i in splicedal.iter() {
        spliced_header.push(i.head.clone());
        spliced_seq.push(i.seq.clone());
    }
    let mergehead: String = mergestr.to_string();
    let sequencemerge: String = spliced_seq.concat().to_string();
    println!(
        "The merged header and the merged sequence for the regions specific is >{:?}\n{:?}",
        mergehead, sequencemerge
    );

    Ok("The final merged sequence has been written".to_string())
}
