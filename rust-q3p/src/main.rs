//! H136: The Antiparallel Analysis
//! Exact perturbation theory: how much does the torroid shift each eigenvalue?
//!
//! Physics: The 3×3 Q3p matrix has a 2×2 brace-box block with eigenvalues
//! λ_bond and λ_crack, and a torroid entry Q22 on the diagonal.
//! The off-diagonal coupling [Q02, Q12] connects torroid to brace-box.
//!
//! Second-order perturbation theory:
//!   Δλ_i = |<v_i|torroid>|² / (λ_i - Q22) for each 2×2 eigenvector v_i
//!
//! H135 showed: torroid is 99.998% along bond, 0.002% along crack.
//! Now: with λ_bond < 0 and c²>0 and (λ_bond - Q22) < 0:
//!   the correction c²/(λ_bond - Q22) is NEGATIVE → DEEPENS the bond
//!
//! This instrument computes exact BigInt perturbation corrections.

use num_bigint::BigInt;
use num_traits::{Zero, One, Signed};
use std::str::FromStr;

/// The Q3p matrix entries (exact BigInt values from the frozen lattice)
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

/// Fixed-point division: (a * 10^prec) / b, returns the decimal string
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

/// Integer square root by Newton's method (floor)
fn integer_sqrt(n: &BigInt) -> BigInt {
    if n.is_zero() || *n == BigInt::one() {
        return n.clone();
    }
    let digits = n.magnitude().to_string().len();
    let mut x = BigInt::from(10u32).pow((digits / 2 + 1) as u32);

    loop {
        let x_next = (&x + n / &x) / BigInt::from(2);
        if x_next >= x {
            break;
        }
        x = x_next;
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

    println!("=== H136: THE ANTIPARALLEL ANALYSIS (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: Recompute 2×2 eigendecomposition from H135
    // =====================================================
    println!("=== STEP 1: 2×2 EIGENDECOMPOSITION (from H135) ===");

    let trace = q00 + q11;
    let det = q00 * q11 - q01 * q01;
    let diff = q00 - q11;
    let disc = &diff * &diff + BigInt::from(4) * q01 * q01;
    let sqrt_disc = integer_sqrt(&disc);
    let sqrt_error = &disc - &sqrt_disc * &sqrt_disc;

    // Eigenvalues: λ = (trace ± sqrt_disc) / 2
    // NOTE: "lambda_plus" = (trace+sqrt_disc)/2 is the ALGEBRAICALLY LARGER eigenvalue
    // For this indefinite matrix: lambda_plus is POSITIVE (smaller magnitude, 323d) = CRACK
    // lambda_minus is NEGATIVE (larger magnitude, 329d) = BOND
    let lambda_plus = (&trace + &sqrt_disc) / BigInt::from(2);  // crack eigenvalue
    let lambda_minus = (&trace - &sqrt_disc) / BigInt::from(2);  // bond eigenvalue

    println!("λ_crack = (trace+sqrt_disc)/2 = {} sign, {}d",
        if lambda_plus.is_negative() { "-" } else { "+" }, digit_count(&lambda_plus));
    println!("λ_bond  = (trace-sqrt_disc)/2 = {} sign, {}d",
        if lambda_minus.is_negative() { "-" } else { "+" }, digit_count(&lambda_minus));
    println!("Q22 (torroid self-energy) = {} sign, {}d",
        if q22.is_negative() { "-" } else { "+" }, digit_count(q22));
    println!("sqrt_disc residual: {}d (not perfect square)", digit_count(&sqrt_error));

    // Eigenvectors
    let two_q01 = BigInt::from(2) * q01;
    let q11_minus_q00 = q11 - q00;

    // crack eigenvector: [2*Q01, (Q11-Q00)+sqrt_disc]
    let v_crack_x = two_q01.clone();
    let v_crack_y = &q11_minus_q00 + &sqrt_disc;

    // bond eigenvector: [2*Q01, (Q11-Q00)-sqrt_disc]
    let v_bond_x = two_q01.clone();
    let v_bond_y = &q11_minus_q00 - &sqrt_disc;

    // Verify angles match H135
    let prec = 40u32;
    let tan_crack_str = fp_div(&v_crack_y, &v_crack_x, prec);
    let tan_bond_str = fp_div(&v_bond_y, &v_bond_x, prec);
    let tan_crack_f = tan_crack_str.parse::<f64>().unwrap_or(0.0);
    let tan_bond_f = tan_bond_str.parse::<f64>().unwrap_or(0.0);
    let theta_crack = tan_crack_f.atan() * 180.0 / std::f64::consts::PI;
    let theta_bond = tan_bond_f.atan() * 180.0 / std::f64::consts::PI;

    println!("\nCrack direction: {:.4}° from brace axis (H135: +29.3473°)", theta_crack);
    println!("Bond direction:  {:.4}° from brace axis (H135: -60.6527°)", theta_bond);
    println!("Orthogonality: |θ_crack − θ_bond| = {:.4}° (should be 90.0000°)",
        (theta_crack - theta_bond).abs());

    // =====================================================
    // STEP 2: Torroid coupling projections (from H135)
    // =====================================================
    println!("\n=== STEP 2: TORROID COUPLING PROJECTIONS ===");

    // c_bond = torroid · v_bond = Q02 * v_bond_x + Q12 * v_bond_y
    let c_bond = q02 * &v_bond_x + q12 * &v_bond_y;
    // c_crack = torroid · v_crack = Q02 * v_crack_x + Q12 * v_crack_y
    let c_crack = q02 * &v_crack_x + q12 * &v_crack_y;

    println!("c_bond  = torroid·v_bond  = {} sign, {}d",
        if c_bond.is_negative() { "-" } else { "+" }, digit_count(&c_bond));
    println!("c_crack = torroid·v_crack = {} sign, {}d",
        if c_crack.is_negative() { "-" } else { "+" }, digit_count(&c_crack));

    // Squared projections
    let c_bond_sq = &c_bond * &c_bond;
    let c_crack_sq = &c_crack * &c_crack;
    println!("\nc_bond²  = {}d (always positive)", digit_count(&c_bond_sq));
    println!("c_crack² = {}d (always positive)", digit_count(&c_crack_sq));

    // Norms of eigenvectors
    let norm_bond_sq = &v_bond_x * &v_bond_x + &v_bond_y * &v_bond_y;
    let norm_crack_sq = &v_crack_x * &v_crack_x + &v_crack_y * &v_crack_y;

    // Normalized c²: c²/|v|²
    // For comparison: (c_bond²/norm_bond²) vs (c_crack²/norm_crack²)
    let c_bond_norm = &c_bond_sq * &norm_crack_sq;
    let c_crack_norm = &c_crack_sq * &norm_bond_sq;
    let c_total = &c_bond_norm + &c_crack_norm;
    let pct_bond = fp_div(&c_bond_norm, &c_total, 20);
    let pct_crack = fp_div(&c_crack_norm, &c_total, 20);
    println!("\nNormalized coupling: {:.6}% bond / {:.6}% crack (H135 confirmation)",
        pct_bond.parse::<f64>().unwrap_or(0.0) * 100.0,
        pct_crack.parse::<f64>().unwrap_or(0.0) * 100.0);

    // =====================================================
    // STEP 3: PERTURBATION DENOMINATORS
    // =====================================================
    println!("\n=== STEP 3: PERTURBATION DENOMINATORS ===");
    println!("Second-order perturbation: Δλ_i = c_i² / (|v_i|² × (λ_i − Q22))");
    println!("where c_i = torroid·v_i (unnormalized dot product)\n");

    // Denominators: λ_i - Q22
    let denom_crack = &lambda_plus - q22;   // λ_crack - Q22
    let denom_bond = &lambda_minus - q22;    // λ_bond - Q22

    println!("λ_crack − Q22:");
    println!("  λ_crack = {} sign, {}d", if lambda_plus.is_negative() { "-" } else { "+" }, digit_count(&lambda_plus));
    println!("  Q22     = {} sign, {}d", if q22.is_negative() { "-" } else { "+" }, digit_count(q22));
    println!("  λ_crack − Q22 = {} sign, {}d",
        if denom_crack.is_negative() { "-" } else { "+" }, digit_count(&denom_crack));

    println!("\nλ_bond − Q22:");
    println!("  λ_bond  = {} sign, {}d", if lambda_minus.is_negative() { "-" } else { "+" }, digit_count(&lambda_minus));
    println!("  Q22     = {} sign, {}d", if q22.is_negative() { "-" } else { "+" }, digit_count(q22));
    println!("  λ_bond − Q22 = {} sign, {}d",
        if denom_bond.is_negative() { "-" } else { "+" }, digit_count(&denom_bond));

    // Physical interpretation of signs
    println!("\n--- SIGN ANALYSIS ---");
    println!("λ_crack (+, 323d) − Q22 (−, 408d) = POSITIVE ({}d)", digit_count(&denom_crack));
    println!("  → (+)−(−) = (+) → denominator is POSITIVE");
    println!("  → c_crack² (always +) / POSITIVE = POSITIVE correction");
    println!("  → torroid RAISES the crack eigenvalue (makes it more positive)");

    println!("\nλ_bond (−, 329d) − Q22 (−, 408d):", );
    if denom_bond.is_positive() {
        println!("  = POSITIVE ({}d)", digit_count(&denom_bond));
        println!("  → (−)−(−): Q22 is more negative than λ_bond");
        println!("  → c_bond² / POSITIVE = POSITIVE correction");
        println!("  → torroid RAISES the bond eigenvalue (makes it less negative)");
        println!("  *** SURPRISE: torroid WEAKENS the bond, not deepens it! ***");
    } else {
        println!("  = NEGATIVE ({}d)", digit_count(&denom_bond));
        println!("  → (−)−(−): Q22 is less negative than λ_bond");
        println!("  → c_bond² / NEGATIVE = NEGATIVE correction");
        println!("  → torroid DEEPENS the bond (makes it more negative)");
    }

    // =====================================================
    // STEP 4: EXACT PERTURBATION CORRECTIONS
    // =====================================================
    println!("\n=== STEP 4: EXACT PERTURBATION CORRECTIONS ===");
    println!("Δλ_i = c_i² / (|v_i|² × (λ_i − Q22))");
    println!("Using unnormalized: Δλ_i = (torroid·v_i)² / (|v_i|² × (λ_i − Q22))\n");

    // Full denominator for each correction
    let full_denom_crack = &norm_crack_sq * &denom_crack;
    let full_denom_bond = &norm_bond_sq * &denom_bond;

    println!("Crack correction:");
    println!("  numerator   = c_crack² = {}d", digit_count(&c_crack_sq));
    println!("  denominator = |v_crack|² × (λ_crack − Q22) = {}d, sign {}",
        digit_count(&full_denom_crack),
        if full_denom_crack.is_negative() { "-" } else { "+" });

    // Compute Δλ_crack as a rational approximation
    // We want c_crack_sq / full_denom_crack
    let delta_crack_str = fp_div(&c_crack_sq, &full_denom_crack, 40);
    println!("  Δλ_crack = {}", &delta_crack_str[..60.min(delta_crack_str.len())]);
    println!("  Δλ_crack sign: {}", if full_denom_crack.is_negative() {
        "NEGATIVE (c²>0 / neg = neg)" } else { "POSITIVE (c²>0 / pos = pos)" });

    println!("\nBond correction:");
    println!("  numerator   = c_bond² = {}d", digit_count(&c_bond_sq));
    println!("  denominator = |v_bond|² × (λ_bond − Q22) = {}d, sign {}",
        digit_count(&full_denom_bond),
        if full_denom_bond.is_negative() { "-" } else { "+" });

    let delta_bond_str = fp_div(&c_bond_sq, &full_denom_bond, 40);
    println!("  Δλ_bond = {}", &delta_bond_str[..60.min(delta_bond_str.len())]);
    println!("  Δλ_bond sign: {}", if full_denom_bond.is_negative() {
        "NEGATIVE (c²>0 / neg = neg) → DEEPENS bond" } else {
        "POSITIVE (c²>0 / pos = pos) → WEAKENS bond" });

    // =====================================================
    // STEP 5: MAGNITUDE COMPARISON
    // =====================================================
    println!("\n=== STEP 5: MAGNITUDE COMPARISON ===");

    // |Δλ_bond| / |Δλ_crack| = (c_bond² / |full_denom_bond|) / (c_crack² / |full_denom_crack|)
    //                         = (c_bond² × |full_denom_crack|) / (c_crack² × |full_denom_bond|)
    let ratio_num = &c_bond_sq * &full_denom_crack.abs();
    let ratio_den = &c_crack_sq * &full_denom_bond.abs();
    let ratio_str = fp_div(&ratio_num, &ratio_den, 20);
    println!("|Δλ_bond| / |Δλ_crack| = {}", &ratio_str[..40.min(ratio_str.len())]);

    let ratio_f = ratio_str.parse::<f64>().unwrap_or(0.0);
    println!("Bond correction is {:.1}× larger than crack correction", ratio_f);

    // Relative to eigenvalues themselves:
    // |Δλ_crack| / |λ_crack| and |Δλ_bond| / |λ_bond|
    // |Δλ_crack|/|λ_crack| = c_crack_sq / (|full_denom_crack| × ... nah, easier with fp_div
    // Actually: Δλ_crack/λ_crack = c_crack²/(norm_crack²×denom_crack×λ_crack)
    // But we can use the decimal strings
    let delta_crack_f = delta_crack_str.parse::<f64>().unwrap_or(0.0);
    let delta_bond_f = delta_bond_str.parse::<f64>().unwrap_or(0.0);

    // We need λ values as f64 too
    let lam_crack_str = fp_div(&lambda_plus, &BigInt::one(), 20);
    let lam_bond_str = fp_div(&lambda_minus, &BigInt::one(), 20);

    // These will overflow f64, so let's compute ratios via BigInt
    // |Δλ_crack|/|λ_crack| = c_crack²/(|norm_crack²×denom_crack|) / |λ_crack|
    //                       = c_crack² / (|norm_crack²×denom_crack×λ_crack|)
    let rel_crack_denom = (&full_denom_crack * &lambda_plus).abs();
    let rel_crack_str = fp_div(&c_crack_sq, &rel_crack_denom, 20);
    println!("\n|Δλ_crack| / |λ_crack| = {}", &rel_crack_str[..30.min(rel_crack_str.len())]);

    let rel_bond_denom = (&full_denom_bond * &lambda_minus).abs();
    let rel_bond_str = fp_div(&c_bond_sq, &rel_bond_denom, 20);
    println!("|Δλ_bond|  / |λ_bond|  = {}", &rel_bond_str[..30.min(rel_bond_str.len())]);

    // =====================================================
    // STEP 6: CORRECTED EIGENVALUES
    // =====================================================
    println!("\n=== STEP 6: CORRECTED EIGENVALUES ===");
    println!("λ_crack_corrected = λ_crack + Δλ_crack");
    println!("λ_bond_corrected  = λ_bond  + Δλ_bond\n");

    // We can't add f64 to 323-digit BigInt directly.
    // Instead: Δλ_i = c_i² / (norm_i² × denom_i)
    // As a fraction: Δλ_i_num / Δλ_i_den = c_i² / (norm_i² × denom_i)
    // λ_corrected = λ_i + c_i²/(norm_i² × denom_i)
    //             = (λ_i × norm_i² × denom_i + c_i²) / (norm_i² × denom_i)

    let corrected_crack_num = &lambda_plus * &full_denom_crack + &c_crack_sq;
    let corrected_crack_den = full_denom_crack.clone();
    println!("λ_crack_corrected = (λ_crack×|v|²×Δ + c²) / (|v|²×Δ)");
    println!("  numerator:   {} sign, {}d",
        if corrected_crack_num.is_negative() { "-" } else { "+" }, digit_count(&corrected_crack_num));
    println!("  denominator: {} sign, {}d",
        if corrected_crack_den.is_negative() { "-" } else { "+" }, digit_count(&corrected_crack_den));

    let corrected_bond_num = &lambda_minus * &full_denom_bond + &c_bond_sq;
    let corrected_bond_den = full_denom_bond.clone();
    println!("\nλ_bond_corrected = (λ_bond×|v|²×Δ + c²) / (|v|²×Δ)");
    println!("  numerator:   {} sign, {}d",
        if corrected_bond_num.is_negative() { "-" } else { "+" }, digit_count(&corrected_bond_num));
    println!("  denominator: {} sign, {}d",
        if corrected_bond_den.is_negative() { "-" } else { "+" }, digit_count(&corrected_bond_den));

    // =====================================================
    // STEP 7: CORRECTED EIGENVALUE RATIO
    // =====================================================
    println!("\n=== STEP 7: CORRECTED EIGENVALUE RATIO ===");

    // Original ratio: |λ_crack| / |λ_bond| ≈ 3.09×10⁻⁶ (from H134/H135)
    let orig_ratio = fp_div(&lambda_plus.abs(), &lambda_minus.abs(), 20);
    println!("ORIGINAL: |λ_crack|/|λ_bond| = {}", &orig_ratio[..30.min(orig_ratio.len())]);

    // Corrected ratio: |λ_crack_corrected| / |λ_bond_corrected|
    // = |corrected_crack_num/corrected_crack_den| / |corrected_bond_num/corrected_bond_den|
    // = |corrected_crack_num × corrected_bond_den| / |corrected_bond_num × corrected_crack_den|
    let corr_ratio_num = (&corrected_crack_num * &corrected_bond_den).abs();
    let corr_ratio_den = (&corrected_bond_num * &corrected_crack_den).abs();
    let corr_ratio = fp_div(&corr_ratio_num, &corr_ratio_den, 20);
    println!("CORRECTED: |λ_crack_corr|/|λ_bond_corr| = {}", &corr_ratio[..30.min(corr_ratio.len())]);

    let orig_f = orig_ratio.parse::<f64>().unwrap_or(0.0);
    let corr_f = corr_ratio.parse::<f64>().unwrap_or(0.0);
    if corr_f > 0.0 && orig_f > 0.0 {
        let change_pct = (corr_f - orig_f) / orig_f * 100.0;
        println!("Change in ratio: {:.6}%", change_pct);
        if change_pct.abs() < 0.001 {
            println!("→ NEGLIGIBLE: torroid barely changes the eigenvalue ratio");
        } else if change_pct > 0.0 {
            println!("→ Ratio INCREASED: gap between eigenvalues widened");
        } else {
            println!("→ Ratio DECREASED: gap between eigenvalues narrowed");
        }
    }

    // =====================================================
    // STEP 8: SCHUR COMPLEMENT (EXACT)
    // =====================================================
    println!("\n=== STEP 8: SCHUR COMPLEMENT ===");
    println!("The 'effective 2×2' after eliminating the torroid degree of freedom.");
    println!("Schur(2×2 | torroid) = M_bb − col × Q22⁻¹ × row");
    println!("where col = [Q02, Q12]^T, row = [Q02, Q12], M_bb = [[Q00,Q01],[Q01,Q11]]\n");

    // Schur complement of Q22 in the full 3×3:
    // S[i][j] = Q[i][j] - Q[i][2]*Q[j][2]/Q22  for i,j in {0,1}
    // As exact fraction: S[i][j] = (Q[i][j]*Q22 - Q[i][2]*Q[j][2]) / Q22

    let s00_num = q00 * q22 - q02 * q02;
    let s01_num = q01 * q22 - q02 * q12;
    let s11_num = q11 * q22 - q12 * q12;
    // All divided by Q22 (which is negative)

    println!("S00_num = Q00×Q22 − Q02² = {} sign, {}d",
        if s00_num.is_negative() { "-" } else { "+" }, digit_count(&s00_num));
    println!("S01_num = Q01×Q22 − Q02×Q12 = {} sign, {}d",
        if s01_num.is_negative() { "-" } else { "+" }, digit_count(&s01_num));
    println!("S11_num = Q11×Q22 − Q12² = {} sign, {}d",
        if s11_num.is_negative() { "-" } else { "+" }, digit_count(&s11_num));
    println!("All divided by Q22 ({} sign, {}d)", if q22.is_negative() { "-" } else { "+" }, digit_count(q22));

    // Effective eigenvalues of Schur complement:
    let s_trace_num = &s00_num + &s11_num;
    let s_det_num = &s00_num * &s11_num - &s01_num * &s01_num;
    // s_trace = s_trace_num / Q22
    // s_det = s_det_num / Q22²

    println!("\nSchur trace_num = {} sign, {}d",
        if s_trace_num.is_negative() { "-" } else { "+" }, digit_count(&s_trace_num));
    println!("Schur det_num = {} sign, {}d",
        if s_det_num.is_negative() { "-" } else { "+" }, digit_count(&s_det_num));

    // Discriminant of Schur complement (times Q22²):
    // disc_S_num = (S00_num - S11_num)² + 4×S01_num²
    let s_diff = &s00_num - &s11_num;
    let s_disc_num = &s_diff * &s_diff + BigInt::from(4) * &s01_num * &s01_num;
    println!("Schur disc_num = {} sign, {}d",
        if s_disc_num.is_negative() { "-" } else { "+" }, digit_count(&s_disc_num));

    let s_sqrt_disc = integer_sqrt(&s_disc_num);
    let s_sqrt_err = &s_disc_num - &s_sqrt_disc * &s_sqrt_disc;
    println!("sqrt(Schur disc_num) = {}d, residual = {}d",
        digit_count(&s_sqrt_disc), digit_count(&s_sqrt_err));

    // Schur eigenvalues (times Q22): λ_S × Q22 = (trace_num ± sqrt(disc_num)) / 2
    let schur_lam_plus_times_q22 = (&s_trace_num + &s_sqrt_disc) / BigInt::from(2);
    let schur_lam_minus_times_q22 = (&s_trace_num - &s_sqrt_disc) / BigInt::from(2);
    // Actual Schur eigenvalues = above / Q22

    println!("\nSchur eigenvalues (each × Q22, before dividing):");
    println!("  λ_S+ × Q22 = {} sign, {}d",
        if schur_lam_plus_times_q22.is_negative() { "-" } else { "+" },
        digit_count(&schur_lam_plus_times_q22));
    println!("  λ_S− × Q22 = {} sign, {}d",
        if schur_lam_minus_times_q22.is_negative() { "-" } else { "+" },
        digit_count(&schur_lam_minus_times_q22));
    println!("  Divide each by Q22 ({}) to get actual Schur eigenvalues",
        if q22.is_negative() { "negative" } else { "positive" });
    println!("  Sign of λ_S+: {} / {} = {}",
        if schur_lam_plus_times_q22.is_negative() { "-" } else { "+" },
        if q22.is_negative() { "-" } else { "+" },
        if (schur_lam_plus_times_q22.is_negative()) == (q22.is_negative()) { "POSITIVE" } else { "NEGATIVE" });
    println!("  Sign of λ_S−: {} / {} = {}",
        if schur_lam_minus_times_q22.is_negative() { "-" } else { "+" },
        if q22.is_negative() { "-" } else { "+" },
        if (schur_lam_minus_times_q22.is_negative()) == (q22.is_negative()) { "POSITIVE" } else { "NEGATIVE" });

    // Compare Schur eigenvalues to corrected eigenvalues
    // Schur eigenvalue ratio
    let schur_ratio = fp_div(&schur_lam_plus_times_q22.abs(), &schur_lam_minus_times_q22.abs(), 20);
    println!("\nSchur eigenvalue ratio |λ_S+|/|λ_S−| = {}", &schur_ratio[..30.min(schur_ratio.len())]);
    println!("(should match corrected ratio to second order)");

    // =====================================================
    // STEP 9: PHYSICAL INTERPRETATION
    // =====================================================
    println!("\n=== STEP 9: PHYSICAL INTERPRETATION ===");

    let rel_crack_f = rel_crack_str.parse::<f64>().unwrap_or(0.0);
    let rel_bond_f = rel_bond_str.parse::<f64>().unwrap_or(0.0);

    println!("The torroid is ANTIPARALLEL to the bond eigenvector (179.73°).");
    println!("It couples 99.998% along bond, 0.002% along crack.\n");

    println!("PERTURBATION CORRECTIONS:");
    if denom_crack.is_positive() {
        println!("  Crack: Δλ_crack is POSITIVE → torroid RAISES crack eigenvalue");
    } else {
        println!("  Crack: Δλ_crack is NEGATIVE → torroid LOWERS crack eigenvalue");
    }
    if denom_bond.is_positive() {
        println!("  Bond:  Δλ_bond is POSITIVE → torroid WEAKENS bond (makes less negative)");
        println!("\n  *** KEY FINDING: THE TORROID DOES NOT DEEPEN THE BOND ***");
        println!("  The initial hypothesis was: c²/λ with λ<0 → negative correction → deepens.");
        println!("  But the denominator is (λ_bond − Q22), and |Q22| > |λ_bond|.");
        println!("  So λ_bond − Q22 = (−329d) − (−408d) = +(408−329)d = POSITIVE.");
        println!("  Therefore c²/POSITIVE = POSITIVE → the bond gets LESS negative.");
        println!("  The torroid WEAKENS the bond (by a tiny amount).\n");
        println!("  PHYSICAL PICTURE:");
        println!("  Q22 (torroid self-energy) is MORE negative than λ_bond.");
        println!("  The torroid is a deeper well than the bond eigenvalue.");
        println!("  Coupling to a deeper well pulls eigenvalues TOWARD that well.");
        println!("  For the bond (above Q22): pulled down → toward Q22 → more negative?");
        println!("  NO — level repulsion: the bond eigenvalue is REPELLED from Q22.");
        println!("  Since Q22 is below λ_bond, repulsion pushes λ_bond UP.");
    } else {
        println!("  Bond:  Δλ_bond is NEGATIVE → torroid DEEPENS bond (makes more negative)");
        println!("\n  CONFIRMS HYPOTHESIS: torroid deepens the bond.");
        println!("  λ_bond − Q22 < 0, so |Q22| < |λ_bond| and c²/negative → negative correction.");
    }

    println!("\n--- RELATIVE SIZES ---");
    println!("|Δλ_crack|/|λ_crack| ≈ {}", &rel_crack_str[..20.min(rel_crack_str.len())]);
    println!("|Δλ_bond|/|λ_bond|   ≈ {}", &rel_bond_str[..20.min(rel_bond_str.len())]);
    println!("Bond correction is {:.1}× larger than crack correction in absolute terms", ratio_f);

    // =====================================================
    // STEP 10: DIGIT COUNT SUMMARY
    // =====================================================
    println!("\n=== STEP 10: DIGIT COUNT SUMMARY ===");
    println!("λ_crack:            {} sign, {}d", if lambda_plus.is_negative() { "-" } else { "+" }, digit_count(&lambda_plus));
    println!("λ_bond:             {} sign, {}d", if lambda_minus.is_negative() { "-" } else { "+" }, digit_count(&lambda_minus));
    println!("Q22:                {} sign, {}d", if q22.is_negative() { "-" } else { "+" }, digit_count(q22));
    println!("c_bond:             {} sign, {}d", if c_bond.is_negative() { "-" } else { "+" }, digit_count(&c_bond));
    println!("c_crack:            {} sign, {}d", if c_crack.is_negative() { "-" } else { "+" }, digit_count(&c_crack));
    println!("c_bond²:            {}d", digit_count(&c_bond_sq));
    println!("c_crack²:           {}d", digit_count(&c_crack_sq));
    println!("λ_crack − Q22:      {} sign, {}d", if denom_crack.is_negative() { "-" } else { "+" }, digit_count(&denom_crack));
    println!("λ_bond − Q22:       {} sign, {}d", if denom_bond.is_negative() { "-" } else { "+" }, digit_count(&denom_bond));
    println!("Schur S00_num:      {} sign, {}d", if s00_num.is_negative() { "-" } else { "+" }, digit_count(&s00_num));
    println!("Schur S01_num:      {} sign, {}d", if s01_num.is_negative() { "-" } else { "+" }, digit_count(&s01_num));
    println!("Schur S11_num:      {} sign, {}d", if s11_num.is_negative() { "-" } else { "+" }, digit_count(&s11_num));
    println!("sqrt_disc residual: {}d", digit_count(&sqrt_error));

    println!("\n=== SUMMARY ===");
    println!("H136: THE ANTIPARALLEL ANALYSIS");
    println!("1. λ_crack = +, {}d | λ_bond = −, {}d | Q22 = −, {}d",
        digit_count(&lambda_plus), digit_count(&lambda_minus), digit_count(q22));
    println!("2. λ_crack − Q22 = {} | λ_bond − Q22 = {}",
        if denom_crack.is_positive() { "POSITIVE" } else { "NEGATIVE" },
        if denom_bond.is_positive() { "POSITIVE" } else { "NEGATIVE" });
    println!("3. Δλ_crack {} | Δλ_bond {}",
        if full_denom_crack.is_positive() { "POSITIVE (raises crack)" } else { "NEGATIVE (lowers crack)" },
        if full_denom_bond.is_positive() { "POSITIVE (weakens bond)" } else { "NEGATIVE (deepens bond)" });
    println!("4. |Δλ_bond|/|Δλ_crack| ≈ {:.1}×", ratio_f);
    println!("5. Original eigenvalue ratio: {}", &orig_ratio[..20.min(orig_ratio.len())]);
    println!("6. Corrected eigenvalue ratio: {}", &corr_ratio[..20.min(corr_ratio.len())]);
    println!("7. Schur eigenvalue ratio: {}", &schur_ratio[..20.min(schur_ratio.len())]);
    if denom_bond.is_positive() {
        println!("8. VERDICT: LEVEL REPULSION — torroid weakens bond, raises crack");
        println!("   The initial hypothesis (torroid deepens bond) was WRONG.");
        println!("   Q22 is a deeper well than λ_bond. Level repulsion pushes them apart.");
    } else {
        println!("8. VERDICT: torroid deepens bond, raises crack");
    }
}
