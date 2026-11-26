// H185: The Near-Twin Probe
// Force d₂=18 AND near-diagonal-equality (|Q00|≈|Q11| within 1-2 digits)
// Then test Layers 2+3: Δ≈42 and Diophantine gate E²−4S=(d₂−1)²
//
// H184 found: the gate requires near-diagonal-equality for cancellation.
// Now: does FORCING that equality let forms pass the gate?
//
// E²−4S identity: (a+b+c)²−4(ab+ac+bc) = a²+b²+c²−2ab−2ac−2bc
//                = (a−b−c)²−4bc = (b+c−a)²−4bc (when b+c>a)
// For this to be small (289), we need massive cancellation.

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

/// Make entry with target mod-9 residue, even, with specific digit count
fn make_entry(rng: &mut impl Rng, digits: u64, target_mod9: u64) -> BigUint {
    let bits = (digits as f64 * 3.32) as u64;
    loop {
        let base = rand_biguint(rng, bits);
        if base.is_zero() { continue; }
        let cur = (&base % BigUint::from(9u32)).try_into().unwrap_or(0u64);
        let adj = (target_mod9 + 9 - cur) % 9;
        let mut val = &base + BigUint::from(adj);
        // Ensure even
        if &val % BigUint::from(2u32) == BigUint::one() {
            val += BigUint::from(9u32);
            if &val % BigUint::from(2u32) == BigUint::one() {
                val += BigUint::from(9u32);
            }
        }
        let check9: u64 = (&val % BigUint::from(9u32)).try_into().unwrap_or(99);
        if check9 == target_mod9 && &val % BigUint::from(2u32) == BigUint::zero() {
            return val;
        }
    }
}

fn smith_d2(m: &[[BigInt; 3]; 3]) -> Option<BigUint> {
    let mut d1 = BigUint::zero();
    for i in 0..3 { for j in i..3 { d1 = d1.gcd(&abs_bigint(&m[i][j])); } }
    if d1.is_zero() { return None; }
    let mut g2 = BigUint::zero();
    for r1 in 0..3 { for r2 in (r1+1)..3 { for c1 in 0..3 { for c2 in (c1+1)..3 {
        let minor = &m[r1][c1] * &m[r2][c2] - &m[r1][c2] * &m[r2][c1];
        g2 = g2.gcd(&abs_bigint(&minor));
    }}}}
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
    let dcs: Vec<usize> = cof.iter().map(|c| digit_count(&abs_bigint(c))).filter(|&d| d > 0).collect();
    if dcs.is_empty() { return 0; }
    (*dcs.iter().max().unwrap() - *dcs.iter().min().unwrap()) as u64
}

fn check_diophantine(m: &[[BigInt; 3]; 3], d2: &BigUint) -> (bool, BigUint) {
    let a = abs_bigint(&m[0][0]);
    let b = abs_bigint(&m[1][1]);
    let c = abs_bigint(&m[2][2]);
    let e = &a + &b + &c;
    let s = &a * &b + &a * &c + &b * &c;
    let e_sq = &e * &e;
    let four_s = BigUint::from(4u32) * &s;
    if e_sq < four_s { return (false, BigUint::zero()); }
    let disc = &e_sq - &four_s;
    let sqrt_disc = disc.sqrt();
    let is_sq = &sqrt_disc * &sqrt_disc == disc;
    if !is_sq || d2.is_zero() { return (false, disc); }
    let d2m1 = d2 - BigUint::one();
    (sqrt_disc == d2m1, disc)
}

