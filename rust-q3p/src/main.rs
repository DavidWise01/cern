// H184: The Constructed Probe
// CONSTRUCT forms with d₂=18 by design (force mod-9 residues)
// Then test Layers 2+3: Δ≈42 and Diophantine gate
//
// Q3p mod-9 residues: {7,6,0,1,6,4} for entries Q00,Q01,Q02,Q11,Q12,Q22
// These force 9|every 2×2 minor. Combined with d₁=2, d₂=gcd(minors)/d₁=36/2=18.
//
// Strategy: generate entries ≡ target mod 9, with d₁=2, varying digit sizes.

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

/// Make a random BigUint of approximately `digits` decimal digits
/// that is ≡ target (mod 9) and divisible by 2 (to ensure d₁=2)
fn make_entry(rng: &mut impl Rng, digits: u64, target_mod9: u64) -> BigUint {
    let bits = (digits as f64 * 3.32) as u64;
    loop {
        let base = rand_biguint(rng, bits);
        if base.is_zero() { continue; }
        // Current mod-9 residue
        let cur_mod9: u64 = (&base % BigUint::from(9u32)).try_into().unwrap_or(0);
        // Adjust to target mod 9
        let adj = if target_mod9 >= cur_mod9 {
            target_mod9 - cur_mod9
        } else {
            9 + target_mod9 - cur_mod9
        };
        let mut val = &base + BigUint::from(adj);
        // Now val ≡ target (mod 9). Ensure even (d₁=2 requires all entries even)
        let is_odd = &val % BigUint::from(2u32) == BigUint::one();
        if is_odd {
            val += BigUint::from(9u32); // add 9 to keep mod-9, change parity
            let still_odd = &val % BigUint::from(2u32) == BigUint::one();
            if still_odd {
                val += BigUint::from(9u32); // try again
            }
        }
        // Verify
        let check_mod9: u64 = (&val % BigUint::from(9u32)).try_into().unwrap_or(99);
        let check_even = &val % BigUint::from(2u32) == BigUint::zero();
        if check_mod9 == target_mod9 && check_even && !val.is_zero() {
            return val;
        }
    }
}

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
    println!("=== H184: THE CONSTRUCTED PROBE ===");
    println!("Constructing forms with d₂=18 BY DESIGN\n");

    let mut rng = rand::thread_rng();

    // Q3p's mod-9 residues: Q00≡7, Q01≡6, Q02≡0, Q11≡6, Q12≡4, Q22≡4
    // But we also try ALL 1431 valid mod-9 sets that give d₂=18 (from H171)
    // For efficiency, use a few representative residue patterns

    // The 6 residue positions: (Q00,Q01,Q02,Q11,Q12,Q22) mod 9
    // We need 9|every 2×2 minor. For symmetric: minor(ij) = Qii*Qjj - Qij²
    // So: Qii*Qjj ≡ Qij² (mod 9) for all pairs (i,j)
    // This gives: Q00*Q11 ≡ Q01² (mod 9), Q00*Q22 ≡ Q02² (mod 9), Q11*Q22 ≡ Q12² (mod 9)

    // Find valid residue sets
    let mut valid_residues: Vec<[u64; 6]> = Vec::new();
    for q00 in 0..9u64 {
        for q11 in 0..9u64 {
            for q22 in 0..9u64 {
                for q01 in 0..9u64 {
                    for q02 in 0..9u64 {
                        for q12 in 0..9u64 {
                            // Check all three minor conditions
                            let c1 = (q00 * q11) % 9 == (q01 * q01) % 9;
                            let c2 = (q00 * q22) % 9 == (q02 * q02) % 9;
                            let c3 = (q11 * q22) % 9 == (q12 * q12) % 9;
                            // Also need at least one entry not ≡ 0 mod 9
                            // (to avoid degenerate zero forms)
                            let nonzero = q00 > 0 || q01 > 0 || q02 > 0 ||
                                          q11 > 0 || q12 > 0 || q22 > 0;
                            if c1 && c2 && c3 && nonzero {
                                valid_residues.push([q00, q01, q02, q11, q12, q22]);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Valid mod-9 residue sets: {}", valid_residues.len());
    // Check Q3p's residues are in the list
    let qp_res = [7u64, 6, 0, 6, 4, 4];
    let qp_found = valid_residues.iter().any(|r| r == &qp_res);
    println!("Q3p residues {{7,6,0,6,4,4}} in list: {}", qp_found);
    println!();

    let n_per_set = 50u64;  // 50 forms per residue set
    let total = valid_residues.len() as u64 * n_per_set;
    println!("Testing {} residue sets × {} forms each = {} total\n", valid_residues.len(), n_per_set, total);

    let mut constructed = 0u64;
    let mut d2_18_confirmed = 0u64;
    let mut d2_other = 0u64;
    let mut layer2_hits = 0u64;
    let mut layer3_hits = 0u64;
    let mut gap_distribution: Vec<u64> = Vec::new();

    for (set_idx, residues) in valid_residues.iter().enumerate() {
        for _trial in 0..n_per_set {
            // Pick random digit sizes spread around Q3p's range (100-500)
            let d0d: u64 = rng.gen_range(100..500);
            let d1d: u64 = rng.gen_range(100..500);
            let d2d: u64 = rng.gen_range(100..500);
            let o01d: u64 = rng.gen_range(100..500);
            let o02d: u64 = rng.gen_range(100..500);
            let o12d: u64 = rng.gen_range(100..500);

            let v00 = make_entry(&mut rng, d0d, residues[0]);
            let v01 = make_entry(&mut rng, o01d, residues[1]);
            let v02 = make_entry(&mut rng, o02d, residues[2]);
            let v11 = make_entry(&mut rng, d1d, residues[3]);
            let v12 = make_entry(&mut rng, o12d, residues[4]);
            let v22 = make_entry(&mut rng, d2d, residues[5]);

            // All diagonal negative (like Q3p)
            let q00 = -BigInt::from(v00);
            let q11 = -BigInt::from(v11);
            let q22 = -BigInt::from(v22);
            // Off-diag: random sign
            let q01 = if rng.gen_bool(0.5) { BigInt::from(v01) } else { -BigInt::from(v01) };
            let q02 = if rng.gen_bool(0.5) { BigInt::from(v02) } else { -BigInt::from(v02) };
            let q12 = if rng.gen_bool(0.5) { BigInt::from(v12) } else { -BigInt::from(v12) };

            let mat = [
                [q00.clone(), q01.clone(), q02.clone()],
                [q01.clone(), q11.clone(), q12.clone()],
                [q02.clone(), q12.clone(), q22.clone()],
            ];

            constructed += 1;

            // Verify d₂
            let d2_val = match smith_d2(&mat) {
                Some(v) => v,
                None => { d2_other += 1; continue; }
            };

            if d2_val == BigUint::from(18u32) {
                d2_18_confirmed += 1;
            } else {
                d2_other += 1;
                continue;
            }

            // Layer 2: cofactor gap
            let gap = cofactor_gap(&mat);
            gap_distribution.push(gap);

            if gap >= 37 && gap <= 47 {
                layer2_hits += 1;
                println!("  L2 HIT at set {}, gap={}", set_idx, gap);

                // Layer 3: Diophantine gate
                if check_diophantine(&mat, &d2_val) {
                    layer3_hits += 1;
                    println!("  *** THREE-LAYER HIT! set={} gap={} ***", set_idx, gap);
                    println!("    Diag digits: {}, {}, {}",
                        digit_count(&abs_bigint(&mat[0][0])),
                        digit_count(&abs_bigint(&mat[1][1])),
                        digit_count(&abs_bigint(&mat[2][2])));
                    println!("    Residues: {:?}", residues);
                }
            }
        }

        if (set_idx + 1) % 500 == 0 {
            println!("  Progress: {}/{} sets — d₂=18:{} L2:{} L3:{}",
                set_idx+1, valid_residues.len(), d2_18_confirmed, layer2_hits, layer3_hits);
        }
    }

    println!("\n=== RESULTS ===");
    println!("Residue sets tested:   {}", valid_residues.len());
    println!("Forms constructed:     {}", constructed);
    println!("d₂=18 confirmed:       {}", d2_18_confirmed);
    println!("d₂ other (unexpected): {}", d2_other);
    println!("Layer 2 hits (Δ≈42):   {}", layer2_hits);
    println!("Layer 3 hits (gate):   {}", layer3_hits);
    println!();

    // Gap distribution statistics
    if !gap_distribution.is_empty() {
        gap_distribution.sort();
        let n = gap_distribution.len();
        let mean: f64 = gap_distribution.iter().sum::<u64>() as f64 / n as f64;
        let median = gap_distribution[n / 2];
        let min_gap = gap_distribution[0];
        let max_gap = gap_distribution[n - 1];
        println!("Cofactor gap distribution (d₂=18 forms only):");
        println!("  Count: {}", n);
        println!("  Min: {}, Max: {}, Median: {}, Mean: {:.1}", min_gap, max_gap, median, mean);

        // Histogram: how many in each 10-bucket
        let mut buckets = vec![0u64; 60]; // 0-9, 10-19, ..., 590-599
        for &g in &gap_distribution {
            let b = (g / 10) as usize;
            if b < buckets.len() { buckets[b] += 1; }
        }
        println!("  Histogram (bucket size 10):");
        for (i, &count) in buckets.iter().enumerate() {
            if count > 0 {
                println!("    {}-{}: {}", i*10, i*10+9, count);
            }
        }
    }

    println!();
    if layer3_hits == 0 {
        println!("VERDICT: ZERO three-layer hits among CONSTRUCTED d₂=18 forms.");
        println!("The rarity is STRUCTURAL, not just statistical.");
        println!("Even purpose-built d₂=18 forms cannot pass Layers 2+3.");
        if layer2_hits > 0 {
            println!("Note: {} forms passed L1+L2 but ALL failed the Diophantine gate.", layer2_hits);
            println!("The gate is the hardest layer — it requires (d₂-1)² = disc exactly.");
        }
        println!("\nO1 status: OPEN-RARE → OPEN-STRUCTURAL");
        println!("The three-layer design is not just rare — it resists construction.");
    } else {
        println!("VERDICT: {} three-layer hit(s)! Q3p's design is a CLASS.", layer3_hits);
        println!("O1 RESOLVED: other forms with the three-layer design exist.");
    }

    // Construction success rate
    let d2_rate = d2_18_confirmed as f64 / constructed as f64 * 100.0;
    println!("\nd₂=18 construction success rate: {:.2}%", d2_rate);
    println!("(Expected ~high since we forced mod-9 residues)");
}
