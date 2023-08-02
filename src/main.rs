use clap::{Parser, ValueEnum};
use sat_slover::Solution;
use std::{fs, path::PathBuf, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SimpleSolution {
    Satisfiable,
    Unsatisfiable,
}

impl From<&Solution> for SimpleSolution {
    fn from(ok: &Solution) -> Self {
        match ok {
            Solution::Satisfiable(_) => Self::Satisfiable,
            Solution::Unsatisfiable => Self::Unsatisfiable,
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    dir: PathBuf,

    #[arg(value_enum)]
    expect: SimpleSolution,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let dirs = fs::read_dir(args.dir)?.collect::<Result<Vec<_>, _>>()?;
    let problems = dirs
        .iter()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .map(|p| sat_slover::parse(&p))
        .collect::<Result<Vec<_>, _>>()?;

    let start = Instant::now();
    let count = problems.len();
    let mut last = 0.0;
    let mut total = 0.0;
    for problem in problems {
        let solution = sat_slover::solve(problem)?;
        let elapsed = start.elapsed().as_secs_f64();
        total += elapsed - last;
        last = elapsed;
        if SimpleSolution::from(&solution) != args.expect {
            return Err(anyhow::anyhow!("Wrong answer"));
        }
    }

    println!("Average: {:.6}", total / count as f64);

    Ok(())
}