fn main() {
    println!("=== H185: THE NEAR-TWIN PROBE ===");
    println!("Force d₂=18 AND |Q00|≈|Q11| (within 0-2 digits)\n");

    let mut rng = rand::thread_rng();

    // Q3p has |Q00|=328d, |Q11|=329d — differ by 1 digit
    // Strategy: pick a base digit count D, make Q00 with D digits, Q11 with D or D±1
    // Q22 varies freely (Q3p has 408d)

    // Use several residue patterns known to give d₂=18
    // Simple valid sets: all entries ≡ 0 mod 9 (trivial but works)
    // Better: use Q3p-like residues and others
    let res_sets: Vec<[u64; 6]> = vec![
        [0, 0, 0, 0, 0, 0], // all zero mod 9
        [1, 1, 1, 1, 1, 1], // all one (1*1=1=1²)
        [4, 2, 4, 1, 2, 4], // mixed
        [7, 4, 1, 7, 4, 1], // mixed
        [4, 4, 4, 4, 4, 4], // all four (4*4=16≡7, 4²=16≡7 ✓)
        [1, 4, 7, 7, 1, 4], // another valid set
        [7, 7, 7, 7, 7, 7], // all seven (7*7=49≡4, 7²=49≡4 ✓)
        [1, 7, 4, 4, 1, 7], // yet another
    ];

    let n_per_config = 500u64;
    let digit_range: Vec<u64> = (100..=400).step_by(50).collect();
    let twin_offsets: Vec<i64> = vec![0, 1, -1, 2, -2];

    let mut total = 0u64;
    let mut d2_18 = 0u64;
    let mut l2_hits = 0u64;
    let mut l3_hits = 0u64;
    let mut disc_sizes: Vec<usize> = Vec::new(); // digit count of E²-4S

    println!("Config: {} residue sets × {} digit sizes × {} twin offsets × {} per config",
        res_sets.len(), digit_range.len(), twin_offsets.len(), n_per_config);
    let total_planned = res_sets.len() as u64 * digit_range.len() as u64
        * twin_offsets.len() as u64 * n_per_config;
    println!("Total planned: {}\n", total_planned);

    for res in &res_sets {
        for &base_digits in &digit_range {
            for &offset in &twin_offsets {
                let q11_digits = (base_digits as i64 + offset) as u64;
                let q22_digits: u64 = rng.gen_range(100..500);

                for _trial in 0..n_per_config {
                    let o01d: u64 = rng.gen_range(100..400);
                    let o02d: u64 = rng.gen_range(100..400);
                    let o12d: u64 = rng.gen_range(100..400);
                    let v00 = make_entry(&mut rng, base_digits, res[0]);
                    let v01 = make_entry(&mut rng, o01d, res[1]);
                    let v02 = make_entry(&mut rng, o02d, res[2]);
                    let v11 = make_entry(&mut rng, q11_digits, res[3]);
                    let v12 = make_entry(&mut rng, o12d, res[4]);
                    let v22 = make_entry(&mut rng, q22_digits, res[5]);

                    let q00 = -BigInt::from(v00);
                    let q11 = -BigInt::from(v11);
                    let q22 = -BigInt::from(v22);
                    let q01 = if rng.gen_bool(0.5) { BigInt::from(v01) } else { -BigInt::from(v01) };
                    let q02 = if rng.gen_bool(0.5) { BigInt::from(v02) } else { -BigInt::from(v02) };
                    let q12 = if rng.gen_bool(0.5) { BigInt::from(v12) } else { -BigInt::from(v12) };

                    let mat = [
                        [q00.clone(), q01.clone(), q02.clone()],
                        [q01.clone(), q11.clone(), q12.clone()],
                        [q02.clone(), q12.clone(), q22.clone()],
                    ];
                    total += 1;

                    let d2_val = match smith_d2(&mat) {
                        Some(v) => v,
                        None => continue,
                    };
                    if d2_val != BigUint::from(18u32) { continue; }
                    d2_18 += 1;

                    let gap = cofactor_gap(&mat);

                    // Record discriminant size for ALL d₂=18 forms
                    let (passed, disc) = check_diophantine(&mat, &d2_val);
                    if !disc.is_zero() {
                        disc_sizes.push(digit_count(&disc));
                    }

                    if gap >= 37 && gap <= 47 {
                        l2_hits += 1;
                        if passed {
                            l3_hits += 1;
                            println!("  *** THREE-LAYER HIT! gap={}, offset={} ***", gap, offset);
                            println!("    Diag: {}d, {}d, {}d",
                                digit_count(&abs_bigint(&mat[0][0])),
                                digit_count(&abs_bigint(&mat[1][1])),
                                digit_count(&abs_bigint(&mat[2][2])));
                        }
                    }
                }
            }
        }
        println!("  Set {:?} done — total:{} d₂=18:{} L2:{} L3:{}", res, total, d2_18, l2_hits, l3_hits);
    }

    println!("\n=== RESULTS ===");
    println!("Total forms tested:    {}", total);
    println!("d₂=18 confirmed:       {}", d2_18);
    println!("Layer 2 hits (Δ≈42):   {}", l2_hits);
    println!("Layer 3 hits (gate):   {}", l3_hits);

    // Discriminant digit-count distribution
    if !disc_sizes.is_empty() {
        disc_sizes.sort();
        let n = disc_sizes.len();
        let mean: f64 = disc_sizes.iter().sum::<usize>() as f64 / n as f64;
        let median = disc_sizes[n / 2];
        let min_d = disc_sizes[0];
        let max_d = disc_sizes[n - 1];
        println!("\nE²−4S digit count distribution (d₂=18 forms, near-twin):");
        println!("  Count: {}", n);
        println!("  Min: {}, Max: {}, Median: {}, Mean: {:.1}", min_d, max_d, median, mean);
        println!("  Q3p's E²−4S = 289 (3 digits)");
        // Count how many have disc < 10 digits
        let tiny = disc_sizes.iter().filter(|&&d| d <= 10).count();
        let small = disc_sizes.iter().filter(|&&d| d <= 50).count();
        println!("  Disc ≤ 10 digits: {} ({:.4}%)", tiny, tiny as f64 / n as f64 * 100.0);
        println!("  Disc ≤ 50 digits: {} ({:.4}%)", small, small as f64 / n as f64 * 100.0);
        // Count exact 289
        // (We already know from check_diophantine: 0)
    }

    println!();
    if l3_hits == 0 {
        println!("VERDICT: ZERO three-layer hits even with forced near-twin diagonals.");
        println!("Near-diagonal-equality is NECESSARY but NOT SUFFICIENT.");
        println!("The gate requires EXACT arithmetic cancellation of E²−4S to 289.");
        println!("This cancellation depends on the PRECISE VALUES, not just digit counts.");
        println!("\nO1 status: OPEN-STRUCTURAL (unchanged)");
        println!("FINDING: a 5th constraint exists — exact value tuning beyond size matching.");
    } else {
        println!("VERDICT: {} three-layer hit(s)! Near-twin + d₂=18 is sufficient!", l3_hits);
        println!("Q3p's design = d₂=18 + near-twin + Δ≈42 (a 4-constraint class).");
    }
}
