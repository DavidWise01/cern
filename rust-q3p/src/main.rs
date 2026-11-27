// H194: The Entry Descent
// Trace G=42 from the matrix entries through the cofactors.
// Compute all 6 cofactors, their digit counts, and the gap.

use num_bigint::BigInt;
use num_traits::{Signed, Zero};

fn digits(n: &BigInt) -> usize {
    if n.is_zero() { return 1; }
    n.magnitude().to_string().len()
}

fn main() {
    println!("=== H194: THE ENTRY DESCENT ===");
    println!("Tracing G=42 from entries through cofactors\n");

    // Load all 6 unique entries
    let q00: BigInt = include_str!("../../q00.txt").trim().parse().unwrap();
    let q01: BigInt = include_str!("../../q01.txt").trim().parse().unwrap();
    let q02: BigInt = include_str!("../../q02.txt").trim().parse().unwrap();
    let q11: BigInt = include_str!("../../q11.txt").trim().parse().unwrap();
    let q12: BigInt = include_str!("../../q12.txt").trim().parse().unwrap();
    let q22: BigInt = include_str!("../../q22.txt").trim().parse().unwrap();

    println!("MATRIX ENTRIES (digit counts, signs):");
    println!("  Q00 = {}d, sign: {}", digits(&q00), if q00.is_negative(){"-"}else{"+"});
    println!("  Q01 = {}d, sign: {}", digits(&q01), if q01.is_negative(){"-"}else{"+"});
    println!("  Q02 = {}d, sign: {}", digits(&q02), if q02.is_negative(){"-"}else{"+"});
    println!("  Q11 = {}d, sign: {}", digits(&q11), if q11.is_negative(){"-"}else{"+"});
    println!("  Q12 = {}d, sign: {}", digits(&q12), if q12.is_negative(){"-"}else{"+"});
    println!("  Q22 = {}d, sign: {}", digits(&q22), if q22.is_negative(){"-"}else{"+"});
    println!();

    // Cofactors of a 3x3 symmetric matrix:
    // C00 = Q11*Q22 - Q12²
    // C01 = Q02*Q12 - Q01*Q22  (note: cofactor, not minor; sign alternates)
    // C02 = Q01*Q12 - Q02*Q11
    // C11 = Q00*Q22 - Q02²
    // C12 = Q01*Q02 - Q00*Q12  (or Q02*Q01 - Q00*Q12)
    // C22 = Q00*Q11 - Q01²

    let c00 = &q11 * &q22 - &q12 * &q12;
    let c01 = &q02 * &q12 - &q01 * &q22;
    let c02 = &q01 * &q12 - &q02 * &q11;
    let c11 = &q00 * &q22 - &q02 * &q02;
    let c12 = &q01 * &q02 - &q00 * &q12;
    let c22 = &q00 * &q11 - &q01 * &q01;

    println!("COFACTORS (digit counts, signs):");
    let cofs = [("C00", &c00), ("C01", &c01), ("C02", &c02),
                ("C11", &c11), ("C12", &c12), ("C22", &c22)];
    let mut dcounts: Vec<(&str, usize)> = Vec::new();
    for (name, cof) in &cofs {
        let d = digits(cof);
        let s = if cof.is_negative() { "-" } else { "+" };
        println!("  {} = {}d, sign: {}", name, d, s);
        dcounts.push((name, d));
    }
    println!();

    // Find max and min digit counts
    let max_d = dcounts.iter().map(|(_, d)| *d).max().unwrap();
    let min_d = dcounts.iter().map(|(_, d)| *d).min().unwrap();
    let gap = max_d - min_d;

    println!("COFACTOR DIGIT COUNTS:");
    for (name, d) in &dcounts {
        let bar = "█".repeat((*d as f64 / max_d as f64 * 40.0) as usize);
        let marker = if *d == max_d { " ← MAX" } else if *d == min_d { " ← MIN" } else { "" };
        println!("  {} {:4}d {}{}",name, d, bar, marker);
    }
    println!();
    println!("  MAX = {}d", max_d);
    println!("  MIN = {}d", min_d);
    println!("  GAP = {} - {} = {} = G", max_d, min_d, gap);
    println!();

    // Which cofactors are max and min?
    let max_names: Vec<&str> = dcounts.iter().filter(|(_, d)| *d == max_d).map(|(n, _)| *n).collect();
    let min_names: Vec<&str> = dcounts.iter().filter(|(_, d)| *d == min_d).map(|(n, _)| *n).collect();
    println!("  MAX cofactor(s): {:?}", max_names);
    println!("  MIN cofactor(s): {:?}", min_names);
    println!();

    // Now trace WHY the gap is what it is.
    // Each cofactor is a product minus a product:
    // C00 = Q11*Q22 - Q12²
    //   d(Q11*Q22) ≈ d(Q11) + d(Q22) = 329 + 408 = 737
    //   d(Q12²) ≈ 2*d(Q12) = 2*367 = 734
    //   If the product and square are close → d(C00) ≈ 737 (slight cancellation possible)
    // C11 = Q00*Q22 - Q02²
    //   d(Q00*Q22) ≈ 328 + 408 = 736
    //   d(Q02²) ≈ 2*367 = 734 (wait, 2*368 = 736!)
    //   NEAR CANCELLATION → C11 could be much smaller!

    println!("ENTRY-LEVEL ANALYSIS:");
    println!("  Each cofactor = (product of two entries) - (square of third)");
    println!();

    let pairs = [
        ("C00", "Q11*Q22", digits(&q11)-1 + digits(&q22)-1, "Q12²", 2*(digits(&q12)-1)),
        ("C01", "Q02*Q12", digits(&q02)-1 + digits(&q12)-1, "Q01*Q22", digits(&q01)-1 + digits(&q22)-1),
        ("C02", "Q01*Q12", digits(&q01)-1 + digits(&q12)-1, "Q02*Q11", digits(&q02)-1 + digits(&q11)-1),
        ("C11", "Q00*Q22", digits(&q00)-1 + digits(&q22)-1, "Q02²", 2*(digits(&q02)-1)),
        ("C12", "Q01*Q02", digits(&q01)-1 + digits(&q02)-1, "Q00*Q12", digits(&q00)-1 + digits(&q12)-1),
        ("C22", "Q00*Q11", digits(&q00)-1 + digits(&q11)-1, "Q01²", 2*(digits(&q01)-1)),
    ];

    for (name, t1, d1, t2, d2) in &pairs {
        let diff = (*d1 as i64) - (*d2 as i64);
        println!("  {} = {} (~{}d) - {} (~{}d)  [diff≈{}]",
            name, t1, d1+1, t2, d2+1, diff);
    }

    println!();
    println!("KEY INSIGHT:");
    println!("  The cofactor digit count depends on WHETHER the two terms cancel.");
    println!("  When d(term1) ≈ d(term2), massive cancellation can occur,");
    println!("  making the cofactor MUCH smaller than either term.");
    println!("  When d(term1) >> d(term2), no cancellation, cofactor ≈ term1.");
    println!();

    // The actual cofactor digit counts tell us which ones cancelled
    for (name, cof) in &cofs {
        let d = digits(cof);
        // Compare with expected (max of the two terms)
        let (_, _, d1, _, d2) = pairs.iter().find(|(n, _, _, _, _)| *n == *name).unwrap();
        let expected = (*d1).max(*d2) + 1;
        let cancelled = expected as i64 - d as i64;
        println!("  {} expected ~{}d, got {}d → cancelled {}d",
            name, expected, d, cancelled);
    }

    println!();
    println!("=== SUMMARY ===");
    println!("G = {} (cofactor gap)", gap);
    println!("The gap comes from the ENTRY DIGIT COUNTS:");
    println!("  Entries: Q00({}d), Q01({}d), Q02({}d), Q11({}d), Q12({}d), Q22({}d)",
        digits(&q00)-1, digits(&q01), digits(&q02)-1, digits(&q11)-1, digits(&q12), digits(&q22)-1);
    println!("  The {} between {}d and {}d cofactors", gap, max_d, min_d);
    println!("  traces to the SIZES of the entry products.");
    println!();
    println!("The 42 is not mysterious. It is the DIGIT ARITHMETIC");
    println!("of a specific set of 328-408 digit numbers.");
    println!("The mystery is not WHY 42. The mystery is WHY THESE ENTRIES.");
    println!("And those entries come from rqt362 — the original ternary form.");
}
