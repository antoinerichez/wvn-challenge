use std::env::args;
use std::iter::empty;
use std::time::Instant;

mod challenges {
    pub mod challenge01;
    pub mod challenge02;
}

fn main() {
    let challenge_number = match args().nth(1) {
        Some(arg) => Some(arg.parse::<u8>().unwrap()),
        None => None
    };

    let to_run = empty()
    .chain(challenges())
    .filter(|sln| challenge_number.is_none_or(|n| sln.identifier.contains(&n.to_string())));

    for Challenge { identifier, solution } in to_run {
        let instant = Instant::now();
        println!("Running solution for {:}...", identifier);
        solution();
        let elapsed = instant.elapsed();

        println!("Runned in {:?}ms", elapsed.as_micros());
    }
}

struct Challenge {
    identifier: String,
    solution: fn() -> (),
}

macro_rules !solution {
    ($challengeRef: tt) => {{
        let reference: String = stringify!($challengeRef).to_string();

        let sln = || {challenges::$challengeRef::execute()};

        Challenge { identifier: reference, solution: sln }
    }};
}

fn challenges() -> Vec<Challenge> {
    vec![
        solution!(challenge01),
        solution!(challenge02),
    ]
}
