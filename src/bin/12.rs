use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/12.txt");
    let mut vertices: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let entry = vertices.entry(from).or_default();
        entry.push(to);
        let entry = vertices.entry(to).or_default();
        entry.push(from);
    }

    let mut completed_paths = Vec::new();
    let mut active_walks = VecDeque::new();
    active_walks.push_back(Walk {
        at: "start",
        path: Vec::new(),
        visited_small: HashSet::new(),
        did_visit_special_cave: false,
    });
    while let Some(mut walk) = active_walks.pop_front() {
        walk.path.push(walk.at);

        if walk.at.chars().all(|ch| ch.is_ascii_lowercase()) {
            walk.visited_small.insert(walk.at);
        }
        if walk.at == "end" {
            completed_paths.push(walk);
            continue;
        }

        if let Some(next_paths) = vertices.get(walk.at) {
            for next_at in next_paths {
                if !walk.visited_small.contains(next_at) {
                    active_walks.push_back(Walk {
                        at: next_at,
                        path: walk.path.clone(),
                        visited_small: walk.visited_small.clone(),
                        did_visit_special_cave: false,
                    });
                }
            }
        }
    }

    println!("Part 1: {}", completed_paths.len());

    let mut completed_paths = HashSet::new();
    for special_cave in vertices
        .keys()
        .filter(|k| **k != "start" && **k != "end" && k.chars().all(|ch| ch.is_ascii_lowercase()))
    {
        let mut active_walks = VecDeque::new();
        active_walks.push_back(Walk {
            at: "start",
            path: Vec::new(),
            visited_small: HashSet::new(),
            did_visit_special_cave: false,
        });
        while let Some(mut walk) = active_walks.pop_front() {
            walk.path.push(walk.at);

            if !walk.did_visit_special_cave && walk.at == *special_cave {
                walk.did_visit_special_cave = true;
            } else if walk.at.chars().all(|ch| ch.is_ascii_lowercase()) {
                walk.visited_small.insert(walk.at);
            }
            if walk.at == "end" {
                completed_paths.insert(walk.path);
                continue;
            }

            if let Some(next_paths) = vertices.get(walk.at) {
                for next_at in next_paths {
                    if !walk.visited_small.contains(next_at) {
                        active_walks.push_back(Walk {
                            at: next_at,
                            path: walk.path.clone(),
                            visited_small: walk.visited_small.clone(),
                            did_visit_special_cave: walk.did_visit_special_cave,
                        });
                    }
                }
            }
        }
    }

    println!("Part 2: {}", completed_paths.len());

    Ok(())
}

struct Walk<'a> {
    at: &'a str,
    path: Vec<&'a str>,
    visited_small: HashSet<&'a str>,
    did_visit_special_cave: bool,
}
