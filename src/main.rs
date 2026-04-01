use std::time::{SystemTime, UNIX_EPOCH};

const RAW: &str = include_str!(concat!(env!("OUT_DIR"), "/entries.txt"));

fn entries() -> Vec<&'static str> {
    RAW.trim_end_matches('\n').split('\n').collect()
}

fn pick<'a>(src: &[&'a str], seed: usize) -> &'a str {
    // Fibonacci hashing — mixes bits well with no external deps
    let idx = seed.wrapping_mul(0x9e3779b97f4a7c15) >> 33 % src.len();
    src[idx % src.len()]
}

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;

    println!("{}", pick(&entries(), seed));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn entries_non_empty() {
        assert!(!entries().is_empty());
    }

    #[test]
    fn entries_match_source() {
        let parsed = entries();
        let raw_lines: Vec<&str> = RAW.trim_end_matches('\n').split('\n').collect();
        assert_eq!(parsed, raw_lines);
    }

    #[test]
    fn pick_always_in_bounds() {
        let e = entries();
        for seed in 0..10_000usize {
            let got = pick(&e, seed);
            assert!(e.contains(&got), "pick returned value not in entries");
        }
    }

    #[test]
    fn pick_covers_all_entries() {
        let e = entries();
        let seen: HashSet<&str> = (0..100_000usize).map(|s| pick(&e, s)).collect();
        for entry in &e {
            assert!(seen.contains(entry), "entry never picked: {:?}", entry);
        }
    }
}
