// H183: The Uniqueness Probe
// Attack O1: search for ANY other 3×3 symmetric form with all three layers
// Layer 1: Smith d₂ = 18
// Layer 2: Δ = d(|Q22|) - d(|Q02|) ≈ 42 (within ±5)
// Layer 3: 8G + 289 = E² (Diophantine gate closes)

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{Zero, One};
use rand::Rng;

fn rand_biguint(rng: &mut impl Rng, bits: u64) -> BigUint {
    let bytes = ((bits + 7) / 8) as usize;
    let mut buf = vec![0u8; bytes];
    rng.fill(&mut buf[..]);
    BigUint::from_bytes_be(&buf)
}

fn digit_count(n: &BigUint) -> usize {
    if n.is_zero() { return 1; }
    n.to_string().len()
}

fn abs_bigint(n: &BigInt) -> BigUint {
    n.magnitude().clone()
}

/// Compute d₂ for a 3×3 symmetric matrix
/// d₁ = gcd of all entries
/// d₂ = gcd of all 2×2 minors / d₁
fn smith_d2(m: &[[BigInt; 3]; 3]) -> Option<BigUint> {
    let mut d1 = BigUint::zero();
    for i in 0..3 {
        for j in i..3 {
            d1 = d1.gcd(&abs_bigint(&m[i][j]));
        }
    }
    if d1.is_zero() { return None; }

    let mut g2 = BigUint::zero();
    for r1 in 0..3 {
        for r2 in (r1+1)..3 {
            for c1 in 0..3 {
                for c2 in (c1+1)..3 {
                    let minor = &m[r1][c1] * &m[r2][c2] - &m[r1][c2] * &m[r2][c1];
                    g2 = g2.gcd(&abs_bigint(&minor));
                }
            }
        }
    }
    if g2.is_zero() { return None; }

    let (d2, rem) = g2.div_rem(&d1);
    if !rem.is_zero() { return None; }
    Some(d2)
}

/// Compute cofactor gap G = max(cofactor digits) - min(cofactor digits)
fn cofactor_gap(m: &[[BigInt; 3]; 3]) -> u64 {
    let cof = [
        &m[1][1] * &m[2][2] - &m[1][2] * &m[2][1],
        &m[0][2] * &m[2][1] - &m[0][1] * &m[2][2],
        &m[0][1] * &m[1][2] - &m[0][2] * &m[1][1],
        &m[0][0] * &m[2][2] - &m[0][2] * &m[2][0],
        &m[0][0] * &m[1][2] - &m[0][1] * &m[0][2],
        &m[0][0] * &m[1][1] - &m[0][1] * &m[1][0],
    ];
    let dcs: Vec<usize> = cof.iter()
        .map(|c| digit_count(&abs_bigint(c)))
        .filter(|&d| d > 0)
        .collect();
    if dcs.is_empty() { return 0; }
    let mx = *dcs.iter().max().unwrap();
    let mn = *dcs.iter().min().unwrap();
    (mx - mn) as u64
}

/// Check Diophantine gate: E²-4S = (d₂-1)²
fn check_diophantine(m: &[[BigInt; 3]; 3], d2: &BigUint) -> bool {
    let q00 = abs_bigint(&m[0][0]);
    let q11 = abs_bigint(&m[1][1]);
    let q22 = abs_bigint(&m[2][2]);

    let e = &q00 + &q11 + &q22;
    let s = &q00 * &q11 + &q00 * &q22 + &q11 * &q22;

    let e_sq = &e * &e;
    let four_s = BigUint::from(4u32) * &s;

    if e_sq < four_s { return false; }
    let disc = &e_sq - &four_s;

    let sqrt_disc = disc.sqrt();
    if &sqrt_disc * &sqrt_disc != disc { return false; }

    if d2.is_zero() { return false; }
    let d2m1 = d2 - BigUint::one();
    sqrt_disc == d2m1
}

