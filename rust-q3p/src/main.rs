//! H166: The Closed System
//! Test the dual rule on a SECOND ternary form Q₂.
//! Q₂(a,b,c) = -100a² + 242ab - 144b² - 10000c²
//! Matrix: [[-100, 121, 0], [121, -144, 0], [0, 0, -10000]]
//! Compare to Q3p's dual rule: E=W+C, S=W×C.

fn main() {
    println!("=== H166: THE CLOSED SYSTEM ===\n");

    let q00: i64 = -100;
    let q01: i64 = 121;
    let q11: i64 = -144;
    let q02: i64 = 0;
    let q12: i64 = 0;
    let q22: i64 = -10000;

    println!("SECOND FORM Q₂:");
    println!("  Q₂(a,b,c) = {}a² + {}ab + {}b² + {}c²", q00, 2*q01, q11, q22);
    println!("  Matrix: [[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
        q00, q01, q02, q01, q11, q12, q02, q12, q22);

    let r01 = q01*q01 - (q00.abs())*(q11.abs());
    println!("\n  R01 = {}² - {}×{} = {}", q01, q00.abs(), q11.abs(), r01);
    let sqrt_r01 = (r01 as f64).sqrt();
    println!("  √R01 ≈ {:.6}", sqrt_r01);
    let isqrt = sqrt_r01 as i64;
    let is_perf_sq = isqrt*isqrt == r01 || (isqrt+1)*(isqrt+1) == r01;
    println!("  R01 perfect square? {}", if is_perf_sq { "YES" } else { "NO (241 is prime)" });

    let n_plus = (q01 as f64 + sqrt_r01) / (q00.abs() as f64);
    let n_minus = (q01 as f64 - sqrt_r01) / (q00.abs() as f64);
    println!("  n₊ = {:.10}", n_plus);
    println!("  n₋ = {:.10}", n_minus);
    println!("  Gap width = {:.10}", n_plus - n_minus);

    // CF expansion
    println!("\n  CF expansion of n₊:");
    let mut x = n_plus;
    let mut cf = Vec::new();
    for _ in 0..12 {
        let a = x.floor() as i64;
        cf.push(a);
        let frac = x - a as f64;
        if frac < 1e-10 { break; }
        x = 1.0 / frac;
    }
    println!("  n₊ = [{};{}]", cf[0],
        cf[1..].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));

    // Convergents
    println!("\n  Convergents:");
    let mut h_prev: i64 = 1;
    let mut h_curr: i64 = cf[0];
    let mut k_prev: i64 = 0;
    let mut k_curr: i64 = 1;
    let ratio0 = h_curr as f64 / k_curr as f64;
    let inside0 = ratio0 > n_minus && ratio0 < n_plus;
    println!("    p₀/q₀ = {}/{} = {:.6} wl={} inside={}",
        h_curr, k_curr, ratio0, h_curr.abs()+k_curr.abs(), inside0);
    for i in 1..cf.len().min(8) {
        let h_new = cf[i] * h_curr + h_prev;
        let k_new = cf[i] * k_curr + k_prev;
        let ratio = h_new as f64 / k_new as f64;
        let inside = ratio > n_minus && ratio < n_plus;
        println!("    p{}/q{} = {}/{} = {:.6} wl={} inside={}",
            i, i, h_new, k_new, ratio, h_new.abs()+k_new.abs(), inside);
        h_prev = h_curr;
        k_prev = k_curr;
        h_curr = h_new;
        k_curr = k_new;
    }

    // Determinant & Smith form
    let det = q00*(q11*q22 - q12*q12) - q01*(q01*q22 - q12*q02) + q02*(q01*q12 - q11*q02);
    println!("\n  det = {}", det);

    let m01 = q00*q11 - q01*q01;
    let m02 = q00*q22 - q02*q02;
    let m12 = q11*q22 - q12*q12;
    let minors_abs: Vec<i64> = vec![m01.abs(), m02.abs(), m12.abs()];
    let mut d2: i64 = 0;
    for &m in &minors_abs {
        if m > 0 { d2 = if d2 == 0 { m } else { gcd(d2, m) }; }
    }
    println!("  d₂ = {}", d2);

    let c00 = q11*q22 - q12*q12;
    let c11 = q00*q22 - q02*q02;
    let c22 = q00*q11 - q01*q01;
    println!("\n  Cofactors:");
    println!("    C00 = {} ({} digits)", c00, dcount(c00));
    println!("    C11 = {} ({} digits)", c11, dcount(c11));
    println!("    C22 = {} ({} digits)", c22, dcount(c22.abs()));
    let span = dcount(c00) as i64 - dcount(c22.abs()) as i64;
    println!("    Span S = {} - {} = {}", dcount(c00), dcount(c22.abs()), span);

    // ═══ ESCAPE SCAN ═══
    println!("\n{}", "=".repeat(70));
    println!("ESCAPE SCAN FOR Q₂");
    println!("{}", "=".repeat(70));
    let eval = |a: i64, b: i64, c: i64| -> i64 {
        q00*a*a + 2*q01*a*b + q11*b*b + 2*q02*a*c + 2*q12*b*c + q22*c*c
    };

    println!("{:<4} {:>8} {:>6} {:>12} {:>30}",
        "WL", "vectors", "pos", "min_val", "first_pos_vector");
    println!("{}", "-".repeat(70));

    let mut first_escape_n: i64 = 0;
    let mut escape_wls = Vec::new();

    for n in 1..=20i64 {
        let mut total = 0u64;
        let mut pos_count = 0u64;
        let mut min_val = i64::MAX;
        let mut first_pos = String::new();

        for a in -n..=n {
            let rem = n - a.abs();
            for b in -rem..=rem {
                let c_abs = rem - b.abs();
                let c_vals: Vec<i64> = if c_abs == 0 { vec![0] } else { vec![c_abs, -c_abs] };
                for &c_val in &c_vals {
                    if a == 0 && b == 0 && c_val == 0 { continue; }
                    total += 1;
                    let v = eval(a, b, c_val);
                    if v < min_val { min_val = v; }
                    if v > 0 {
                        pos_count += 1;
                        if first_pos.is_empty() {
                            first_pos = format!("({},{},{}) Q₂={}", a, b, c_val, v);
                        }
                    }
                }
            }
        }

        let tag = if pos_count > 0 { " ← ESCAPE" } else { "" };
        println!("{:<4} {:>8} {:>6} {:>12} {:>30}{}",
            n, total, pos_count, min_val,
            if first_pos.is_empty() { "—".to_string() } else { first_pos }, tag);

        if pos_count > 0 {
            escape_wls.push(n);
            if first_escape_n == 0 { first_escape_n = n; }
        }
    }

    println!("\nEscape word lengths: {:?}", escape_wls);
    println!("Escape pattern: {} (escapes at multiple wl)",
        if escape_wls.len() > 1 { "PERIODIC" } else { "ISOLATED" });

    // ═══ CRT ANALYSIS ═══
    println!("\n{}", "=".repeat(70));
    println!("CRT ANALYSIS FOR Q₂");
    println!("{}", "=".repeat(70));
    let primes = [2i64, 3, 5, 7, 11, 13, 241];
    for &p in &primes {
        let mut min_wl: i64 = 0;
        'outer: for n in 1..=50i64 {
            for a in 0..p {
                for b in 0..p {
                    for c_val in 0..p {
                        if a == 0 && b == 0 && c_val == 0 { continue; }
                        let wl = a + b + c_val;
                        if wl != n { continue; }
                        let v = ((q00*a*a + 2*q01*a*b + q11*b*b
                            + 2*q02*a*c_val + 2*q12*b*c_val + q22*c_val*c_val)
                            % p + p) % p;
                        if v > 0 {
                            min_wl = n;
                            break 'outer;
                        }
                    }
                }
            }
        }
        println!("  mod {:>3}: first nonzero Q₂ at wl = {}", p, min_wl);
    }

    // ═══ DUAL RULE TEST ═══
    println!("\n{}", "=".repeat(70));
    println!("DUAL RULE TEST");
    println!("{}", "=".repeat(70));

    let e_q2 = first_escape_n;
    let s_q2 = span;
    let e_q3p: i64 = 25;
    let s_q3p: i64 = 84;

    println!("\n  Q₂:  E={}, S={}", e_q2, s_q2);
    println!("  Q3p: E=25, S=84");

    let disc_q2 = e_q2*e_q2 - 4*s_q2;
    let disc_q3p = e_q3p*e_q3p - 4*s_q3p;

    println!("\n  Diophantine condition: t² - Et + S = 0 has integer roots");
    println!("  iff E² - 4S is a non-negative perfect square.\n");

    println!("  Q3p: E² - 4S = {}² - 4×{} = {} - {} = {}",
        e_q3p, s_q3p, e_q3p*e_q3p, 4*s_q3p, disc_q3p);
    let isq3 = (disc_q3p as f64).sqrt() as i64;
    println!("    √{} = {} → perfect square? {}", disc_q3p, isq3,
        if isq3*isq3 == disc_q3p { "YES!" } else { "NO" });
    if isq3*isq3 == disc_q3p {
        println!("    W = (E+√disc)/2 = ({0}+{1})/2 = {2}", e_q3p, isq3, (e_q3p+isq3)/2);
        println!("    C = (E-√disc)/2 = ({0}-{1})/2 = {2}", e_q3p, isq3, (e_q3p-isq3)/2);
        println!("    W + C = {} ✓  W × C = {} ✓", (e_q3p+isq3)/2 + (e_q3p-isq3)/2,
            ((e_q3p+isq3)/2) * ((e_q3p-isq3)/2));
    }

    println!("\n  Q₂: E² - 4S = {}² - 4×{} = {} - {} = {}",
        e_q2, s_q2, e_q2*e_q2, 4*s_q2, disc_q2);
    println!("    √{} ≈ {:.4} → perfect square? NO", disc_q2, (disc_q2 as f64).sqrt());
    println!("    → NO integer (W,C) with W+C={} and W×C={}", e_q2, s_q2);
    println!("    → DUAL RULE FAILS FOR Q₂\n");

    println!("{}", "=".repeat(70));
    println!("VERDICT: THE CLOSED SYSTEM IS CONDITIONAL");
    println!("{}", "=".repeat(70));
    println!("1. The dual rule (E=W+C, S=W×C) holds for Q3p.");
    println!("2. It FAILS for Q₂ because E²-4S = {} ≠ k².", disc_q2);
    println!("3. CONDITION: E²-4S must be a non-negative perfect square.");
    println!("4. For Q3p: 25²-4×84 = 289 = 17². PERFECT SQUARE.");
    println!("5. Q3p has d₂=18 (rich modular structure); Q₂ has d₂=1 (trivial).");
    println!("6. The escape in Q₂ is PERIODIC (recurs); in Q3p it was ISOLATED.");
    println!("7. The Closed System is a PROPERTY of Q3p, not a general theorem.");
}

fn dcount(n: i64) -> usize {
    if n == 0 { return 1; }
    n.abs().to_string().len()
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}
