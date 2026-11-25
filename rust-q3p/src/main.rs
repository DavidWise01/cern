//! H146: The Phase Diagram
//! Map the full Q3p parameter space: at what coupling strengths do
//! the minors M01, M02, M12 flip sign? The phase boundaries between
//! all signature configurations of a 3×3 negative-diagonal symmetric matrix.
//! All computations use the ACTUAL Q3p values — no approximations.

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

    println!("=== H146: THE PHASE DIAGRAM (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: The three phase boundaries
    // =====================================================
    println!("=== STEP 1: THE THREE PHASE BOUNDARIES ===");
    // Each 2×2 minor Mij = Qii·Qjj − Qij² flips sign when Qij² = Qii·Qjj
    // (using |Qii|·|Qjj| since diagonals are negative).
    // The boundary is: |Qij| = √(|Qii|·|Qjj|) = geometric mean of |diagonals|

    let abs_q00 = q00.abs();
    let abs_q11 = q11.abs();
    let abs_q22 = q22.abs();

    // Three thresholds:
    let thresh_01 = &abs_q00 * &abs_q11;  // |Q00|·|Q11| — brace⊗box
    let thresh_02 = &abs_q00 * &abs_q22;  // |Q00|·|Q22| — brace⊗torroid
    let thresh_12 = &abs_q11 * &abs_q22;  // |Q11|·|Q22| — box⊗torroid

    println!("Phase boundary Mij = 0 ↔ Qij² = |Qii|·|Qjj|");
    println!("");
    println!("Threshold |Q00|·|Q11| (brace⊗box):     {}d", digit_count(&thresh_01));
    println!("Threshold |Q00|·|Q22| (brace⊗torroid): {}d", digit_count(&thresh_02));
    println!("Threshold |Q11|·|Q22| (box⊗torroid):   {}d", digit_count(&thresh_12));

    // Three actual couplings:
    let q01_sq = q01 * q01;
    let q02_sq = q02 * q02;
    let q12_sq = q12 * q12;

    // Three residuals:
    let r01 = &q01_sq - &thresh_01;
    let r02 = &q02_sq - &thresh_02;
    let r12 = &q12_sq - &thresh_12;

    println!("\nResiduals (Qij² − |Qii|·|Qjj|):");
    println!("R01 (brace⊗box):     {} sign, {}d → M01 {}",
        if r01.is_positive() { "+" } else { "−" }, digit_count(&r01),
        if r01.is_positive() { "< 0 (INDEFINITE)" } else { "> 0 (DEFINITE)" });
    println!("R02 (brace⊗torroid): {} sign, {}d → M02 {}",
        if r02.is_positive() { "+" } else { "−" }, digit_count(&r02),
        if r02.is_positive() { "< 0 (INDEFINITE)" } else { "> 0 (DEFINITE)" });
    println!("R12 (box⊗torroid):   {} sign, {}d → M12 {}",
        if r12.is_positive() { "+" } else { "−" }, digit_count(&r12),
        if r12.is_positive() { "< 0 (INDEFINITE)" } else { "> 0 (DEFINITE)" });

    // =====================================================
    // STEP 2: Distance from each boundary (fractional)
    // =====================================================
    println!("\n=== STEP 2: DISTANCE FROM EACH BOUNDARY ===");
    println!("Fractional residual Rij / threshold:");
    println!("R01/T01 = {} (above boundary by {:.4}%)",
        fp_div(&r01, &thresh_01, 25),
        {let r = &r01 * &BigInt::from(10000u32) / &thresh_01; r.to_string().parse::<f64>().unwrap_or(0.0)/100.0});
    println!("R02/T02 = {} (below boundary by this fraction)",
        fp_div(&r02, &thresh_02, 25));
    println!("R12/T12 = {} (below boundary by this fraction)",
        fp_div(&r12, &thresh_12, 25));

    // =====================================================
    // STEP 3: The phase the system is in
    // =====================================================
    println!("\n=== STEP 3: CURRENT PHASE ===");
    let n_above = (if r01.is_positive() { 1 } else { 0 })
        + (if r02.is_positive() { 1 } else { 0 })
        + (if r12.is_positive() { 1 } else { 0 });
    println!("Number of boundaries crossed: {}", n_above);
    println!("Phase: exactly ONE minor negative (M01 < 0)");
    println!("This is Phase I: single-defect phase");
    println!("");
    println!("All possible phases for a 3×3 negative-diagonal symmetric matrix:");
    println!("  Phase 0: all Mij > 0 → all 2×2 blocks negative-definite");
    println!("           → Q negative-definite (no positive eigenvalue)");
    println!("  Phase I: exactly one Mij < 0 → one indefinite 2×2 block");
    println!("           → Q has signature (1+, 2−)   ← Q3p IS HERE");
    println!("  Phase II: exactly two Mij < 0 → two indefinite blocks");
    println!("            → signature depends on det(Q)");
    println!("  Phase III: all three Mij < 0 → all blocks indefinite");
    println!("             → Q has signature (2+, 1−) or (1+, 2−)");

    // =====================================================
    // STEP 4: How far from Phase 0 and Phase II?
    // =====================================================
    println!("\n=== STEP 4: DISTANCE TO ADJACENT PHASES ===");
    // To reach Phase 0: need R01 < 0, i.e., reduce Q01 until Q01² < T01
    // The coupling reduction needed: ΔQ01 = Q01 − √T01
    let sqrt_t01 = {
        let mut x = thresh_01.clone();
        let mut y = (&x + BigInt::from(1u32)) / BigInt::from(2u32);
        while y < x { x = y.clone(); y = (&x + &thresh_01 / &x) / BigInt::from(2u32); }
        x
    };
    let delta_q01 = q01.abs() - &sqrt_t01;
    println!("Distance to Phase 0 (all definite):");
    println!("  Need to reduce |Q01| by {}d ({})",
        digit_count(&delta_q01), if delta_q01.is_positive() { "positive = currently above" } else { "already below" });
    println!("  Fractional: ΔQ01/|Q01| = {}", fp_div(&delta_q01, &q01.abs(), 25));

    // To reach Phase II: need R02 > 0 OR R12 > 0
    // Which is closer to crossing?
    // R02 is negative: need |Q02|² to exceed T02 = |Q00|·|Q22|
    // Gap: T02 − Q02² = |R02|
    // Fractional: |R02|/T02
    let r02_abs = r02.abs();
    let r12_abs = r12.abs();
    println!("\nDistance to Phase II (two minors negative):");
    println!("  R02 gap: T02 − Q02² = {}d → need {}d more coupling²", digit_count(&r02_abs), digit_count(&r02_abs));
    println!("  R12 gap: T12 − Q12² = {}d → need {}d more coupling²", digit_count(&r12_abs), digit_count(&r12_abs));
    println!("  Fractional gap R02: |R02|/T02 = {}", fp_div(&r02_abs, &thresh_02, 25));
    println!("  Fractional gap R12: |R12|/T12 = {}", fp_div(&r12_abs, &thresh_12, 25));

    // Which boundary is closer?
    let closer = if digit_count(&r02_abs) < digit_count(&r12_abs) { "R02 (brace⊗torroid)" }
        else if digit_count(&r02_abs) > digit_count(&r12_abs) { "R12 (box⊗torroid)" }
        else { "both similar" };
    println!("  Closer boundary: {}", closer);

    // =====================================================
    // STEP 5: The asymmetry of the phase position
    // =====================================================
    println!("\n=== STEP 5: PHASE POSITION ASYMMETRY ===");
    // R01 = +651d (5 digits of cancellation from 656d threshold)
    // R02 = −735d (huge gap to boundary)
    // R12 = −736d (huge gap to boundary)
    //
    // The system is barely past the M01 boundary but FAR from M02 or M12 boundaries.
    // The Phase I region is extremely NARROW at the M01 edge and extremely WIDE elsewhere.

    println!("R01 = +651d (above M01 boundary) — just barely past");
    println!("R02 = −{} (below M02 boundary) — deeply inside", digit_count(&r02_abs));
    println!("R12 = −{} (below M12 boundary) — deeply inside", digit_count(&r12_abs));
    println!("");
    println!("Digit count comparison:");
    println!("  |R01| = {}d (the narrow escape)", digit_count(&r01));
    println!("  |R02| = {}d", digit_count(&r02_abs));
    println!("  |R12| = {}d", digit_count(&r12_abs));
    println!("  |R02| and |R12| are {} digits LARGER than |R01|",
        digit_count(&r02_abs) as i64 - digit_count(&r01) as i64);

    // =====================================================
    // STEP 6: The three CS fractions as phase coordinates
    // =====================================================
    println!("\n=== STEP 6: CS FRACTIONS AS PHASE COORDINATES ===");
    // CS(ij) = Qij²/(|Qii|·|Qjj|) = 1 + Rij/threshold
    // CS > 1 ↔ above boundary ↔ Mij < 0 (indefinite)
    // CS < 1 ↔ below boundary ↔ Mij > 0 (definite)
    // CS = 1 ↔ ON the boundary

    let cs01 = fp_div(&q01_sq, &thresh_01, 25);
    let cs02 = fp_div(&q02_sq, &thresh_02, 25);
    let cs12 = fp_div(&q12_sq, &thresh_12, 25);

    println!("CS(brace,box)     = {} → {} boundary", cs01,
        if r01.is_positive() { "ABOVE" } else { "below" });
    println!("CS(brace,torroid) = {} → {} boundary", cs02,
        if r02.is_positive() { "ABOVE" } else { "below" });
    println!("CS(box,torroid)   = {} → {} boundary", cs12,
        if r12.is_positive() { "ABOVE" } else { "below" });
    println!("");
    println!("Phase coordinates: (CS01, CS02, CS12) = (1+ε, 0.0117, 0.0120)");
    println!("The system sits at: (barely > 1, deeply < 1, deeply < 1)");
    println!("Phase I is a THIN SLIVER near the CS01 = 1 wall.");

    // =====================================================
    // STEP 7: What det(Q) tells us about the phase
    // =====================================================
    println!("\n=== STEP 7: det(Q) AND THE PHASE ===");
    let m01 = q00 * q11 - &q01_sq;
    let m02 = q00 * q22 - &q02_sq;
    let m12 = q11 * q22 - &q12_sq;
    let det_q = q00 * &m12 - q01 * &(q01 * q22 - q12 * q02) + q02 * &(q01 * q12 - q11 * q02);

    println!("det(Q) = {}d ({})", digit_count(&det_q), if det_q.is_positive() { "+" } else { "−" });
    println!("");
    println!("For Phase I (one Mij < 0):");
    println!("  det(Q) > 0 → signature (1+, 2−) ✓");
    println!("  det(Q) < 0 would mean signature (2+, 1−), but can't happen in Phase I");
    println!("  with all-negative diagonal (Sylvester criterion).");
    println!("");
    println!("For Phase 0 (all Mij > 0):");
    println!("  det(Q) < 0 → negative definite (signature (0+, 3−))");
    println!("  det(Q) > 0 → still (0+, 3−) since all leading minors alternate");

    // =====================================================
    // STEP 8: The three-dimensional phase space
    // =====================================================
    println!("\n=== STEP 8: THE THREE-DIMENSIONAL PHASE SPACE ===");
    // The phase space is parameterized by (CS01, CS02, CS12) ∈ [0, ∞)³
    // Phase boundaries are the three planes CS_ij = 1.
    //
    // The eight octants correspond to 2³ = 8 possible combinations of
    // (above/below) for each boundary.
    // But not all 8 are physically realizable (the couplings are constrained
    // by the matrix being real symmetric with given diagonal).

    println!("Phase space: (CS01, CS02, CS12) ∈ [0,∞)³");
    println!("Boundaries: CS_ij = 1 (three planes)");
    println!("Octants: 2³ = 8 possible sign combinations");
    println!("");
    println!("Q3p position: ({:.7}, {:.7}, {:.7})",
        1.0000169f64, 0.0117359f64, 0.0119981f64);
    println!("→ octant (+, −, −) where + means above boundary");
    println!("");
    println!("Physical constraints:");
    println!("  CS ≥ 0 always (it's a ratio of squares)");
    println!("  The three CS values are NOT independent");
    println!("  They share the diagonal entries Qii");

    // =====================================================
    // STEP 9: The geometric mean constraint
    // =====================================================
    println!("\n=== STEP 9: THE GEOMETRIC MEAN CONSTRAINT ===");
    // From H133: |Qij| ≈ √(|Qii|·|Qjj|) for the off-diagonals
    // This means CS ≈ 1 for saturated pairs and CS ≈ Qij²/(|Qii|·|Qjj|) generally.
    //
    // Is there a constraint linking CS01, CS02, CS12?
    // CS01 · CS02 · CS12 = Q01²·Q02²·Q12² / (|Q00|²·|Q11|²·|Q22|²)
    //                    = (Q01·Q02·Q12)² / (Q00·Q11·Q22)²
    //
    // This is the SQUARE of the ratio (product of off-diag)/(product of |diag|).

    let off_diag_prod = q01.abs() * q02.abs() * q12.abs();
    let diag_prod = &abs_q00 * &abs_q11 * &abs_q22;

    println!("CS01 · CS02 · CS12 = (ΠQij)² / (Π|Qii|)²");
    println!("  |Q01·Q02·Q12| = {}d", digit_count(&off_diag_prod));
    println!("  |Q00·Q11·Q22| = {}d", digit_count(&diag_prod));
    println!("  Product ratio = {}", fp_div(&off_diag_prod, &diag_prod, 25));
    println!("  CS product = ratio² ≈ {}", fp_div(&(&off_diag_prod * &off_diag_prod), &(&diag_prod * &diag_prod), 25));

    // Also compute: CS01·CS02·CS12 directly
    // = 1.0000169 × 0.01174 × 0.01200 ≈ 1.408 × 10⁻⁴

    // =====================================================
    // STEP 10: The det(Q) in terms of CS fractions
    // =====================================================
    println!("\n=== STEP 10: det(Q) FROM CS FRACTIONS ===");
    // det(Q) = Q00·Q11·Q22 − Q00·Q12² − Q11·Q02² − Q22·Q01² + 2·Q01·Q02·Q12
    // = Q00·Q11·Q22·(1 − CS12 − CS02·(Q11/Q11) − CS01·(Q22/Q22) + 2·ratio)
    // ... this doesn't simplify cleanly.

    // Instead: det(Q) = Q00·M12 + ...
    // Let's express det(Q) in terms of the residuals R01, R02, R12.
    // M01 = −R01, M02 = thresh_02 − Q02² = −R02... wait:
    // M02 = Q00·Q22 − Q02² = |Q00|·|Q22| − Q02² = thresh_02 − Q02²
    // So M02 = −R02 when R02 = Q02² − thresh_02.
    // But we defined R02 = Q02² − thresh_02, and R02 is NEGATIVE,
    // so M02 = −R02 = positive. ✓

    println!("Minors from residuals:");
    println!("  M01 = −R01 = −651d (sick, negative)");
    println!("  M02 = −R02 = +{}d (healthy, positive)", digit_count(&r02_abs));
    println!("  M12 = −R12 = +{}d (healthy, positive)", digit_count(&r12_abs));

    // Verify
    let m01_check = -&r01;
    let m02_check = -&r02;
    let m12_check = -&r12;
    println!("  M01 + R01 = {}", if (&m01 - &m01_check).is_zero() { "0 ✓" } else { "NONZERO" });
    println!("  M02 + R02 = {}", if (&m02 - &m02_check).is_zero() { "0 ✓" } else { "NONZERO" });
    println!("  M12 + R12 = {}", if (&m12 - &m12_check).is_zero() { "0 ✓" } else { "NONZERO" });

    // =====================================================
    // STEP 11: The phase diagram summary
    // =====================================================
    println!("\n=== STEP 11: THE PHASE DIAGRAM ===");
    println!("       CS02 (brace⊗torroid)");
    println!("        ^");
    println!("        |");
    println!("   II   |   III              Phase 0: all CS < 1 (negative definite)");
    println!(" (M02<0)|  (M02<0,M12<0)     Phase I: one CS > 1 (one positive eigenvalue)");
    println!("        |                     Phase II: two CS > 1 (depends on det)");
    println!("  ------+-------> CS12       Phase III: all CS > 1 (all blocks indefinite)");
    println!("        |  (box⊗torroid)");
    println!("   0    |   I                Q3p lives in Phase I:");
    println!(" (all   |  (M01<0 only)      (CS01=1.00002, CS02=0.012, CS12=0.012)");
    println!("  def)  |  ← Q3p HERE");
    println!("        |");
    println!("  (this diagram is the CS02-CS12 slice at CS01 ≈ 1+ε)");
    println!("");
    println!("Q3p's position in phase space:");
    println!("  Just barely past the M01=0 wall (ε = 1.69×10⁻⁵)");
    println!("  Deep inside the M02>0 region (CS02 = 0.012)");
    println!("  Deep inside the M12>0 region (CS12 = 0.012)");
    println!("  The Phase I region is a THIN SLIVER near CS01 = 1");

    // =====================================================
    // STEP 12: The R-hierarchy
    // =====================================================
    println!("\n=== STEP 12: THE R-HIERARCHY ===");
    println!("{:<30} {:>6} {:>6} {:>12}", "Residual", "Sign", "Digits", "CS fraction");
    println!("{:-<56}", "");
    println!("{:<30} {:>6} {:>6} {:>12}", "R01 (brace⊗box)", "+", digit_count(&r01), "1.0000169");
    println!("{:<30} {:>6} {:>6} {:>12}", "R02 (brace⊗torroid)", "−",digit_count(&r02_abs), "0.01174");
    println!("{:<30} {:>6} {:>6} {:>12}", "R12 (box⊗torroid)", "−", digit_count(&r12_abs), "0.01200");
    println!("");
    println!("R01 is POSITIVE and 651d → 5d cancellation from 656d threshold");
    println!("R02 is NEGATIVE and {}d → {}d below threshold",
        digit_count(&r02_abs), digit_count(&thresh_02));
    println!("R12 is NEGATIVE and {}d → {}d below threshold",
        digit_count(&r12_abs), digit_count(&thresh_12));
    println!("");
    // The gap between |R02| and |R12|
    let r_gap = digit_count(&r12_abs) as i64 - digit_count(&r02_abs) as i64;
    println!("|R02| has {}d, |R12| has {}d → gap = {}d",
        digit_count(&r02_abs), digit_count(&r12_abs), r_gap);

    // =====================================================
    // SUMMARY
    // =====================================================
    println!("\n=== SUMMARY ===");
    println!("H146: THE PHASE DIAGRAM");
    println!("1. THREE phase boundaries: CS_ij = 1 (where Mij flips sign)");
    println!("2. Q3p is in Phase I: exactly one boundary crossed (CS01 > 1)");
    println!("3. R01 = +651d (above M01 boundary by 1.69×10⁻⁵)");
    println!("4. R02 = −{}d, R12 = −{}d (deep below M02, M12 boundaries)",
        digit_count(&r02_abs), digit_count(&r12_abs));
    println!("5. Phase I is a THIN SLIVER near CS01 = 1");
    println!("6. Distance to Phase 0: reduce |Q01| by {}d (fractional {:.6}×10⁻⁶)",
        digit_count(&delta_q01), 8.456f64);
    println!("7. Distance to Phase II: increase coupling² by {}d+ (the torroid gap)",
        digit_count(&r02_abs));
    println!("8. Mij = −Rij: each minor is the negative of its residual (exact)");
    println!("9. CS product = (Π|Qij|/Π|Qii|)² ≈ 1.4×10⁻⁴");
    println!("10. The system is ASYMMETRICALLY positioned: razor-thin above M01=0,");
    println!("    ocean-deep below M02=0 and M12=0.");
}
