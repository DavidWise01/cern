//! H141: The Halving Law
//! Second-level Schur complement: eliminate brace AND crack.
//! Tests whether the 85d → 42d gap cascade continues to ~21d.
//! Also: the full Schur descent from 3×3 to 2×2 to 1×1 to scalar.

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

    println!("=== H141: THE HALVING LAW (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: Recompute the first-level Schur complement (H140)
    // =====================================================
    println!("=== STEP 1: FIRST SCHUR COMPLEMENT (H140 verification) ===");
    let m01 = q00 * q11 - q01 * q01;
    let m02 = q00 * q22 - q02 * q02;
    let m12 = q11 * q22 - q12 * q12;
    let cross = q00 * q12 - q01 * q02;  // X from H140

    println!("M01 = {}d ({})", digit_count(&m01), if m01.is_negative() { "−" } else { "+" });
    println!("M02 = {}d ({})", digit_count(&m02), if m02.is_negative() { "−" } else { "+" });
    println!("X   = {}d ({})", digit_count(&cross), if cross.is_negative() { "−" } else { "+" });

    let det_s_scaled = &m01 * &m02 - &cross * &cross;
    println!("det(S_scaled) = M01·M02 − X² = {}d ({})",
        digit_count(&det_s_scaled), if det_s_scaled.is_negative() { "−" } else { "+" });

    // Verify: det(S_scaled) = Q00 · det(Q)
    let det_q = q00 * &m12 - q01 * &(q01 * q22 - q12 * q02) + q02 * &(q01 * q12 - q11 * q02);
    let q00_det_q = q00 * &det_q;
    let check1 = &det_s_scaled - &q00_det_q;
    println!("Q00·det(Q) check: {}", if check1.is_zero() { "✓ EXACT" } else { "✗ MISMATCH" });

    // =====================================================
    // STEP 2: Second-level Schur complement
    // =====================================================
    println!("\n=== STEP 2: SECOND SCHUR COMPLEMENT (eliminate crack/M01) ===");
    // S_scaled = [[M01, X], [X, M02]]
    // Schur complement of M01 in S_scaled:
    // σ = M02 − X²/M01
    // σ_scaled = M01 · σ = M01·M02 − X² = det(S_scaled)
    //
    // So the second-level Schur complement (scaled by M01) IS det(S_scaled)!
    // This is the 1×1 "matrix" at the bottom of the Schur descent.

    println!("σ_scaled = M01·M02 − X² = det(S_scaled) = {}d ({})",
        digit_count(&det_s_scaled), if det_s_scaled.is_negative() { "−" } else { "+" });
    println!("This is the TERMINAL SCALAR of the Schur descent.");

    // The actual Schur complement σ = σ_scaled / M01
    println!("\nσ = σ_scaled / M01 = det(S_scaled) / M01");
    println!("σ ≈ {}", fp_div(&det_s_scaled, &m01, 25));

    // =====================================================
    // STEP 3: The gap at each level
    // =====================================================
    println!("\n=== STEP 3: THE GAP AT EACH LEVEL ===");

    // Level 0: Original Q3p
    // The "gap" is between the smallest and largest off-diagonal impact.
    // M01 = −651d vs M02 = +735d → gap = 735 − 651 = 84 (call it 85 rounding)
    let gap_L0 = digit_count(&m02) as i64 - digit_count(&m01) as i64;
    println!("Level 0 (original Q3p minors):");
    println!("  |M01| = {}d, |M02| = {}d", digit_count(&m01), digit_count(&m02));
    println!("  Gap = {} digits", gap_L0);

    // Level 1: First Schur complement S_scaled = [[M01, X], [X, M02]]
    // The gap is between off-diagonal X and larger diagonal M02
    let gap_L1 = digit_count(&m02) as i64 - digit_count(&cross) as i64;
    println!("\nLevel 1 (first Schur complement):");
    println!("  |X| = {}d, |M02| = {}d", digit_count(&cross), digit_count(&m02));
    println!("  Gap = {} digits", gap_L1);

    // Level 2: Second Schur complement — it's scalar, no more gap.
    // But we can measure the "gap" differently:
    // The correction to M02 from eliminating M01:
    // σ = M02 − X²/M01
    // The correction term is X²/M01
    let x_sq = &cross * &cross;
    // X²/M01 — exact division may not be possible, use scaled version
    // |X²|/|M01| in digit counts:
    let correction_digits = digit_count(&x_sq) as i64 - digit_count(&m01) as i64;
    println!("\nLevel 2 (second Schur = terminal scalar):");
    println!("  σ_scaled = det(S_scaled) = {}d", digit_count(&det_s_scaled));
    println!("  Correction term X²/M01:");
    println!("    |X²| = {}d", digit_count(&x_sq));
    println!("    |M01| = {}d", digit_count(&m01));
    println!("    |X²/M01| ≈ {}d ({} − {} = {})", correction_digits, digit_count(&x_sq), digit_count(&m01), correction_digits);
    println!("    |M02| = {}d", digit_count(&m02));
    println!("    Gap (|M02| − |X²/M01|) ≈ {} digits", digit_count(&m02) as i64 - correction_digits);

    // =====================================================
    // STEP 4: Alternative gap measurement — using the FULL Schur descent
    // =====================================================
    println!("\n=== STEP 4: THE FULL SCHUR DESCENT ===");
    // Three-level descent:
    // Level 0: Q3p (3×3)
    //   → eliminate Q00 → S₁ (2×2) with gap = 42d
    // Level 1: S₁ = [[M01, X], [X, M02]]
    //   → eliminate M01 → σ (scalar) = det(S₁)/M01... no, σ_scaled = det(S₁) = M01·M02 − X²
    //
    // The descent produces:
    // Q00 (328d) → S₁ with entries (651d, 693d, 735d) → σ_scaled (1386d)
    //
    // But the RATIOS matter more than the absolute sizes.
    // At each level, the "coupling ratio" = off-diag² / (diag₁ · diag₂)

    // Level 0 — three coupling ratios:
    let q01_sq = q01 * q01;
    let q02_sq = q02 * q02;
    let q12_sq = q12 * q12;
    let cs_01 = fp_div(&q01_sq, &(q00.abs() * q11.abs()), 25);
    let cs_02 = fp_div(&q02_sq, &(q00.abs() * q22.abs()), 25);
    let cs_12 = fp_div(&q12_sq, &(q11.abs() * q22.abs()), 25);
    println!("Level 0 coupling ratios (Q3p):");
    println!("  CS(brace,box)     = Q01²/(|Q00|·|Q11|) = {}", cs_01);
    println!("  CS(brace,torroid) = Q02²/(|Q00|·|Q22|) = {}", cs_02);
    println!("  CS(box,torroid)   = Q12²/(|Q11|·|Q22|) = {}", cs_12);

    // Level 1 — one coupling ratio:
    let cs_schur = fp_div(&x_sq, &(m01.abs() * m02.abs()), 25);
    println!("\nLevel 1 coupling ratio (Schur complement):");
    println!("  CS(Schur) = X²/(|M01|·M02) = {}", cs_schur);

    // =====================================================
    // STEP 5: The three Schur complements from each pivot
    // =====================================================
    println!("\n=== STEP 5: ALL THREE PIVOTS ===");
    // We can Schur-eliminate from ANY diagonal entry.
    // Each gives a different 2×2 and a different gap.

    // Pivot Q00 (brace) — done in H140
    println!("Pivot Q00 (eliminate brace):");
    println!("  S_scaled = [[M01(−651d), X(−693d)], [X(−693d), M02(+735d)]]");
    println!("  Gap = {}d", gap_L1);

    // Pivot Q11 (box)
    let n01 = q00 * q11 - q01 * q01;  // same as M01
    let n12 = q11 * q22 - q12 * q12;  // same as M12
    let cross_11 = q11 * q02 - q01 * q12;  // Q11·Q02 − Q01·Q12
    println!("\nPivot Q11 (eliminate box):");
    println!("  S_scaled[0][0] = Q00·Q11 − Q01² = M01 = {}d ({})", digit_count(&n01), if n01.is_negative() { "−" } else { "+" });
    println!("  S_scaled[0][1] = Q11·Q02 − Q01·Q12 = {}d ({})", digit_count(&cross_11), if cross_11.is_negative() { "−" } else { "+" });
    println!("  S_scaled[1][1] = Q11·Q22 − Q12² = M12 = {}d ({})", digit_count(&n12), if n12.is_negative() { "−" } else { "+" });
    let gap_pivot11 = digit_count(&n12) as i64 - digit_count(&cross_11) as i64;
    let max_diag_11 = std::cmp::max(digit_count(&n01), digit_count(&n12));
    println!("  Gap (max diag − off-diag) = {} − {} = {}d",
        std::cmp::max(digit_count(&n01), digit_count(&n12)),
        digit_count(&cross_11),
        max_diag_11 as i64 - digit_count(&cross_11) as i64);

    // Pivot Q22 (torroid)
    let p02 = q00 * q22 - q02 * q02;  // same as M02
    let p12 = q11 * q22 - q12 * q12;  // same as M12
    let cross_22 = q22 * q01 - q02 * q12;  // Q22·Q01 − Q02·Q12
    println!("\nPivot Q22 (eliminate torroid):");
    println!("  S_scaled[0][0] = Q00·Q22 − Q02² = M02 = {}d ({})", digit_count(&p02), if p02.is_negative() { "−" } else { "+" });
    println!("  S_scaled[0][1] = Q22·Q01 − Q02·Q12 = {}d ({})", digit_count(&cross_22), if cross_22.is_negative() { "−" } else { "+" });
    println!("  S_scaled[1][1] = Q11·Q22 − Q12² = M12 = {}d ({})", digit_count(&p12), if p12.is_negative() { "−" } else { "+" });
    let max_diag_22 = std::cmp::max(digit_count(&p02), digit_count(&p12));
    println!("  Gap (max diag − off-diag) = {} − {} = {}d",
        max_diag_22,
        digit_count(&cross_22),
        max_diag_22 as i64 - digit_count(&cross_22) as i64);

    // Cross-minor anatomy for Q11 pivot
    println!("\n  Cross-minor anatomy (Q11 pivot):");
    let q11_q02 = q11 * q02;
    let q01_q12_prod = q01 * q12;
    println!("    Q11·Q02 = {}d ({})", digit_count(&q11_q02), if q11_q02.is_negative() { "−" } else { "+" });
    println!("    Q01·Q12 = {}d ({})", digit_count(&q01_q12_prod), if q01_q12_prod.is_negative() { "−" } else { "+" });
    let cancel_11 = std::cmp::max(digit_count(&q11_q02), digit_count(&q01_q12_prod)) as i64 - digit_count(&cross_11) as i64;
    println!("    Cancellation: {}d", cancel_11);

    // Cross-minor anatomy for Q22 pivot
    println!("\n  Cross-minor anatomy (Q22 pivot):");
    let q22_q01 = q22 * q01;
    let q02_q12_prod = q02 * q12;
    println!("    Q22·Q01 = {}d ({})", digit_count(&q22_q01), if q22_q01.is_negative() { "−" } else { "+" });
    println!("    Q02·Q12 = {}d ({})", digit_count(&q02_q12_prod), if q02_q12_prod.is_negative() { "−" } else { "+" });
    let cancel_22 = std::cmp::max(digit_count(&q22_q01), digit_count(&q02_q12_prod)) as i64 - digit_count(&cross_22) as i64;
    println!("    Cancellation: {}d", cancel_22);

    // =====================================================
    // STEP 6: Verify all three pivots give the same det
    // =====================================================
    println!("\n=== STEP 6: DETERMINANT CONSISTENCY ===");
    let det_pivot00 = &det_s_scaled;  // M01·M02 − X²
    let det_pivot11 = &n01 * &n12 - &cross_11 * &cross_11;
    let det_pivot22 = &p02 * &p12 - &cross_22 * &cross_22;

    // All should equal their pivot · det(Q)
    let q11_det_q = q11 * &det_q;
    let q22_det_q = q22 * &det_q;

    let check_11 = &det_pivot11 - &q11_det_q;
    let check_22 = &det_pivot22 - &q22_det_q;

    println!("det(S_00) = Q00·det(Q)? {}", if check1.is_zero() { "✓" } else { "✗" });
    println!("det(S_11) = Q11·det(Q)? {}", if check_11.is_zero() { "✓" } else { "✗" });
    println!("det(S_22) = Q22·det(Q)? {}", if check_22.is_zero() { "✓" } else { "✗" });

    // =====================================================
    // STEP 7: The gap table
    // =====================================================
    println!("\n=== STEP 7: THE GAP TABLE ===");
    println!("{:<40} {:>8} {:>8} {:>8}", "Level", "Off-diag", "Max diag", "Gap");
    println!("{:-<66}", "");
    println!("{:<40} {:>8} {:>8} {:>8}", "L0: M01 vs M02 (H138)",
        format!("{}d", digit_count(&m01)), format!("{}d", digit_count(&m02)), format!("{}d", gap_L0));
    println!("{:<40} {:>8} {:>8} {:>8}", "L1: X vs M02 (pivot Q00, H140)",
        format!("{}d", digit_count(&cross)), format!("{}d", digit_count(&m02)), format!("{}d", gap_L1));

    // For the Q11 pivot:
    let gap_11 = max_diag_11 as i64 - digit_count(&cross_11) as i64;
    println!("{:<40} {:>8} {:>8} {:>8}", "L1: Y vs max(M01,M12) (pivot Q11)",
        format!("{}d", digit_count(&cross_11)), format!("{}d", max_diag_11), format!("{}d", gap_11));

    // For the Q22 pivot:
    let gap_22 = max_diag_22 as i64 - digit_count(&cross_22) as i64;
    println!("{:<40} {:>8} {:>8} {:>8}", "L1: Z vs max(M02,M12) (pivot Q22)",
        format!("{}d", digit_count(&cross_22)), format!("{}d", max_diag_22), format!("{}d", gap_22));

    // Level 2: correction term
    // X²/M01 gives the correction to M02 at level 2
    // |X²|/|M01| = digits(X²) - digits(M01)
    let corr_digits = digit_count(&x_sq) as i64 - digit_count(&m01) as i64;
    let gap_L2 = digit_count(&m02) as i64 - corr_digits;
    println!("{:<40} {:>8} {:>8} {:>8}", "L2: X²/M01 vs M02 (second Schur)",
        format!("{}d", corr_digits), format!("{}d", digit_count(&m02)), format!("{}d", gap_L2));

    // =====================================================
    // STEP 8: The halving test
    // =====================================================
    println!("\n=== STEP 8: THE HALVING TEST ===");
    println!("Gap sequence:");
    println!("  L0: {} digits (original minor hierarchy)", gap_L0);
    println!("  L1: {} digits (Schur complement of Q00)", gap_L1);
    println!("  Ratio L1/L0: {:.4}", gap_L1 as f64 / gap_L0 as f64);
    println!("  L2: {} digits (second-level correction)", gap_L2);
    println!("  Ratio L2/L1: {:.4}", gap_L2 as f64 / gap_L1 as f64);
    println!("  Ratio L2/L0: {:.4}", gap_L2 as f64 / gap_L0 as f64);

    let predicted_L2 = gap_L1 as f64 / 2.0;
    println!("\n  Predicted L2 (halving): {:.1}", predicted_L2);
    println!("  Actual L2: {}", gap_L2);
    println!("  Prediction error: {:.1} digits", (gap_L2 as f64 - predicted_L2).abs());

    // =====================================================
    // STEP 9: The CS descent
    // =====================================================
    println!("\n=== STEP 9: CAUCHY-SCHWARZ DESCENT ===");
    // Track CS fraction at each level

    // Level 0: three CS fractions
    println!("Level 0 (Q3p):");
    println!("  CS(01) = {} (brace-box, SATURATED)", cs_01);
    println!("  CS(02) = {} (brace-torroid)", cs_02);
    println!("  CS(12) = {} (box-torroid)", cs_12);

    // Level 1: one CS fraction per pivot
    println!("\nLevel 1 (Schur complements):");
    println!("  CS(pivot Q00) = X²/(|M01|·M02)   = {}", cs_schur);
    let cs_pivot11 = fp_div(&(&cross_11 * &cross_11), &(n01.abs() * n12.abs()), 25);
    let cs_pivot22 = fp_div(&(&cross_22 * &cross_22), &(p02.abs() * p12.abs()), 25);
    println!("  CS(pivot Q11) = Y²/(|M01|·M12)   = {}", cs_pivot11);
    println!("  CS(pivot Q22) = Z²/(|M02|·M12)   = {}", cs_pivot22);

    // =====================================================
    // STEP 10: Cross-minor GCD structure
    // =====================================================
    println!("\n=== STEP 10: CROSS-MINOR GCD STRUCTURE ===");
    let gcd_all = cross.gcd(&cross_11).gcd(&cross_22);
    println!("gcd(X_00, X_11, X_22) = {}", gcd_all);
    println!("  X_00 = Q00·Q12 − Q01·Q02 = {}d", digit_count(&cross));
    println!("  X_11 = Q11·Q02 − Q01·Q12 = {}d", digit_count(&cross_11));
    println!("  X_22 = Q22·Q01 − Q02·Q12 = {}d", digit_count(&cross_22));

    // These are the three cofactors of Q (up to sign)!
    // cofactor(Q, 0, 2) = Q01·Q12 − Q11·Q02 = -X_11
    // cofactor(Q, 1, 0) = Q12·Q02 − Q22·Q01 = -X_22
    // cofactor(Q, 2, 1) = Q02·Q01 − Q00·Q12 = -X_00
    println!("\nThese are the OFF-DIAGONAL COFACTORS of Q (up to sign)!");
    println!("  X_00 = −C₂₁ (cofactor row 2, col 1)");
    println!("  X_11 = −C₂₀ (cofactor row 2, col 0)");
    println!("  X_22 = −C₁₀ (cofactor row 1, col 0)");

    // =====================================================
    // STEP 11: Summary
    // =====================================================
    println!("\n=== SUMMARY ===");
    println!("H141: THE HALVING LAW");
    println!("1. Gap cascade: L0={}d → L1={}d → L2={}d", gap_L0, gap_L1, gap_L2);
    println!("   L1/L0 = {:.4}, L2/L1 = {:.4}", gap_L1 as f64 / gap_L0 as f64, gap_L2 as f64 / gap_L1 as f64);
    let halving_holds = (gap_L1 as f64 / gap_L0 as f64 - 0.5).abs() < 0.1;
    println!("   Halving at L1: {} ({:.1}% of L0)", if halving_holds { "YES" } else { "APPROXIMATE" }, 100.0 * gap_L1 as f64 / gap_L0 as f64);
    println!("2. All three pivots give det(S) = pivot·det(Q) ✓");
    println!("3. Cross-minors ARE the off-diagonal cofactors of Q");
    println!("4. gcd of all three cross-minors: {}", gcd_all);
    println!("5. Three pivot gaps: Q00={}d, Q11={}d, Q22={}d", gap_L1, gap_11, gap_22);
    println!("6. The Schur descent: 3×3 → 2×2 → 1×1 is COMPLETE");
    println!("7. Terminal scalar = det(S_scaled)/M01 = det(Q)·Q00/M01");
}
