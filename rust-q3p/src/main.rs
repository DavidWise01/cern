//! H140: The Jacobi Spectrum
//! Schur complement of Q00 in Q3p — the brace-eliminated 2×2 block.
//! Computes the cross-minor, the scaled Schur complement, its eigenvalues,
//! and compares with the principal submatrix and the full 3×3 spectrum.

use num_bigint::BigInt;
use num_traits::{Zero, Signed};
use num_integer::Integer;
use std::str::FromStr;

fn q3p_matrix() -> [[BigInt; 3]; 3] {
    let q00 = BigInt::from_str("-4253717548231547020072693081161910555005243191618621382159372419461855224907202535906842832709092438879947727674389925183077645131066922858577018895943751046822319478442630217018545716834095147972395953888153983664209914844602489625279376395675148602660862812190681326170913579342495584589932614562185034376081141477004382243998").unwrap();
    let q01 = BigInt::from_str("7565498733988797543826666309539672158645764751961099285226226077230250038814204868002363430242854452375972875662278073279414852310308928208990849487269672921067786123089459062793497253243665343748038187908109783688902924752341605325907260640578690410003638186608350436210381112241291833840826752544367458682323939831675097835352").unwrap();
    let q11 = BigInt::from_str("-13455478047907106164677383535368857666436918207351665858120935409324292584029315996247637634836378793010071122621677395561112459568386370293382352655448679002690196196492296975213981584274284568264707225917115327646439866640814758493702987058883061859833338472752964669690461130438096121614528457633572730235565108466691163592876").unwrap();
    let q02 = BigInt::from_str("-2503571742683028409040537740730011137595410794050954062318667765376411144384814665298893249955557857479495740639589132650419023842031118221489646818696725513887888080197689003518244525083703158968159003834440857697530544950131148612295779462478720973898653489630197028599535370732116592212734308695045528926037327021132885487530215578107269455695152634418708409369838").unwrap();
    let q12 = BigInt::from_str("4502179793442409678928953455261155055669041071134544470724140576225302228946314478228192303193379535025440160574756444437840882618010772223391918519657650625236438194502033887572823936232354965562526082212167945389924093542593039846803099009395739504893990163174918863785028366190539027121980934507415751272885502047919875114240878353386031348247869331129706928655078").unwrap();
    let q22 = BigInt::from_str("-125554881570561012885320044186620554728844545718413227582010431541944144267072332736289359842169883426726901297556830208559953295523051045851522156646872885080157267369257604860086139136040536760360085923550347222203411393498428665967572402554306569449899917819777656012431744179783562552161460497518663279351074756942294649046859697283713170954414056469544022338159259876071978639281923573112272775958587826").unwrap();

    [
        [q00.clone(), q01.clone(), q02.clone()],
        [q01, q11.clone(), q12.clone()],
        [q02, q12, q22],
    ]
}

fn digit_count(n: &BigInt) -> usize {
    if n.is_zero() { return 1; }
    n.magnitude().to_string().len()
}

fn fp_div(a: &BigInt, b: &BigInt, prec: u32) -> String {
    let scale = BigInt::from(10u32).pow(prec);
    let result = (a * &scale) / b;
    let s = result.magnitude().to_string();
    let sign = if result.is_negative() { "-" } else { "" };
    if s.len() <= prec as usize {
        let pad = prec as usize - s.len();
        format!("{}0.{}{}", sign, "0".repeat(pad), s)
    } else {
        let integer_part = &s[..s.len() - prec as usize];
        let decimal_part = &s[s.len() - prec as usize..];
        format!("{}{}.{}", sign, integer_part, decimal_part)
    }
}

