use dialoguer::Input;
use indicatif::ProgressBar;
use needle_searcher::find_seed;
use std::io;

fn input_vec(prompt: &str, init: &str, radix: u32) -> Vec<u64> {
    Input::<String>::new()
        .with_prompt(prompt)
        .with_initial_text(init)
        .interact_text()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| u64::from_str_radix(x, radix).unwrap())
        .collect()
}

fn main() -> io::Result<()> {
    let needles = input_vec("Needles", "5 2 14 8 7 6 4 6 11 6 3 8", 10);
    assert!(
        needles.len() == 12 && needles.iter().all(|&needle| needle <= 16),
        "Needles must be space-delimited 12 integers between 0 and 17."
    );

    let offset: u32 = Input::<String>::new()
        .with_prompt("Advance offset (417/477)")
        .with_initial_text("417")
        .interact_text()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(
        offset == 417 || offset == 477,
        "Offset must be 417 or 477 based on the game being SM or USUM."
    );

    println!();
    let now = std::time::Instant::now();

    let pb = ProgressBar::new(0);
    let notify_progress = move |hits: &[u32], len: u32| {
        pb.set_length(len as u64);
        for hit in hits {
            pb.println(format!("Hit! => Seed: {:08X}", hit));
        }
        pb.inc(1);
    };

    let hits = find_seed(
        (0x0000, 0x10000),
        needles,
        offset,
        notify_progress,
    );

    println!("Done!");
    println!("Elapsed: {:?}", now.elapsed());

    println!();
    println!("Results:");
    for hit in &hits {
        println!("- Seed: {:08X}", hit);
    }
    println!();

    let _ = Input::<String>::new()
        .with_prompt("Press Ctrl+C to quit")
        .interact();
    Ok(())
}