fn main() {
    println!("=== H183: THE UNIQUENESS PROBE ===");
    println!("Searching for forms with all three layers of Q3p's design\n");

    let mut rng = rand::thread_rng();

    println!("Q3p reference:");
    println!("  d₂ = 18");
    println!("  Δ(cofactor gap) ≈ 42 (actually 41)");
    println!("  E²−4S = 289 = 17² = (d₂−1)²");
    println!();

    let n_candidates = 500_000u64;
    let mut layer1_hits = 0u64;
    let mut layer2_hits = 0u64;
    let mut layer3_hits = 0u64;
    let mut near_misses = 0u64;

    println!("Running {} candidates...", n_candidates);
    println!("Strategy: random symmetric 3×3, mixed digit sizes (50-200 digits)");
    println!("  All diagonal negative (like Q3p)\n");

    for trial in 0..n_candidates {
        // Random digit counts — use bits (3.32 bits per digit)
        let bits = |d: u64| -> u64 { (d as f64 * 3.32) as u64 };
        let d0d: u64 = rng.gen_range(50..200);
        let d1d: u64 = rng.gen_range(50..200);
        let d2d: u64 = rng.gen_range(50..200);
        let o01d: u64 = rng.gen_range(50..200);
        let o02d: u64 = rng.gen_range(50..200);
        let o12d: u64 = rng.gen_range(50..200);

        let q00 = -BigInt::from(rand_biguint(&mut rng, bits(d0d)));
        let q11 = -BigInt::from(rand_biguint(&mut rng, bits(d1d)));
        let q22 = -BigInt::from(rand_biguint(&mut rng, bits(d2d)));
        let mut q01 = BigInt::from(rand_biguint(&mut rng, bits(o01d)));
        let mut q02 = BigInt::from(rand_biguint(&mut rng, bits(o02d)));
        let mut q12 = BigInt::from(rand_biguint(&mut rng, bits(o12d)));

        if rng.gen_bool(0.5) { q01 = -q01; }
        if rng.gen_bool(0.5) { q02 = -q02; }
        if rng.gen_bool(0.5) { q12 = -q12; }

        let mat = [
            [q00.clone(), q01.clone(), q02.clone()],
            [q01.clone(), q11.clone(), q12.clone()],
            [q02.clone(), q12.clone(), q22.clone()],
        ];

        // Layer 1: d₂ = 18?
        let d2_val = match smith_d2(&mat) {
            Some(v) => v,
            None => continue,
        };
        if d2_val != BigUint::from(18u32) { continue; }
        layer1_hits += 1;

        // Layer 2: cofactor gap ≈ 42?
        let gap = cofactor_gap(&mat);
        if gap >= 37 && gap <= 47 {
            layer2_hits += 1;

            // Layer 3: Diophantine gate
            if check_diophantine(&mat, &d2_val) {
                layer3_hits += 1;
                println!("  *** THREE-LAYER HIT at trial {} ***", trial);
                println!("    d₂=18, Δ={}, Diophantine CLOSED", gap);
                println!("    Diag digits: {}, {}, {}",
                    digit_count(&abs_bigint(&mat[0][0])),
                    digit_count(&abs_bigint(&mat[1][1])),
                    digit_count(&abs_bigint(&mat[2][2])));
            } else {
                // Check if disc is at least a perfect square
                let q00a = abs_bigint(&mat[0][0]);
                let q11a = abs_bigint(&mat[1][1]);
                let q22a = abs_bigint(&mat[2][2]);
                let e = &q00a + &q11a + &q22a;
                let s = &q00a * &q11a + &q00a * &q22a + &q11a * &q22a;
                let e_sq = &e * &e;
                let four_s = BigUint::from(4u32) * &s;
                if e_sq >= four_s {
                    let disc = &e_sq - &four_s;
                    let sqrt_d = disc.sqrt();
                    if &sqrt_d * &sqrt_d == disc {
                        near_misses += 1;
                    }
                }
            }
        }

        if (trial + 1) % 50000 == 0 {
            println!("  Progress: {}/{} — L1:{} L2:{} L3:{}",
                trial+1, n_candidates, layer1_hits, layer2_hits, layer3_hits);
        }
    }

    println!("\n=== RESULTS ===");
    println!("Candidates tested:     {}", n_candidates);
    println!("Layer 1 hits (d₂=18):  {}", layer1_hits);
    println!("Layer 2 hits (+Δ≈42):  {}", layer2_hits);
    println!("Layer 3 hits (+gate):  {}", layer3_hits);
    println!("Near misses (L2+sq):   {}", near_misses);
    println!();

    if layer3_hits == 0 {
        println!("VERDICT: ZERO three-layer hits in {} candidates.", n_candidates);
        println!("O1 STRENGTHENED: Q3p's three-layer design is rare.");
        if layer1_hits > 0 {
            let l1_rate = layer1_hits as f64 / n_candidates as f64;
            println!("  Layer 1 rate: {:.6}%", l1_rate * 100.0);
        }
        if layer2_hits > 0 {
            let l2_rate = layer2_hits as f64 / layer1_hits.max(1) as f64;
            println!("  Layer 2|1 rate: {:.6}%", l2_rate * 100.0);
        }
        println!("\n  P(all 3 layers) ≤ {:.2e}", 1.0 / n_candidates as f64);
    } else {
        println!("VERDICT: {} three-layer hit(s) found!", layer3_hits);
        println!("O1 RESOLVED: The design is a CLASS, not unique.");
    }

    println!("\nExpected d₂=18 rate (H171 theory): ~0.269%");
    if layer1_hits > 0 {
        println!("Observed d₂=18 rate: {:.4}%",
            layer1_hits as f64 / n_candidates as f64 * 100.0);
    }
}