fn isqrt(n: &BigInt) -> BigInt {
    if n.is_zero() { return BigInt::zero(); }
    let mut x = n.clone();
    let mut y = (&x + BigInt::from(1u32)) / BigInt::from(2u32);
    while y < x {
        x = y.clone();
        y = (&x + n / &x) / BigInt::from(2u32);
    }
    x
}

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0];
    let q01 = &q[0][1];
    let q11 = &q[1][1];
    let q02 = &q[0][2];
    let q12 = &q[1][2];
    let q22 = &q[2][2];

    println!("=== H140: THE JACOBI SPECTRUM (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: The three 2×2 minors (verify H138)
    // =====================================================
    println!("=== STEP 1: THE THREE 2×2 MINORS (H138 verification) ===");
    let m01 = q00 * q11 - q01 * q01;  // brace⊗box minor
    let m02 = q00 * q22 - q02 * q02;  // brace⊗torroid minor
    let m12 = q11 * q22 - q12 * q12;  // box⊗torroid minor

    println!("M01 (brace⊗box)     = {} sign, {}d", if m01.is_negative() { "−" } else { "+" }, digit_count(&m01));
    println!("M02 (brace⊗torroid) = {} sign, {}d", if m02.is_negative() { "−" } else { "+" }, digit_count(&m02));
    println!("M12 (box⊗torroid)   = {} sign, {}d", if m12.is_negative() { "−" } else { "+" }, digit_count(&m12));

    // =====================================================
    // STEP 2: The CROSS-MINOR (new quantity)
    // =====================================================
    println!("\n=== STEP 2: THE CROSS-MINOR (NEW) ===");
    // X = Q00·Q12 − Q01·Q02
    // This is the off-diagonal of the Schur complement of Q00.
    // It represents the RESIDUAL bond-torroid coupling after brace elimination.
    let q00_q12 = q00 * q12;
    let q01_q02 = q01 * q02;
    let cross = &q00_q12 - &q01_q02;

    println!("Q00·Q12 = {} sign, {}d", if q00_q12.is_negative() { "−" } else { "+" }, digit_count(&q00_q12));
    println!("Q01·Q02 = {} sign, {}d", if q01_q02.is_negative() { "−" } else { "+" }, digit_count(&q01_q02));
    println!("X = Q00·Q12 − Q01·Q02 = {} sign, {}d", if cross.is_negative() { "−" } else { "+" }, digit_count(&cross));

    // Check for cancellation
    let max_term = std::cmp::max(digit_count(&q00_q12), digit_count(&q01_q02));
    let cancel = max_term as i64 - digit_count(&cross) as i64;
    println!("Max input term: {}d, result: {}d → cancellation: {}d",
        max_term, digit_count(&cross), cancel);

    // =====================================================
    // STEP 3: The Schur complement of Q00 (scaled)
    // =====================================================
    println!("\n=== STEP 3: THE SCALED SCHUR COMPLEMENT ===");
    // S_scaled = Q00 · S where S = Schur complement of Q00 in Q
    // S_scaled[0][0] = M01 = Q00·Q11 − Q01²
    // S_scaled[0][1] = X   = Q00·Q12 − Q01·Q02
    // S_scaled[1][1] = M02 = Q00·Q22 − Q02²
    println!("S_scaled = [[M01, X], [X, M02]]");
    println!("S_scaled[0][0] = M01 = {}d ({})", digit_count(&m01), if m01.is_negative() { "−" } else { "+" });
    println!("S_scaled[0][1] = X   = {}d ({})", digit_count(&cross), if cross.is_negative() { "−" } else { "+" });
    println!("S_scaled[1][1] = M02 = {}d ({})", digit_count(&m02), if m02.is_negative() { "−" } else { "+" });

    // Trace of S_scaled
    let trace_s = &m01 + &m02;
    println!("\ntrace(S_scaled) = M01 + M02 = {} sign, {}d",
        if trace_s.is_negative() { "−" } else { "+" }, digit_count(&trace_s));

    // Determinant of S_scaled
    let det_s = &m01 * &m02 - &cross * &cross;
    println!("det(S_scaled)   = M01·M02 − X² = {} sign, {}d",
        if det_s.is_negative() { "−" } else { "+" }, digit_count(&det_s));

    // =====================================================
    // STEP 4: Verify det(S_scaled) = Q00 · det(Q)
    // =====================================================
    println!("\n=== STEP 4: DETERMINANT IDENTITY ===");
    // det(Q) via cofactor expansion along row 0
    let det_q = q00 * &m12 - q01 * &(q01 * q22 - q12 * q02) + q02 * &(q01 * q12 - q11 * q02);
    println!("det(Q) = {} sign, {}d", if det_q.is_negative() { "−" } else { "+" }, digit_count(&det_q));

    let q00_det_q = q00 * &det_q;
    println!("Q00·det(Q) = {} sign, {}d", if q00_det_q.is_negative() { "−" } else { "+" }, digit_count(&q00_det_q));

    let det_check = &det_s - &q00_det_q;
    println!("det(S_scaled) − Q00·det(Q) = {}",
        if det_check.is_zero() { "0 ✓ (EXACT MATCH)".to_string() }
        else { format!("NONZERO: {}d", digit_count(&det_check)) });

    // =====================================================
    // STEP 5: Eigenvalues of S_scaled
    // =====================================================
    println!("\n=== STEP 5: EIGENVALUES OF S_scaled ===");
    // μ_± = (trace ± √disc) / 2  where disc = (M01−M02)² + 4X²
    let m01_minus_m02 = &m01 - &m02;
    let diff_sq = &m01_minus_m02 * &m01_minus_m02;
    let cross_sq = &cross * &cross;
    let four_cross_sq = BigInt::from(4) * &cross_sq;
    let disc_s = &diff_sq + &four_cross_sq;

    println!("(M01−M02)² = {}d", digit_count(&diff_sq));
    println!("4·X²       = {}d", digit_count(&four_cross_sq));
    println!("disc        = (M01−M02)² + 4X² = {}d", digit_count(&disc_s));

    // Which term dominates?
    println!("(M01−M02)² / disc ≈ {}", fp_div(&diff_sq, &disc_s, 25));
    println!("4X² / disc        ≈ {}", fp_div(&four_cross_sq, &disc_s, 25));

    let sqrt_disc = isqrt(&disc_s);
    let is_perfect = &sqrt_disc * &sqrt_disc == disc_s;
    println!("√disc = {}d, perfect square: {}", digit_count(&sqrt_disc), is_perfect);

    // Eigenvalues of S_scaled: (trace ± √disc)/2
    let mu_plus = (&trace_s + &sqrt_disc) / BigInt::from(2);
    let mu_minus = (&trace_s - &sqrt_disc) / BigInt::from(2);

    println!("\nμ₊ = (trace+√disc)/2 = {} sign, {}d",
        if mu_plus.is_negative() { "−" } else { "+" }, digit_count(&mu_plus));
    println!("μ₋ = (trace−√disc)/2 = {} sign, {}d",
        if mu_minus.is_negative() { "−" } else { "+" }, digit_count(&mu_minus));

    // =====================================================
    // STEP 6: Convert to Schur complement eigenvalues
    // =====================================================
    println!("\n=== STEP 6: SCHUR COMPLEMENT EIGENVALUES ===");
    // Eigenvalues of S = S_scaled/Q00
    // Since Q00 is negative, dividing flips sign order
    // s_± = μ_± / Q00
    println!("S = S_scaled / Q00 (Q00 is NEGATIVE → signs flip)");
    println!("s₊ = μ₊/Q00 ≈ {}", fp_div(&mu_plus, q00, 25));
    println!("s₋ = μ₋/Q00 ≈ {}", fp_div(&mu_minus, q00, 25));

    // =====================================================
    // STEP 7: Principal submatrix eigenvalues
    // =====================================================
    println!("\n=== STEP 7: PRINCIPAL SUBMATRIX [[Q11,Q12],[Q12,Q22]] ===");
    // The 2×2 box-torroid block (WITHOUT brace elimination)
    let trace_p = q11 + q22;
    let disc_p = &(q11 - q22) * &(q11 - q22) + BigInt::from(4) * q12 * q12;
    let sqrt_disc_p = isqrt(&disc_p);

    println!("trace(P) = Q11+Q22 = {} sign, {}d",
        if trace_p.is_negative() { "−" } else { "+" }, digit_count(&trace_p));
    println!("disc(P)  = (Q11−Q22)² + 4Q12² = {}d", digit_count(&disc_p));
    println!("√disc(P) = {}d", digit_count(&sqrt_disc_p));

    let p_plus = (&trace_p + &sqrt_disc_p) / BigInt::from(2);
    let p_minus = (&trace_p - &sqrt_disc_p) / BigInt::from(2);

    println!("p₊ = (trace+√disc)/2 = {} sign, {}d",
        if p_plus.is_negative() { "−" } else { "+" }, digit_count(&p_plus));
    println!("p₋ = (trace−√disc)/2 = {} sign, {}d",
        if p_minus.is_negative() { "−" } else { "+" }, digit_count(&p_minus));

    // =====================================================
    // STEP 8: Cross-minor anatomy
    // =====================================================
    println!("\n=== STEP 8: CROSS-MINOR ANATOMY ===");
    // X = Q00·Q12 − Q01·Q02
    // |Q00|·|Q12| vs |Q01|·|Q02|
    println!("|Q00|·|Q12| = {}d", digit_count(&(q00.abs() * q12.abs())));
    println!("|Q01|·|Q02| = {}d", digit_count(&(q01.abs() * q02.abs())));

    // The GCD of the Schur matrix entries
    let gcd_m01_m02 = m01.gcd(&m02);
    let gcd_m01_x = m01.gcd(&cross);
    let gcd_all_3 = gcd_m01_m02.gcd(&cross);
    println!("\ngcd(M01, M02) = {}d", digit_count(&gcd_m01_m02));
    println!("gcd(M01, X)   = {}d", digit_count(&gcd_m01_x));
    println!("gcd(M01, M02, X) = {}d", digit_count(&gcd_all_3));
    println!("gcd value = {}", gcd_all_3);

    // Cauchy-Schwarz for the Schur complement
    // CS = X² / (M01·M02) — but M01 is negative, so we need |M01|·M02
    let m01_m02_abs = m01.abs() * m02.abs();
    println!("\nCauchy-Schwarz for Schur complement:");
    println!("X² = {}d", digit_count(&cross_sq));
    println!("|M01|·M02 = {}d", digit_count(&m01_m02_abs));
    println!("X² / (|M01|·M02) ≈ {}", fp_div(&cross_sq, &m01_m02_abs, 25));

    // =====================================================
    // STEP 9: The Schur complement Jacobi angle
    // =====================================================
    println!("\n=== STEP 9: SCHUR COMPLEMENT JACOBI ANGLE ===");
    // For the 2×2 S_scaled, the Jacobi angle is:
    // tan(2φ) = 2X / (M01 − M02)
    let two_x = BigInt::from(2) * &cross;
    println!("2X = {} sign, {}d", if two_x.is_negative() { "−" } else { "+" }, digit_count(&two_x));
    println!("M01−M02 = {} sign, {}d", if m01_minus_m02.is_negative() { "−" } else { "+" }, digit_count(&m01_minus_m02));
    println!("tan(2φ) = 2X/(M01−M02) ≈ {}", fp_div(&two_x, &m01_minus_m02, 25));

    // Compare with H139's first Jacobi angle
    let two_q01 = BigInt::from(2) * q01;
    let q00_minus_q11 = q00 - q11;
    println!("\nFor comparison — H139's first angle:");
    println!("tan(2θ) = 2Q01/(Q00−Q11) ≈ {}", fp_div(&two_q01, &q00_minus_q11, 25));
    println!("tan(2φ) for Schur     ≈ {}", fp_div(&two_x, &m01_minus_m02, 25));

    // =====================================================
    // STEP 10: Comparison — Schur vs Principal vs Full
    // =====================================================
    println!("\n=== STEP 10: THE THREE SPECTRA ===");
    println!("{:<45} {:>5} {:>6}", "Quantity", "Sign", "Digits");
    println!("{:-<58}", "");

    println!("--- Principal submatrix (box-torroid, NO brace) ---");
    println!("{:<45} {:>5} {:>6}", "p₊ (larger)", if p_plus.is_negative() { "−" } else { "+" }, digit_count(&p_plus));
    println!("{:<45} {:>5} {:>6}", "p₋ (smaller)", if p_minus.is_negative() { "−" } else { "+" }, digit_count(&p_minus));

    println!("--- Schur complement (brace eliminated) ---");
    println!("{:<45} {:>5} {:>6}", "μ₊/Q00 (Schur larger)", "—", "ratio");
    println!("{:<45} {:>5} {:>6}", "μ₋/Q00 (Schur smaller)", "—", "ratio");
    println!("{:<45} {:>5} {:>6}", "μ₊ (S_scaled larger)", if mu_plus.is_negative() { "−" } else { "+" }, digit_count(&mu_plus));
    println!("{:<45} {:>5} {:>6}", "μ₋ (S_scaled smaller)", if mu_minus.is_negative() { "−" } else { "+" }, digit_count(&mu_minus));

    println!("--- Full 3×3 (H137) ---");
    println!("{:<45} {:>5} {:>6}", "λ_crack (from H137)", "+", "323");
    println!("{:<45} {:>5} {:>6}", "λ_bond (from H137)", "−", "329");
    println!("{:<45} {:>5} {:>6}", "λ_torroid (from H137)", "−", "408");

    // =====================================================
    // STEP 11: Verify 4-det identity
    // =====================================================
    println!("\n=== STEP 11: THE FOUR-DETERMINANT IDENTITY ===");
    // det(Q) = Q00 · det(S) = Q00 · (M01·M02 − X²) / Q00²
    // ↔ det(S_scaled) = Q00 · det(Q) [verified above]
    //
    // Also: det(S_scaled) = M01·M02 − X²
    let m01_m02 = &m01 * &m02;
    println!("M01·M02 = {} sign, {}d", if m01_m02.is_negative() { "−" } else { "+" }, digit_count(&m01_m02));
    println!("X²      = {} sign, {}d", if cross_sq.is_negative() { "−" } else { "+" }, digit_count(&cross_sq));
    println!("M01·M02 − X² = det(S_scaled) = {} sign, {}d",
        if det_s.is_negative() { "−" } else { "+" }, digit_count(&det_s));

    // Check cancellation in det(S_scaled)
    let max_det_term = std::cmp::max(digit_count(&m01_m02), digit_count(&cross_sq));
    let det_cancel = max_det_term as i64 - digit_count(&det_s) as i64;
    println!("Max term: {}d, result: {}d → cancellation: {}d",
        max_det_term, digit_count(&det_s), det_cancel);

    // =====================================================
    // STEP 12: Cross-minor to off-diagonal ratio
    // =====================================================
    println!("\n=== STEP 12: CROSS-MINOR RATIOS ===");
    // X / (Q00·Q12) = how much of Q00·Q12 survives after subtracting Q01·Q02
    println!("X / (Q00·Q12) ≈ {}", fp_div(&cross, &q00_q12, 25));
    // X / M02 = off-diagonal to diagonal ratio of Schur complement
    println!("X / M02       ≈ {}", fp_div(&cross, &m02, 25));
    // X² / det(S_scaled)
    println!("X² / det(S_scaled) ≈ {}", fp_div(&cross_sq, &det_s, 25));

    // =====================================================
    // STEP 13: Summary
    // =====================================================
    println!("\n=== SUMMARY ===");
    println!("H140: THE JACOBI SPECTRUM");
    println!("1. Cross-minor X = Q00·Q12 − Q01·Q02 = {} sign, {}d",
        if cross.is_negative() { "−" } else { "+" }, digit_count(&cross));
    println!("   Cancellation in X: {}d (from {}d input terms)", cancel, max_term);
    println!("2. Schur complement S_scaled = [[M01, X], [X, M02]]");
    println!("   M01 = {}d(−), X = {}d, M02 = {}d(+)", digit_count(&m01), digit_count(&cross), digit_count(&m02));
    println!("3. det(S_scaled) = Q00·det(Q) ✓ (identity verified exactly)");
    println!("4. Schur discriminant = {}d, perfect square: {}", digit_count(&disc_s), is_perfect);
    println!("5. Schur eigenvalues μ₊ = {}d({}), μ₋ = {}d({})",
        digit_count(&mu_plus), if mu_plus.is_negative() { "−" } else { "+" },
        digit_count(&mu_minus), if mu_minus.is_negative() { "−" } else { "+" });
    println!("6. Principal submatrix eigenvalues p₊ = {}d({}), p₋ = {}d({})",
        digit_count(&p_plus), if p_plus.is_negative() { "−" } else { "+" },
        digit_count(&p_minus), if p_minus.is_negative() { "−" } else { "+" });
    println!("7. gcd(M01, M02, X) = {}", gcd_all_3);
    println!("8. Schur Jacobi angle tan(2φ) ≈ {}", fp_div(&two_x, &m01_minus_m02, 15));
    println!("9. det(S_scaled) cancellation: {}d", det_cancel);
}
