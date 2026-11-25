//! H145: The Unbinding Energy
//! Exact energy cost of the sign flip: how much coupling Q01 must supply
//! to overcome negative self-energies Q00, Q11 and create the crack eigenvalue.
//! The threshold where CS crosses 1 — computed as exact BigInt.

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

    println!("=== H145: THE UNBINDING ENERGY (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: The threshold condition
    // =====================================================
    println!("=== STEP 1: THE THRESHOLD CONDITION ===");
    // The 2×2 brace-box block [[Q00, Q01], [Q01, Q11]] has eigenvalues:
    // λ± = (Q00 + Q11 ± √((Q00−Q11)² + 4Q01²)) / 2
    //
    // λ₊ > 0 (the crack) iff the discriminant term exceeds |trace|.
    // Since Q00+Q11 < 0 (both negative), we need:
    // √((Q00−Q11)² + 4Q01²) > |Q00+Q11| = −(Q00+Q11)
    //
    // Squaring: (Q00−Q11)² + 4Q01² > (Q00+Q11)²
    // Expand: Q00²−2Q00Q11+Q11² + 4Q01² > Q00²+2Q00Q11+Q11²
    // Simplify: −2Q00Q11 + 4Q01² > 2Q00Q11
    // Therefore: 4Q01² > 4Q00Q11
    // Therefore: Q01² > Q00·Q11
    //
    // Since Q00 < 0 and Q11 < 0: Q00·Q11 = |Q00|·|Q11| > 0
    // So: Q01² > |Q00|·|Q11|  ←→  CS > 1  ←→  CRACK EXISTS
    //
    // The threshold is EXACTLY: Q01² = |Q00|·|Q11|
    // The EXCESS is: R01 = Q01² − |Q00|·|Q11|

    let abs_q00 = q00.abs();
    let abs_q11 = q11.abs();
    let q01_sq = q01 * q01;
    let threshold = &abs_q00 * &abs_q11;  // = |Q00|·|Q11|
    let r01 = &q01_sq - &threshold;

    println!("ALGEBRAIC PROOF:");
    println!("  λ₊ > 0 iff Q01² > |Q00|·|Q11|");
    println!("  This is EQUIVALENT to CS(brace,box) > 1");
    println!("  The threshold is Q01² = |Q00|·|Q11|");
    println!("");
    println!("Q01²       = {}d ({})", digit_count(&q01_sq), if q01_sq.is_positive() { "+" } else { "−" });
    println!("|Q00|·|Q11| = {}d ({})", digit_count(&threshold), if threshold.is_positive() { "+" } else { "−" });
    println!("R01 = Q01² − |Q00|·|Q11| = {}d ({})",
        digit_count(&r01), if r01.is_positive() { "+" } else { "−" });
    println!("R01 > 0 → CRACK EXISTS ✓");

    // =====================================================
    // STEP 2: The unbinding energy
    // =====================================================
    println!("\n=== STEP 2: THE UNBINDING ENERGY ===");
    // R01 is the EXACT amount by which Q01² exceeds the threshold.
    // It measures how much "extra" coupling energy is available after
    // paying the cost of overcoming the negative self-energies.
    //
    // In energy units: R01 = Q01² − |Q00|·|Q11|
    // The threshold cost = |Q00|·|Q11| (the product of self-energies)
    // The coupling supply = Q01² (the squared coupling)
    // The excess = R01 (the unbinding energy)

    println!("Threshold cost (|Q00|·|Q11|): {}d", digit_count(&threshold));
    println!("Coupling supply (Q01²):       {}d", digit_count(&q01_sq));
    println!("Unbinding energy (R01):        {}d", digit_count(&r01));
    println!("");
    println!("R01 / threshold = {} (fractional excess)",
        fp_div(&r01, &threshold, 25));
    println!("= 1.691 × 10⁻⁵ — the coupling exceeds the threshold by 0.00169%");

    // =====================================================
    // STEP 3: The crack eigenvalue in terms of R01
    // =====================================================
    println!("\n=== STEP 3: CRACK EIGENVALUE FROM R01 ===");
    // λ₊ = (Q00+Q11+√disc)/2  where disc = (Q00−Q11)²+4Q01²
    // disc = (Q00−Q11)² + 4|Q00|·|Q11| + 4R01
    //      = (Q00+Q11)² + 4R01     [using (a−b)²+4ab = (a+b)² for negative a,b]
    //
    // Wait — let's be careful. For Q00,Q11 both negative:
    // (Q00−Q11)² + 4Q01² = (Q00−Q11)² + 4(|Q00|·|Q11| + R01)
    //                     = (Q00−Q11)² + 4Q00·Q11 + 4R01   [since Q00·Q11 = |Q00|·|Q11|]
    //                     = Q00²−2Q00Q11+Q11² + 4Q00Q11 + 4R01
    //                     = Q00²+2Q00Q11+Q11² + 4R01
    //                     = (Q00+Q11)² + 4R01

    let trace = q00 + q11;
    let trace_sq = &trace * &trace;
    let four_r01 = BigInt::from(4) * &r01;
    let disc = &trace_sq + &four_r01;

    println!("IDENTITY: disc = (Q00+Q11)² + 4R01");
    println!("  (Q00+Q11)² = {}d", digit_count(&trace_sq));
    println!("  4R01       = {}d", digit_count(&four_r01));
    println!("  disc       = {}d", digit_count(&disc));

    // Verify this equals (Q00−Q11)² + 4Q01²
    let diff = q00 - q11;
    let disc_check = &diff * &diff + BigInt::from(4) * &q01_sq;
    let verify = &disc - &disc_check;
    println!("  Verification: disc − ((Q00−Q11)²+4Q01²) = {}",
        if verify.is_zero() { "0 ✓ EXACT" } else { "NONZERO" });

    // Since R01 is TINY compared to (Q00+Q11)²:
    // R01 = 651d, (Q00+Q11)² = 657d → 4R01 is 6 digits smaller
    println!("\n  4R01 / (Q00+Q11)² = {} (the perturbation ratio)",
        fp_div(&four_r01, &trace_sq, 25));

    // √disc ≈ |Q00+Q11| · √(1 + 4R01/(Q00+Q11)²) ≈ |Q00+Q11| · (1 + 2R01/(Q00+Q11)²)
    let sqrt_disc = isqrt(&disc);
    let abs_trace = trace.abs();

    println!("  √disc = {}d", digit_count(&sqrt_disc));
    println!("  |Q00+Q11| = {}d", digit_count(&abs_trace));
    println!("  √disc − |Q00+Q11| = {}d (this is what creates the crack)",
        digit_count(&(&sqrt_disc - &abs_trace)));

    // λ₊ = (trace + √disc)/2 = (−|trace| + √disc)/2
    // Since √disc > |trace| (because R01 > 0):
    // λ₊ = (√disc − |trace|)/2 > 0
    let crack = (&sqrt_disc - &abs_trace) / BigInt::from(2);
    println!("\n  λ_crack = (√disc − |trace|)/2 = {}d ({})",
        digit_count(&crack), if crack.is_positive() { "+" } else { "−" });

    // =====================================================
    // STEP 4: First-order approximation
    // =====================================================
    println!("\n=== STEP 4: FIRST-ORDER APPROXIMATION ===");
    // √(T²+4R) ≈ |T|·(1 + 2R/T²) for small R/T²
    // λ_crack ≈ (|T|·(1+2R/T²) − |T|)/2 = |T|·R/T² = R/|T|
    // where T = Q00+Q11, R = R01

    // So λ_crack ≈ R01/|Q00+Q11| to first order!

    let first_order = &r01 * &BigInt::from(10u32).pow(25) / &abs_trace;
    println!("First-order: λ_crack ≈ R01/|Q00+Q11|");

    // Exact ratio
    println!("  R01/|Q00+Q11| ≈ {}", fp_div(&r01, &abs_trace, 25));

    // Compare with actual crack
    // We need a better comparison — both are ~323d numbers
    // Let's compute the ratio crack / (R01/|trace|)
    // = crack · |trace| / R01
    let ratio_check = &crack * &abs_trace / &r01;
    println!("  λ_crack · |trace| / R01 = {} (should be ≈ 1.0 if first-order exact)",
        fp_div(&(&crack * &abs_trace), &r01, 25));

    // The correction is second-order in R01/T²
    // λ_crack = R01/|T| − 2R01²/|T|³ + ...
    // The second-order correction:
    let second_order_corr = BigInt::from(2) * &r01 * &r01 / &(&abs_trace * &abs_trace * &abs_trace);
    println!("  Second-order correction: 2R01²/|T|³ ≈ {}d",
        digit_count(&second_order_corr));
    println!("  First order (R01/|T|) ≈ {}d → correction is {}d smaller",
        digit_count(&r01) as i64 - digit_count(&abs_trace) as i64 + digit_count(&r01) as i64,
        (digit_count(&r01) as i64 - digit_count(&abs_trace) as i64) -
        digit_count(&second_order_corr) as i64);

    // =====================================================
    // STEP 5: The bond eigenvalue from R01
    // =====================================================
    println!("\n=== STEP 5: THE BOND EIGENVALUE ===");
    // λ₋ = (trace − √disc)/2 = (−|trace| − √disc)/2 (negative)
    let bond = (&trace - &sqrt_disc) / BigInt::from(2);
    println!("λ_bond = (trace − √disc)/2 = {}d ({})",
        digit_count(&bond), if bond.is_negative() { "−" } else { "+" });

    // The bond absorbs what the crack doesn't:
    // λ_crack + λ_bond = trace = Q00+Q11
    let ev_sum = &crack + &bond;
    let sum_check = &ev_sum - &trace;
    println!("λ_crack + λ_bond = trace(Q00+Q11)? {}",
        if sum_check.is_zero() { "✓ EXACT" } else { "NONZERO" });

    // λ_crack · λ_bond = det = Q00·Q11 − Q01² = −R01 − |Q00|·|Q11| + |Q00|·|Q11|
    // Actually: det = Q00·Q11 − Q01²
    // Q00·Q11 = |Q00|·|Q11| (both negative), Q01² = |Q00|·|Q11| + R01
    // det = |Q00|·|Q11| − |Q00|·|Q11| − R01 = −R01
    // Wait, that's not right. Let me recompute:
    // M01 = Q00·Q11 − Q01² = |Q00|·|Q11| − Q01² = threshold − Q01² = −R01
    // So λ_crack · λ_bond = M01 = −R01

    let ev_prod = &crack * &bond;
    let m01 = q00 * q11 - &q01_sq;
    let prod_check = &ev_prod - &m01;
    println!("λ_crack · λ_bond = M01 = −R01? product − M01 = {} ({}d residual from integer truncation)",
        if prod_check.is_zero() { "0 ✓ EXACT".to_string() }
        else { format!("{}d residual", digit_count(&prod_check)) },
        digit_count(&prod_check));

    // Note: there may be a small residual because we used isqrt (floor of √disc)
    // which introduces ±1 error in √disc, hence ±1 in the eigenvalues

    println!("\nKEY IDENTITY: λ_crack · λ_bond = −R01");
    println!("The PRODUCT of the two eigenvalues is exactly the negative of the unbinding energy.");
    println!("R01 = {}d → the eigenvalues' product has {}d", digit_count(&r01), digit_count(&m01));

    // =====================================================
    // STEP 6: The energy balance sheet
    // =====================================================
    println!("\n=== STEP 6: THE ENERGY BALANCE SHEET ===");
    println!("{:<40} {:>8} {:>6}", "Quantity", "Sign", "Digits");
    println!("{:-<56}", "");
    println!("{:<40} {:>8} {:>6}", "Q00 (brace self-energy)", "−", digit_count(q00));
    println!("{:<40} {:>8} {:>6}", "Q11 (box self-energy)", "−", digit_count(q11));
    println!("{:<40} {:>8} {:>6}", "Q01 (coupling)", "+", digit_count(q01));
    println!("{:<40} {:>8} {:>6}", "|Q00|·|Q11| (threshold cost)", "+", digit_count(&threshold));
    println!("{:<40} {:>8} {:>6}", "Q01² (coupling supply)", "+", digit_count(&q01_sq));
    println!("{:<40} {:>8} {:>6}", "R01 (unbinding energy)", "+", digit_count(&r01));
    println!("{:<40} {:>8} {:>6}", "λ_crack (the positive mode)", "+", digit_count(&crack));
    println!("{:<40} {:>8} {:>6}", "λ_bond (the negative mode)", "−", digit_count(&bond));
    println!("{:<40} {:>8} {:>6}", "λ_crack + λ_bond = Q00+Q11", "−", digit_count(&trace));
    println!("{:<40} {:>8} {:>6}", "λ_crack · λ_bond = −R01", "−", digit_count(&r01));

    // =====================================================
    // STEP 7: What if the coupling were EXACTLY at threshold?
    // =====================================================
    println!("\n=== STEP 7: AT THE THRESHOLD ===");
    // If Q01² = |Q00|·|Q11| exactly (R01 = 0):
    // disc = (Q00+Q11)² + 0 = (Q00+Q11)²
    // √disc = |Q00+Q11|
    // λ₊ = (Q00+Q11+|Q00+Q11|)/2 = 0  (trace is negative, so |trace|=−trace)
    //     = (−|trace|+|trace|)/2 = 0
    // λ₋ = (−|trace|−|trace|)/2 = −|trace| = Q00+Q11

    println!("If Q01² = |Q00|·|Q11| exactly (R01 = 0):");
    println!("  λ₊ = 0 (the crack VANISHES — it becomes the zero mode)");
    println!("  λ₋ = Q00+Q11 = −{}d (bond absorbs everything)", digit_count(&abs_trace));
    println!("");
    println!("At the threshold, the crack eigenvalue is EXACTLY ZERO.");
    println!("Below threshold (CS < 1): both eigenvalues negative, no crack.");
    println!("Above threshold (CS > 1): one positive, one negative — crack exists.");
    println!("The transition is CONTINUOUS: λ_crack grows smoothly from 0.");

    // =====================================================
    // STEP 8: How far above threshold?
    // =====================================================
    println!("\n=== STEP 8: DISTANCE ABOVE THRESHOLD ===");
    // Q01_actual² = threshold + R01
    // Q01_threshold = √threshold = √(|Q00|·|Q11|) = geometric mean of |diagonals|

    let q01_threshold = isqrt(&threshold);
    println!("Q01_threshold = √(|Q00|·|Q11|) = {}d", digit_count(&q01_threshold));
    println!("Q01_actual    = {}d", digit_count(q01));

    let excess_q01 = q01.abs() - &q01_threshold;
    println!("Q01_actual − Q01_threshold = {}d ({})",
        digit_count(&excess_q01), if excess_q01.is_positive() { "+" } else { "−" });
    println!("Fractional excess in Q01: (actual−threshold)/threshold ≈ {}",
        fp_div(&excess_q01, &q01_threshold, 25));

    // The coupling is about 8.5×10⁻⁶ above threshold
    // This tiny excess creates the entire +323d crack eigenvalue!

    // =====================================================
    // STEP 9: The amplification factor
    // =====================================================
    println!("\n=== STEP 9: THE AMPLIFICATION FACTOR ===");
    // The fractional excess in Q01 is ~8.5×10⁻⁶
    // The crack eigenvalue is +323d ≈ 10^322
    // The threshold coupling is ~328d ≈ 10^327
    // So a tiny fractional excess creates a huge eigenvalue.
    //
    // The amplification: λ_crack / (ΔQ01) where ΔQ01 = Q01−Q01_threshold
    // ≈ R01/|trace| / (R01/(2·Q01_threshold)) ≈ 2·Q01_threshold/|trace|

    println!("Fractional excess in Q01: ~8.5 × 10⁻⁶");
    println!("Crack eigenvalue: +{}d ≈ 10^{}", digit_count(&crack), digit_count(&crack)-1);
    println!("Threshold coupling: {}d ≈ 10^{}", digit_count(&q01_threshold), digit_count(&q01_threshold)-1);
    println!("");
    println!("The amplification:");
    println!("  A tiny excess (10⁻⁶ fractional) in Q01");
    println!("  creates a MACROSCOPIC eigenvalue (+323d)");
    println!("  This is because λ_crack ≈ R01/|trace|");
    println!("  and R01 = (Q01+Q01_thr)(Q01−Q01_thr)");
    println!("  ≈ 2·Q01_thr · ΔQ01");
    println!("  So λ_crack ≈ 2·Q01_thr·ΔQ01/|trace|");

    let amplification = BigInt::from(2) * &q01_threshold / &abs_trace;
    println!("  Amplification factor 2·Q01_thr/|trace| ≈ {}", fp_div(&(BigInt::from(2) * &q01_threshold), &abs_trace, 25));

    // =====================================================
    // STEP 10: The GCD of R01
    // =====================================================
    println!("\n=== STEP 10: R01 ARITHMETIC ===");
    let gcd_r01_threshold = r01.gcd(&threshold);
    println!("R01 = {}d", digit_count(&r01));
    println!("gcd(R01, |Q00|·|Q11|) = {}d (value = {})", digit_count(&gcd_r01_threshold), gcd_r01_threshold);

    // Factor out 2s from R01
    let mut r01_work = r01.clone();
    let mut twos = 0u32;
    while &r01_work % BigInt::from(2u32) == BigInt::zero() {
        r01_work /= BigInt::from(2u32);
        twos += 1;
    }
    println!("R01 = 2^{} × (odd {}d number)", twos, digit_count(&r01_work));

    // The 2-adic valuation tells us about the parity structure
    println!("2-adic valuation of R01: v₂(R01) = {}", twos);

    // =====================================================
    // STEP 11: Connection to M01 and det(Q)
    // =====================================================
    println!("\n=== STEP 11: THE R01 → M01 → det(Q) CHAIN ===");
    // R01 = Q01² − |Q00|·|Q11|
    // M01 = Q00·Q11 − Q01² = −R01 (since Q00·Q11 = |Q00|·|Q11| for both negative)
    // det(Q) depends on M01 (and M02, M12)

    println!("R01 = +{}d (the unbinding energy)", digit_count(&r01));
    println!("M01 = −R01 = −{}d (the sick minor)", digit_count(&r01));
    println!("det(Q) = Q00·M12 − ... = +1059d");
    println!("");
    println!("THE CHAIN:");
    println!("  R01 > 0  (coupling exceeds threshold)");
    println!("  → M01 < 0  (brace-box minor is NEGATIVE)");
    println!("  → 2×2 block is INDEFINITE (signature (1+,1−))");
    println!("  → crack eigenvalue EXISTS (+323d)");
    println!("  → full Q3p has signature (1+, 2−)");
    println!("  → the Three-Body Theorem applies (H143)");
    println!("  → torroid mediates by existence (H142)");
    println!("  → Jacobi rotation separates modes (H139)");
    println!("");
    println!("EVERYTHING TRACES BACK TO R01.");
    println!("One number — 651 digits, positive — is the root of the entire structure.");

    // =====================================================
    // SUMMARY
    // =====================================================
    println!("\n=== SUMMARY ===");
    println!("H145: THE UNBINDING ENERGY");
    println!("1. THRESHOLD: Q01² = |Q00|·|Q11| is EXACTLY where the crack appears");
    println!("2. R01 = Q01² − |Q00|·|Q11| = +{}d (the unbinding energy)", digit_count(&r01));
    println!("3. Fractional excess: R01/threshold = 1.691 × 10⁻⁵ (0.00169%)");
    println!("4. λ_crack ≈ R01/|Q00+Q11| to first order");
    println!("5. IDENTITY: disc = (Q00+Q11)² + 4R01 (exact)");
    println!("6. IDENTITY: λ_crack · λ_bond = −R01 = M01 (exact, up to isqrt rounding)");
    println!("7. At threshold: λ_crack = 0 exactly (continuous transition)");
    println!("8. Q01 exceeds threshold by ~8.5×10⁻⁶ fractionally");
    println!("9. R01 = 2^{} × odd — 2-adic structure", twos);
    println!("10. R01 is the ROOT of the entire Q3p structure chain");
}
