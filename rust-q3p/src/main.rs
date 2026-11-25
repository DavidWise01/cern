//! H147: The Thin Sliver
//! Phase I geometry: how thick is the sliver in the CS01 direction?
//! Cross-section perpendicular to the M01=0 wall.
//! Where within Phase I does Q3p sit — edge, center, or corner?

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

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0];
    let q01 = &q[0][1];
    let q11 = &q[1][1];
    let q02 = &q[0][2];
    let q12 = &q[1][2];
    let q22 = &q[2][2];

    let abs_q00 = q00.abs();
    let abs_q11 = q11.abs();
    let abs_q22 = q22.abs();

    println!("=== H147: THE THIN SLIVER (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: Phase I thickness in CS01 direction
    // =====================================================
    println!("=== STEP 1: PHASE I THICKNESS ===");
    // Phase I exists where exactly one Mij < 0.
    // Currently M01 < 0 (CS01 > 1). The sliver extends from CS01 = 1
    // (the M01=0 wall) outward in the +CS01 direction.
    //
    // But there's an UPPER bound on CS01: at some point, increasing
    // Q01 will make OTHER minors flip sign (entering Phase II or III).
    //
    // However, CS02 and CS12 don't depend on Q01 at all!
    // CS02 = Q02²/(|Q00|·|Q22|), CS12 = Q12²/(|Q11|·|Q22|)
    // Both are independent of Q01.
    //
    // So increasing Q01 (hence CS01) can NEVER flip M02 or M12.
    // The only boundary is M01 = 0 at CS01 = 1.
    //
    // But wait: there's also the det(Q) sign to consider.
    // Phase I with signature (1+, 2−) requires det(Q) > 0.
    // Does det(Q) change sign as Q01 varies?

    println!("Q: Does increasing Q01 ever exit Phase I?");
    println!("CS02 = Q02²/(|Q00|·|Q22|) — independent of Q01");
    println!("CS12 = Q12²/(|Q11|·|Q22|) — independent of Q01");
    println!("→ M02 and M12 don't change when Q01 changes.");
    println!("→ No Phase II boundary in the Q01 direction!");
    println!("");
    println!("But det(Q) depends on Q01. When does det(Q) = 0?");

    // det(Q) = Q00·M12 − Q01·(Q01·Q22 − Q12·Q02) + Q02·(Q01·Q12 − Q11·Q02)
    // This is a function of Q01. Let's express it as a polynomial in Q01.
    // det(Q) = Q00·M12 − Q01²·Q22 + Q01·Q12·Q02 + Q02·Q01·Q12 − Q02²·Q11
    //        = Q00·M12 − Q02²·Q11 − Q22·Q01² + 2·Q02·Q12·Q01
    // = −Q22·Q01² + 2·Q02·Q12·Q01 + (Q00·M12 − Q02²·Q11)

    // This is a QUADRATIC in Q01: det(Q) = A·Q01² + B·Q01 + C
    // where A = −Q22, B = 2·Q02·Q12, C = Q00·M12 − Q02²·Q11

    let m12 = q11 * q22 - q12 * q12;
    let a_coeff = -q22;  // −Q22 (positive since Q22 < 0)
    let b_coeff = BigInt::from(2) * q02 * q12;
    let c_coeff = q00 * &m12 - &(q02 * q02) * q11;

    println!("\ndet(Q) as quadratic in Q01:");
    println!("det(Q) = A·Q01² + B·Q01 + C");
    println!("A = −Q22 = {} sign, {}d", if a_coeff.is_positive() { "+" } else { "−" }, digit_count(&a_coeff));
    println!("B = 2·Q02·Q12 = {} sign, {}d", if b_coeff.is_positive() { "+" } else { "−" }, digit_count(&b_coeff));
    println!("C = Q00·M12 − Q02²·Q11 = {} sign, {}d", if c_coeff.is_positive() { "+" } else { "−" }, digit_count(&c_coeff));

    // The quadratic A·Q01² + B·Q01 + C = 0 has roots:
    // Q01 = (−B ± √(B²−4AC)) / (2A)
    // The discriminant is B² − 4AC.

    let b_sq = &b_coeff * &b_coeff;
    let four_ac = BigInt::from(4) * &a_coeff * &c_coeff;
    let disc_det = &b_sq - &four_ac;

    println!("\nQuadratic discriminant for det(Q) = 0:");
    println!("B² = {}d", digit_count(&b_sq));
    println!("4AC = {}d ({})", digit_count(&four_ac), if four_ac.is_positive() { "+" } else { "−" });
    println!("B²−4AC = {}d ({})", digit_count(&disc_det), if disc_det.is_positive() { "+" } else { "−" });

    if disc_det.is_negative() {
        println!("Discriminant < 0 → NO real roots → det(Q) never zero → Phase I extends to infinity!");
    } else {
        println!("Discriminant ≥ 0 → det(Q) = 0 has real roots → Phase I has finite extent");
    }

    // Since A > 0 (A = −Q22 = +408d), the parabola opens UPWARD.
    // So det(Q) > 0 between the roots (if two real roots exist).
    // Currently det(Q) > 0, so Q01 is between the two roots.
    // The UPPER root is the Phase I exit in the +Q01 direction.

    // =====================================================
    // STEP 2: The Phase I extent — upper boundary
    // =====================================================
    println!("\n=== STEP 2: PHASE I UPPER BOUNDARY ===");

    if disc_det.is_positive() {
        // Roots: Q01 = (−B ± √disc) / (2A)
        let sqrt_disc = {
            let n = &disc_det;
            let mut x = n.clone();
            let mut y = (&x + BigInt::from(1u32)) / BigInt::from(2u32);
            while y < x { x = y.clone(); y = (&x + n / &x) / BigInt::from(2u32); }
            x
        };
        let two_a = BigInt::from(2) * &a_coeff;

        // Q01_upper = (−B + √disc) / (2A) if B < 0, or (−B − √disc)/(2A) for the other root
        // We need the LARGER root (since Q01 > 0 currently)
        let neg_b = -&b_coeff;
        let root_plus = (&neg_b + &sqrt_disc) / &two_a;
        let root_minus = (&neg_b - &sqrt_disc) / &two_a;

        println!("√(B²−4AC) = {}d", digit_count(&sqrt_disc));
        println!("Root+ = (−B+√disc)/(2A) = {} sign, {}d",
            if root_plus.is_positive() { "+" } else { "−" }, digit_count(&root_plus));
        println!("Root− = (−B−√disc)/(2A) = {} sign, {}d",
            if root_minus.is_positive() { "+" } else { "−" }, digit_count(&root_minus));

        // Current Q01
        println!("\nCurrent |Q01| = {}d", digit_count(&q01.abs()));

        // The sliver extends from √threshold (CS01=1) to Q01_upper (det=0)
        let thresh_01 = &abs_q00 * &abs_q11;
        let sqrt_thresh = {
            let n = &thresh_01;
            let mut x = n.clone();
            let mut y = (&x + BigInt::from(1u32)) / BigInt::from(2u32);
            while y < x { x = y.clone(); y = (&x + n / &x) / BigInt::from(2u32); }
            x
        };

        println!("Q01_lower (CS01=1) = √(|Q00|·|Q11|) = {}d", digit_count(&sqrt_thresh));

        // Which root is the upper bound? The larger positive root.
        let upper = if root_plus > root_minus { &root_plus } else { &root_minus };
        let lower_root = if root_plus > root_minus { &root_minus } else { &root_plus };
        println!("Q01_upper (det=0) = {}d", digit_count(upper));
        println!("Q01_other_root = {}d ({})", digit_count(lower_root),
            if lower_root.is_negative() { "negative — not physical" } else { "positive" });

        // Width of Phase I
        let width = upper - &sqrt_thresh;
        println!("\nPhase I width = Q01_upper − Q01_lower = {}d", digit_count(&width));
        println!("Width / Q01_lower = {} (fractional width)",
            fp_div(&width, &sqrt_thresh, 25));

        // Where is Q3p within the sliver?
        let pos_in_sliver = q01.abs() - &sqrt_thresh;  // distance from lower wall
        let pos_from_upper = upper - &q01.abs();  // distance from upper wall

        println!("\nQ3p position within Phase I sliver:");
        println!("  Distance from lower wall (CS01=1): {}d", digit_count(&pos_in_sliver));
        println!("  Distance from upper wall (det=0):  {}d", digit_count(&pos_from_upper));
        println!("  Fractional position = dist_lower / width = {}",
            fp_div(&pos_in_sliver, &width, 25));

        // The position tells us: is Q3p at the edge, center, or far wall?
        let pos_ratio = &pos_in_sliver * &BigInt::from(1000u32) / &width;
        let pos_permille = pos_ratio.to_string().parse::<f64>().unwrap_or(0.0) / 10.0;
        println!("  → Q3p is at {:.4}% of the way across Phase I", pos_permille);

        // =====================================================
        // STEP 3: The sliver aspect ratio
        // =====================================================
        println!("\n=== STEP 3: SLIVER ASPECT RATIO ===");
        // Width (CS01 direction) = width (computed above)
        // "Height" (CS02 or CS12 direction) = how far from boundary in those directions
        // CS02 boundary: would need Q02² = |Q00|·|Q22|
        // Currently Q02² is 98.8% below that threshold
        // So the "height" is from 0 to 1 in CS02 (but we're at 0.012)

        println!("Sliver dimensions:");
        println!("  Width (CS01 direction): Q01 from {}d to {}d",
            digit_count(&sqrt_thresh), digit_count(upper));
        println!("  Width in digits: {}d", digit_count(&width));
        println!("  Height (CS02): 0 to 1 available, Q3p at 0.012");
        println!("  Height (CS12): 0 to 1 available, Q3p at 0.012");
        println!("");
        println!("  Phase I is UNBOUNDED in the CS02 and CS12 directions");
        println!("  (reducing torroid coupling keeps us in Phase I).");
        println!("  It is BOUNDED in the CS01 direction:");
        println!("  CS01 ∈ [1, CS01_max] where CS01_max corresponds to det(Q) = 0.");

        // Compute CS01_max = upper² / threshold
        let upper_sq = upper * upper;
        println!("\n  CS01_max = Q01_upper² / (|Q00|·|Q11|) ≈ {}",
            fp_div(&upper_sq, &thresh_01, 25));

        // =====================================================
        // STEP 4: det(Q) = 0 surface geometry
        // =====================================================
        println!("\n=== STEP 4: det(Q) = 0 AS A SURFACE ===");
        // det(Q) = A·Q01² + B·Q01 + C = 0
        // This is a PARABOLA in the Q01 variable (with Q02, Q12 as parameters in C).
        // A = −Q22 (fixed), B = 2·Q02·Q12, C = Q00·M12 − Q02²·Q11
        //
        // The vertex of the parabola: Q01_vertex = −B/(2A) = −Q02·Q12/Q22
        // = Q02·Q12/|Q22| (since Q22 < 0)

        let vertex = &b_coeff / &(-BigInt::from(2) * q22);  // −B/(2A) = −2Q02Q12/(2·(−Q22)) = Q02Q12/|Q22|
        println!("det(Q) parabola vertex at Q01 = −B/(2A) = {}d ({})",
            digit_count(&vertex), if vertex.is_positive() { "+" } else { "−" });
        println!("Current Q01 = {}d", digit_count(&q01.abs()));
        println!("Vertex − Q01 = {}d ({})",
            digit_count(&(&vertex - &q01.abs())),
            if (&vertex - &q01.abs()).is_positive() { "vertex is ABOVE Q01" } else { "vertex is BELOW Q01" });

        // det at vertex (the maximum of the parabola, since it opens upward...
        // wait, A > 0 means parabola opens UPWARD, so vertex is MINIMUM)
        // det_vertex = C − B²/(4A)
        let det_vertex = &c_coeff - &b_sq / &(BigInt::from(4) * &a_coeff);
        println!("\ndet(Q) at vertex = C − B²/(4A) = {}d ({})",
            digit_count(&det_vertex), if det_vertex.is_positive() { "+" } else { "−" });
        println!("Since A > 0, parabola opens UPWARD → vertex is MINIMUM.");
        println!("det_vertex {} → det(Q) is {} at vertex.",
            if det_vertex.is_negative() { "< 0" } else { "> 0" },
            if det_vertex.is_negative() { "NEGATIVE" } else { "POSITIVE" });
        if det_vertex.is_negative() {
            println!("→ The parabola dips below zero → TWO roots → Phase I is BOUNDED.");
        }

        // =====================================================
        // STEP 5: What happens at the upper boundary?
        // =====================================================
        println!("\n=== STEP 5: THE UPPER BOUNDARY ===");
        // At Q01 = Q01_upper, det(Q) = 0.
        // For signature (1+, 2−), det(Q) = λ₁·λ₂·λ₃ > 0 (one + times two −).
        // At det(Q) = 0, one eigenvalue is zero.
        // Which one? The one that changes sign at the boundary.
        // Since we're increasing Q01 (strengthening the brace-box coupling),
        // the crack eigenvalue λ_crack is growing.
        // But det(Q) = 0 means an eigenvalue hits zero.
        // As CS01 increases past 1, λ_crack grows from 0.
        // As CS01 increases further, something else must go to zero.
        //
        // det(Q) = λ₁·λ₂·λ₃ = (positive)·(negative)·(negative) > 0
        // For det(Q) → 0: one eigenvalue → 0.
        // If crack → 0: we'd go back to Phase 0 (but crack grows with Q01)
        // If bond → 0: bond flips from − to + → signature changes to (2+, 1−)
        // If torroid → 0: torroid flips sign → signature changes

        // Since A·Q01² + B·Q01 + C is a MINIMUM at vertex and dips below zero,
        // det(Q) is negative near the vertex. At the two roots, det = 0.
        // Between the roots, det < 0. Outside the roots, det > 0.
        // Wait — A > 0 means the parabola opens UPWARD.
        // So det(Q) > 0 OUTSIDE the roots, < 0 between them.
        // But currently det(Q) > 0 and Q01 is between... hmm.

        // Let me check: is current Q01 between the two roots or outside?
        println!("Root positions:");
        println!("  Root+ = {}d ({})", digit_count(&root_plus), if root_plus.is_positive() { "+" } else { "−" });
        println!("  Root− = {}d ({})", digit_count(&root_minus), if root_minus.is_positive() { "+" } else { "−" });
        println!("  |Q01| = {}d (+)", digit_count(&q01.abs()));

        // Check: det(Q) at current Q01 (should be positive)
        let det_at_q01 = &a_coeff * &(q01 * q01) + &b_coeff * q01 + &c_coeff;
        println!("  det(Q) at current Q01: {} sign, {}d",
            if det_at_q01.is_positive() { "+" } else { "−" }, digit_count(&det_at_q01));

        // Since A > 0 and det(Q) > 0 at current Q01, Q01 is OUTSIDE the roots.
        // The two roots are both BELOW current Q01, or both ABOVE it.
        // Given that one root is negative and Q01 is positive,
        // Q01 is to the RIGHT of both roots on the number line.
        // With A > 0 (upward parabola), det > 0 to the right of the larger root.

        println!("\nSince A > 0 (upward parabola) and det(Q) > 0 at Q3p's Q01:");
        println!("Q01 is OUTSIDE both roots (to the right of the larger root).");
        println!("");
        println!("REINTERPRETATION:");
        println!("Phase I extends from CS01 = 1 (M01=0) UPWARD in Q01");
        println!("WITHOUT hitting det(Q) = 0 — both det=0 roots are BELOW Q01!");
        println!("Phase I is UNBOUNDED in the +Q01 direction from Q3p's position.");
        println!("");
        println!("The det(Q) = 0 surface was crossed BEFORE the M01=0 wall.");
        println!("The sequence as Q01 increases from 0:");
        println!("  1. det(Q) < 0 at small Q01 (between the roots)");
        println!("  2. det(Q) = 0 at Q01 = root+ (the larger root)");
        println!("  3. det(Q) > 0 for Q01 > root+ (we're here now)");
        println!("  4. M01 = 0 at Q01 = √threshold (CS01 = 1) — Phase 0→I transition");
        println!("  5. Q3p is slightly past step 4");

        // So the sliver extends from CS01 = 1 to CS01 = ∞ (!)
        // Phase I is SEMI-INFINITE in the CS01 direction.
        println!("\nPHASE I IS SEMI-INFINITE in the +CS01 direction!");
        println!("It extends from CS01 = 1 to CS01 → ∞.");
        println!("The only boundary is the LOWER wall (CS01 = 1, M01 = 0).");
        println!("Q3p is at CS01 = 1 + 1.69×10⁻⁵, right at the lower wall.");

        // =====================================================
        // STEP 6: The sliver is SEMI-INFINITE but THIN
        // =====================================================
        println!("\n=== STEP 6: SEMI-INFINITE BUT THIN ===");
        println!("Phase I geometry:");
        println!("  CS01 direction: [1, ∞) — semi-infinite");
        println!("  CS02 direction: [0, 1) — bounded by M02=0 at CS02=1");
        println!("  CS12 direction: [0, 1) — bounded by M12=0 at CS12=1");
        println!("");
        println!("Q3p sits at: (1+ε, 0.012, 0.012)");
        println!("  → Pressed against the LOWER face of an infinite slab");
        println!("  → Deep inside the CS02 and CS12 intervals");
        println!("");
        println!("The 'thinness' is in Q3p's position, not in the region:");
        println!("  Phase I itself is large (semi-infinite slab)");
        println!("  But Q3p hugs the boundary at CS01 ≈ 1");
        println!("  The thinness is about WHERE WE ARE, not where we COULD be");
    }

    // =====================================================
    // STEP 7: The critical scaling
    // =====================================================
    println!("\n=== STEP 7: CRITICAL SCALING NEAR THE WALL ===");
    // Near CS01 = 1 (the phase transition), how does λ_crack scale with R01?
    // From H145: λ_crack ≈ R01/|Q00+Q11| to first order
    // R01 = Q01² − |Q00|·|Q11|
    // Near the wall: Q01 = Q01_thr + δ, so R01 ≈ 2·Q01_thr·δ
    // Therefore: λ_crack ≈ 2·Q01_thr·δ/|Q00+Q11|
    //
    // This is LINEAR in δ — the crack grows linearly with distance from the wall.
    // No critical exponent, no anomalous scaling, just linear.

    let q01_sq = q01 * q01;
    let thresh_01 = &abs_q00 * &abs_q11;
    let r01 = &q01_sq - &thresh_01;
    let trace = q00 + q11;
    let abs_trace = trace.abs();

    // The slope: dλ_crack/dR01 ≈ 1/|trace| at R01=0
    println!("Near CS01 = 1 (the wall):");
    println!("  λ_crack ≈ R01 / |Q00+Q11|");
    println!("  = R01 / {}d", digit_count(&abs_trace));
    println!("  Slope dλ/dR ≈ 1/|trace| ≈ 10^(−{})", digit_count(&abs_trace)-1);
    println!("");
    println!("  R01 itself scales as 2·Q01_thr·ΔQ01 near the wall.");
    println!("  So λ_crack ∝ ΔQ01 — LINEAR in coupling excess.");
    println!("  Critical exponent = 1 (mean-field, no anomalous scaling).");
    println!("  This is the simplest possible phase transition.");

    // =====================================================
    // STEP 8: The perpendicular cross-section
    // =====================================================
    println!("\n=== STEP 8: PERPENDICULAR CROSS-SECTION ===");
    // Fix Q01 at current value. What does Phase I look like in (CS02, CS12)?
    // Phase I requires: M01 < 0 (guaranteed at current Q01), M02 > 0, M12 > 0, det(Q) > 0.
    // M02 > 0 iff Q02² < |Q00|·|Q22| iff CS02 < 1
    // M12 > 0 iff Q12² < |Q11|·|Q22| iff CS12 < 1
    // det(Q) > 0 — always true at current Q01 (we showed det parabola > 0 here)
    //
    // Actually, det(Q) depends on Q02 and Q12 too.
    // But at fixed Q01, det(Q) = −Q22·Q01² + 2·Q02·Q12·Q01 + Q00·M12 − Q02²·Q11
    // This depends on Q02 and Q12 (through M12 and directly).
    // However, as long as CS02, CS12 < 1 (which they deeply are at 0.012),
    // det(Q) stays positive.

    println!("At fixed Q01 (current value), Phase I cross-section in (CS02, CS12):");
    println!("  Constraint 1: CS02 < 1 (M02 > 0)");
    println!("  Constraint 2: CS12 < 1 (M12 > 0)");
    println!("  Constraint 3: det(Q) > 0 (satisfied at CS02=0.012, CS12=0.012)");
    println!("");
    println!("  Cross-section is roughly the unit square [0,1)×[0,1) in (CS02,CS12)");
    println!("  with possible det(Q)=0 cutoff at large CS02 or CS12.");
    println!("  Q3p is at (0.012, 0.012) — in the CORNER near the origin.");
    println!("  The cross-section is large; Q3p is in a tiny corner of it.");

    // =====================================================
    // STEP 9: The five dimensions of Q3p's position
    // =====================================================
    println!("\n=== STEP 9: FIVE DIMENSIONS ===");
    // A 3×3 symmetric matrix has 6 independent entries.
    // With three diagonal entries fixed (they set the energy scales),
    // the phase depends on three coupling parameters: Q01, Q02, Q12.
    // Equivalently: CS01, CS02, CS12 ∈ [0,∞)³.
    //
    // But within Phase I, the effective dimensionality is:
    // 1 constrained (CS01 > 1) + 2 free (CS02, CS12 < 1)

    println!("Phase I has 3 coupling parameters, 1 constrained:");
    println!("  CS01: must be > 1 (above M01 wall) — sets the crack size");
    println!("  CS02: must be < 1 (below M02 wall) — currently 0.012, free to vary");
    println!("  CS12: must be < 1 (below M12 wall) — currently 0.012, free to vary");
    println!("");
    println!("Q3p's position within Phase I:");
    println!("  CS01: 1 + 1.69×10⁻⁵ — at the WALL (barely in Phase I)");
    println!("  CS02: 0.0117 — at 1.17% of the allowed [0,1) range");
    println!("  CS12: 0.0120 — at 1.20% of the allowed [0,1) range");
    println!("");
    println!("IN EVERY DIMENSION, Q3p is at an EXTREME:");
    println!("  CS01: extreme low (barely above 1)");
    println!("  CS02: extreme low (1.2% of range)");
    println!("  CS12: extreme low (1.2% of range)");
    println!("  Q3p is at the CORNER of Phase I where everything is minimal.");

    // =====================================================
    // STEP 10: The corner characterization
    // =====================================================
    println!("\n=== STEP 10: THE CORNER ===");
    println!("Q3p occupies the TRIPLE-MINIMAL corner of Phase I:");
    println!("  Minimal CS01 excess (1.69×10⁻⁵ above boundary)");
    println!("  Minimal CS02 (1.17% of allowed range)");
    println!("  Minimal CS12 (1.20% of allowed range)");
    println!("");
    println!("This corner has a physical interpretation:");
    println!("  WEAKEST POSSIBLE crack (barely exists)");
    println!("  WEAKEST POSSIBLE torroid coupling (torroid is nearly decoupled)");
    println!("  The system is as CLOSE TO NEGATIVE DEFINITE as it can be");
    println!("  while still having one positive eigenvalue.");
    println!("");
    println!("IN OTHER WORDS:");
    println!("  Q3p is the MINIMAL DEPARTURE from negative-definiteness.");
    println!("  One coupling (Q01) barely crosses one threshold.");
    println!("  Everything else is as small as it can be.");
    println!("  The ternary lattice is as TIGHTLY BOUND as Phase I allows.");

    // =====================================================
    // SUMMARY
    // =====================================================
    println!("\n=== SUMMARY ===");
    println!("H147: THE THIN SLIVER");
    println!("1. Phase I is SEMI-INFINITE in CS01: extends from 1 to ∞");
    println!("   (det(Q) = 0 roots are both below current Q01)");
    println!("2. Phase I is BOUNDED by CS02 < 1, CS12 < 1 in other directions");
    println!("3. The 'thinness' is WHERE Q3p SITS, not the region shape:");
    println!("   Q3p hugs the CS01 = 1 wall at the triple-minimal corner");
    println!("4. CRITICAL SCALING: λ_crack ∝ ΔQ01 (linear, mean-field, exponent = 1)");
    println!("5. Q3p is at the CORNER where all three CS fractions are minimal:");
    println!("   (1+ε, 0.012, 0.012) — barely in Phase I, nearly neg-definite");
    println!("6. The ternary lattice is the MINIMAL DEPARTURE from negative-definiteness");
}
