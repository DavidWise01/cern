//! H170: The Witness Matrix
//! Construct Q₃ with the SAME mod-9 residues as Q3p but small coefficients.
//! Test: does d₂=18 follow from the residues alone?
//! Does the closed system (E²-4S=k²) reproduce?

fn main() {
    println!("=== H170: THE WITNESS MATRIX ===\n");

    // Q3p mod 9 residues: Q00≡7, Q01≡6, Q11≡0, Q02≡1, Q12≡6, Q22≡4
    // Q3p parities: ALL even (mod 2 ≡ 0)
    // So entries must be: even AND have specific mod-9 residues.
    // CRT(mod 2, mod 9) → mod 18:
    //   Q00: even, ≡7 mod 9 → ≡16 mod 18 (or negative: -2 mod 18)
    //   Q01: even, ≡6 mod 9 → ≡6 mod 18
    //   Q11: even, ≡0 mod 9 → ≡0 mod 18
    //   Q02: even, ≡1 mod 9 → ≡10 mod 18
    //   Q12: even, ≡6 mod 9 → ≡6 mod 18
    //   Q22: even, ≡4 mod 9 → ≡4 mod 18

    println!("CRT(mod 2, mod 9) → mod 18 residues:");
    println!("  Q00: even ∧ ≡7 mod 9 → ≡16 mod 18");
    println!("  Q01: even ∧ ≡6 mod 9 → ≡6 mod 18");
    println!("  Q11: even ∧ ≡0 mod 9 → ≡0 mod 18");
    println!("  Q02: even ∧ ≡1 mod 9 → ≡10 mod 18");
    println!("  Q12: even ∧ ≡6 mod 9 → ≡6 mod 18");
    println!("  Q22: even ∧ ≡4 mod 9 → ≡4 mod 18\n");

    // Build small matrices: entries = base + 18k for various k
    // Need: signature (1+, 2−), R01 > 0 and not perfect square
    // Smallest candidates with correct residues:
    // Q00 ∈ {-2, -20, -38, -56, -74, ...} (negative diagonal, ≡16 mod 18)
    // Q01 ∈ {6, 24, 42, 60, 78, ...} (positive off-diag, ≡6 mod 18)
    // Q11 ∈ {-18, -36, -54, -72, -90, ...} (negative diagonal, ≡0 mod 18)
    // Q02 ∈ {10, 28, 46, 64, ...} or {-8, -26, -44, ...} (≡10 mod 18)
    // Q12 ∈ {6, 24, 42, 60, ...} or {-12, -30, ...} (≡6 mod 18)
    // Q22 ∈ {-14, -32, -50, -68, ...} (negative diagonal, ≡4 mod 18)

    println!("═══ SEARCHING FOR WITNESS MATRICES ═══\n");

    // We need: Q01² > |Q00|·|Q11| (so R01 > 0, indefinite binary part)
    // Try systematic search over small multiples

    let mut witnesses = Vec::new();

    // Generate candidates with correct mod-18 residues
    let q00_vals: Vec<i64> = (1..=50).map(|k| -18*k + 16).filter(|x| *x < 0).collect();
    let q11_vals: Vec<i64> = (1..=50).map(|k| -18*k).collect();
    let q01_vals: Vec<i64> = (0..=50).map(|k| 18*k + 6).filter(|x| *x > 0).collect();
    let q22_vals: Vec<i64> = (1..=200).map(|k| -18*k + 4).collect();
    let q02_vals: Vec<i64> = vec![10, 28, -8, -26];
    let q12_vals: Vec<i64> = vec![6, 24, -12, -30];
    println!("Q00 candidates: {} (first few: {:?})", q00_vals.len(), &q00_vals[..3.min(q00_vals.len())]);
    println!("Q01 candidates: {} (first few: {:?})", q01_vals.len(), &q01_vals[..3.min(q01_vals.len())]);
    println!("Q11 candidates: {} (first few: {:?})", q11_vals.len(), &q11_vals[..3.min(q11_vals.len())]);

    for &q02 in &q02_vals {
    for &q12 in &q12_vals {
    for &q00 in &q00_vals {
        for &q01 in &q01_vals {
            if q01.abs() > q00.abs() * 3 { break; }
            for &q11 in &q11_vals {
                let r01 = q01*q01 - q00.abs()*q11.abs();
                if r01 <= 0 { continue; } // need indefinite
                // Check R01 not perfect square
                let sr = (r01 as f64).sqrt() as i64;
                if sr*sr == r01 || (sr+1)*(sr+1) == r01 { continue; }

                // Need signature (1+, 2−): check via determinant signs
                // M1 = Q00 < 0 ✓ (1 neg eigenvalue)
                // M2 = Q00·Q11 - Q01² (should be negative for 2 neg eigenvalues? depends)
                // Actually for sig (1+,2−), need det > 0
                for &q22 in &q22_vals {
                    if q22.abs() < 10 { continue; }
                    // Compute det
                    let det = q00*(q11*q22 - q12*q12)
                            - q01*(q01*q22 - q12*q02)
                            + q02*(q01*q12 - q11*q02);
                    if det <= 0 { continue; } // need det > 0 for sig (1+,2−)

                    // Compute all 2×2 minors
                    let m1 = q00*q11 - q01*q01;
                    let m2 = q00*q12 - q01*q02;
                    let m3 = q01*q12 - q11*q02;
                    let m4 = q00*q22 - q02*q02;
                    let m5 = q01*q22 - q02*q12;
                    let m6 = q11*q22 - q12*q12;

                    let minors = [m1.abs(), m2.abs(), m3.abs(), m4.abs(), m5.abs(), m6.abs()];
                    let g = minors.iter().filter(|&&x| x > 0).fold(0i64, |acc, &x| {
                        if acc == 0 { x } else { gcd(acc, x) }
                    });

                    let d1 = 2i64; // by construction all entries even
                    let d2 = g / d1;

                    if d2 != 18 { continue; }

                    // Found a witness with d₂ = 18!
                    // Now compute escape
                    let eval = |a: i64, b: i64, c: i64| -> i64 {
                        q00*a*a + 2*q01*a*b + q11*b*b + 2*q02*a*c + 2*q12*b*c + q22*c*c
                    };

                    let mut first_escape = 0i64;
                    let mut escape_vec = (0i64, 0i64, 0i64);
                    for n in 1..=50i64 {
                        let mut found = false;
                        for a in -n..=n {
                            let rem = n - a.abs();
                            for b in -rem..=rem {
                                let c_abs = rem - b.abs();
                                let c_vals: Vec<i64> = if c_abs == 0 { vec![0] } else { vec![c_abs, -c_abs] };
                                for &cv in &c_vals {
                                    if a==0 && b==0 && cv==0 { continue; }
                                    if eval(a,b,cv) > 0 {
                                        first_escape = n;
                                        escape_vec = (a,b,cv);
                                        found = true;
                                        break;
                                    }
                                }
                                if found { break; }
                            }
                            if found { break; }
                        }
                        if found { break; }
                    }

                    if first_escape == 0 { continue; } // no escape found

                    // Cofactor span
                    let cof_digits: Vec<usize> = minors.iter().map(|x| x.to_string().len()).collect();
                    let span = (*cof_digits.iter().max().unwrap() as i64)
                             - (*cof_digits.iter().filter(|&&d| d > 0).min().unwrap() as i64);

                    // Test Diophantine condition
                    let e = first_escape;
                    let s = span;
                    let disc = e*e - 4*s;
                    let sr2 = if disc >= 0 { (disc as f64).sqrt() as i64 } else { -1 };
                    let is_perf = disc >= 0 && sr2 >= 0 && sr2*sr2 == disc;

                    witnesses.push((q00, q01, q11, q02, q12, q22, d2, r01, det,
                        first_escape, escape_vec, span, disc, is_perf, g));
                    if witnesses.len() >= 30 { break; }
                }
                if witnesses.len() >= 30 { break; }
            }
            if witnesses.len() >= 30 { break; }
        }
        if witnesses.len() >= 30 { break; }
    }
    if witnesses.len() >= 30 { break; }
    } // q12
    if witnesses.len() >= 30 { break; }
    } // q02

    println!("Found {} witness matrices with d₂ = 18\n", witnesses.len());

    println!("{:<6} {:<6} {:<6} {:<5} {:<5} {:<7} {:>5} {:>5} {:>5} {:>5} {:>8} {:>5}",
        "Q00", "Q01", "Q11", "Q02", "Q12", "Q22", "d₂", "E", "S", "disc", "k²?", "gcd36");
    println!("{}", "-".repeat(85));

    for (q00,q01,q11,q02,q12,q22,d2,_r01,_det,e,_ev,s,disc,perf,g) in &witnesses {
        println!("{:<6} {:<6} {:<6} {:<5} {:<5} {:<7} {:>5} {:>5} {:>5} {:>5} {:>8} {:>5}",
            q00, q01, q11, q02, q12, q22, d2, e, s, disc,
            if *perf { "YES" } else { "no" }, g);
    }

    // Analyze: how many have E²-4S = perfect square?
    let perf_count = witnesses.iter().filter(|w| w.13).count();
    let total = witnesses.len();
    println!("\nWitnesses with E²-4S = k²: {}/{}", perf_count, total);

    // Show the ones that DO have perfect square discriminant
    println!("\n═══ WITNESSES WITH PERFECT SQUARE DISCRIMINANT ═══");
    for (q00,q01,q11,q02,q12,q22,d2,r01,det,e,ev,s,disc,perf,g) in &witnesses {
        if !perf { continue; }
        let k = (*disc as f64).sqrt() as i64;
        let w = (*e + k) / 2;
        let c = (*e - k) / 2;
        println!("\n  Q₃ = [{},{},{}; {},{},{}]", q00, q01, q11, q02, q12, q22);
        println!("  R01={}, det={}, gcd(minors)={}, d₂={}", r01, det, g, d2);
        println!("  E={}, S={}, E²-4S={} = {}²", e, s, disc, k);
        println!("  W=({}+{})/2={}, C=({}-{})/2={}", e, k, w, e, k, c);
        println!("  W+C={} ✓  W×C={} vs S={} {}",
            w+c, w*c, s, if w*c == *s { "✓" } else { "✗" });
        println!("  Escape vector: ({},{},{})", ev.0, ev.1, ev.2);
    }

    // Show the ones that DON'T
    println!("\n═══ WITNESSES WHERE DISCRIMINANT IS NOT PERFECT SQUARE ═══");
    let mut not_perf_count = 0;
    for (q00,q01,q11,_q02,_q12,q22,_d2,_r01,_det,e,_ev,s,disc,perf,_g) in &witnesses {
        if *perf { continue; }
        not_perf_count += 1;
        if not_perf_count <= 5 {
            println!("  [{},{},{},...,{}]: E={}, S={}, E²-4S={}  (not k²)",
                q00, q01, q11, q22, e, s, disc);
        }
    }
    if not_perf_count > 5 { println!("  ... and {} more", not_perf_count - 5); }

    // ═══ VERDICT ═══
    println!("\n{}", "=".repeat(60));
    println!("VERDICT");
    println!("{}", "=".repeat(60));
    println!("\n  d₂ = 18 witnesses found: {}", total);
    println!("  Of those, E²-4S = k²: {}", perf_count);
    println!("  Of those, E²-4S ≠ k²: {}", total - perf_count);

    if perf_count > 0 && perf_count < total {
        println!("\n  FINDING: d₂ = 18 is NECESSARY but NOT SUFFICIENT.");
        println!("  The mod-9 residues force d₂ = 18, but NOT all d₂=18");
        println!("  matrices have the closed system. The escape time E and");
        println!("  cofactor span S must ALSO align — and that depends on");
        println!("  the coefficient magnitudes, not just the residues.");
        println!("  The giant coefficients are NOT decoration. They carry content.");
    } else if perf_count == total && total > 0 {
        println!("\n  FINDING: ALL d₂=18 witnesses have E²-4S = k²!");
        println!("  The residues ALONE determine the closed system.");
        println!("  The giant coefficients are decoration.");
    } else if perf_count == 0 {
        println!("\n  FINDING: NO d₂=18 witness has E²-4S = k²!");
        println!("  Q3p's closed system requires BOTH d₂=18 AND giant coefficients.");
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}
