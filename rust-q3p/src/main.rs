//! H135: The Residual Eigenvalue
//! Compute exact eigenvectors of the 2×2 brace-box block.
//! Find the "crack direction" — the small eigenvalue's eigenvector.
//! Compare to torroid coupling direction φ = 60.92° from H132.

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

    println!("=== H135: THE RESIDUAL EIGENVALUE (Rust BigInt) ===\n");

    // --- Step 1: 2x2 eigenvalue decomposition ---
    println!("=== STEP 1: 2x2 BRACE-BOX BLOCK ===");
    println!("|Q00| = {}d", digit_count(q00));
    println!("|Q01| = {}d", digit_count(q01));
    println!("|Q11| = {}d", digit_count(q11));

    let trace = q00 + q11;
    let det = q00 * q11 - q01 * q01;
    println!("\ntrace = Q00 + Q11");
    println!("trace sign: {}", if trace.is_negative() { "NEGATIVE" } else { "POSITIVE" });
    println!("|trace| = {}d", digit_count(&trace));
    println!("det = Q00*Q11 - Q01^2");
    println!("det sign: {}", if det.is_negative() { "NEGATIVE" } else { "POSITIVE" });
    println!("|det| = {}d", digit_count(&det));

    // --- Step 2: discriminant = (Q00-Q11)^2 + 4*Q01^2 ---
    println!("\n=== STEP 2: DISCRIMINANT ===");
    let diff = q00 - q11;
    let disc = &diff * &diff + BigInt::from(4) * q01 * q01;
    println!("disc = (Q00-Q11)^2 + 4*Q01^2");
    println!("disc sign: {}", if disc.is_negative() { "NEGATIVE (impossible!)" } else { "POSITIVE -> two real eigenvalues" });
    println!("|disc| = {}d", digit_count(&disc));

    // --- Step 3: integer sqrt of discriminant ---
    println!("\n=== STEP 3: INTEGER SQUARE ROOT OF DISCRIMINANT ===");
    let sqrt_disc = integer_sqrt(&disc);
    let check = &sqrt_disc * &sqrt_disc;
    let sqrt_error = &disc - &check;
    println!("|sqrt_disc| = {}d", digit_count(&sqrt_disc));
    println!("disc - (sqrt_disc)^2 = {} ({}d residual)",
        if sqrt_error.is_zero() { "0" } else { "nonzero" }, digit_count(&sqrt_error));
    println!("Perfect square? {}", if sqrt_error.is_zero() { "YES" } else { "NO" });

    // --- Step 4: eigenvector components ---
    // For symmetric 2x2 [[a,b],[b,c]]:
    // eigenvalues: lam = (a+c)/2 +/- sqrt((a-c)^2+4b^2)/2
    // eigenvector for lam: [b, lam-a] = [2b, (c-a) +/- sqrt_disc]
    println!("\n=== STEP 4: EIGENVECTOR COMPONENTS ===");
    let two_q01 = BigInt::from(2) * q01;
    let q11_minus_q00 = q11 - q00;

    // Large eigenvalue eigenvector: [2*Q01, (Q11-Q00) + sqrt_disc]
    let v_large_x = two_q01.clone();
    let v_large_y = &q11_minus_q00 + &sqrt_disc;

    // Small eigenvalue eigenvector: [2*Q01, (Q11-Q00) - sqrt_disc]
    let v_small_x = two_q01.clone();
    let v_small_y = &q11_minus_q00 - &sqrt_disc;

    println!("LARGE eigenvector (dominant bond):");
    println!("  v+ = [2*Q01, (Q11-Q00)+sqrt_disc]");
    println!("  v+_x sign: {}, {}d", if v_large_x.is_negative() { "-" } else { "+" }, digit_count(&v_large_x));
    println!("  v+_y sign: {}, {}d", if v_large_y.is_negative() { "-" } else { "+" }, digit_count(&v_large_y));

    println!("\nSMALL eigenvector (crack direction):");
    println!("  v- = [2*Q01, (Q11-Q00)-sqrt_disc]");
    println!("  v-_x sign: {}, {}d", if v_small_x.is_negative() { "-" } else { "+" }, digit_count(&v_small_x));
    println!("  v-_y sign: {}, {}d", if v_small_y.is_negative() { "-" } else { "+" }, digit_count(&v_small_y));

    // --- Step 5: angle computation ---
    println!("\n=== STEP 5: EIGENVECTOR ANGLES ===");
    let prec = 40u32;
    let tan_large = fp_div(&v_large_y, &v_large_x, prec);
    let tan_small = fp_div(&v_small_y, &v_small_x, prec);
    println!("tan(theta_large) = v+_y/v+_x = {}", &tan_large[..50.min(tan_large.len())]);
    println!("tan(theta_small) = v-_y/v-_x = {}", &tan_small[..50.min(tan_small.len())]);

    let tan_large_f = tan_large.parse::<f64>().unwrap_or(0.0);
    let tan_small_f = tan_small.parse::<f64>().unwrap_or(0.0);
    let theta_large_deg = tan_large_f.atan() * 180.0 / std::f64::consts::PI;
    let theta_small_deg = tan_small_f.atan() * 180.0 / std::f64::consts::PI;
    println!("\ntheta_large = {:.6} deg from brace axis (dominant bond direction)", theta_large_deg);
    println!("theta_small = {:.6} deg from brace axis (crack direction)", theta_small_deg);
    println!("theta_large + theta_small = {:.6} deg (should be +/-90 for orthogonal)", theta_large_deg + theta_small_deg);

    // --- Step 6: comparison to H122 and H132 ---
    println!("\n=== STEP 6: COMPARISON TO PRIOR INSTRUMENTS ===");
    let phi_torroid = 60.92_f64;
    let theta_h122 = 29.35_f64;

    println!("H122 Q3p thin eigenvector (collapse): theta1 = {:.2} deg", theta_h122);
    println!("H122 Q3p wide eigenvector (stable):   theta2 = {:.2} deg", theta_h122 - 90.0);
    println!("H132 torroid coupling direction:        phi   = {:.2} deg", phi_torroid);
    println!("H135 2x2 large (bond) direction:        thetaL = {:.4} deg", theta_large_deg);
    println!("H135 2x2 small (crack) direction:       thetaS = {:.4} deg", theta_small_deg);

    let crack_vs_torroid = (theta_small_deg - phi_torroid).abs();
    let crack_vs_thin = (theta_small_deg - theta_h122).abs();
    let crack_vs_wide = (theta_small_deg - (theta_h122 - 90.0)).abs();
    let bond_vs_thin = (theta_large_deg - theta_h122).abs();

    println!("\n=== ANGULAR RELATIONSHIPS ===");
    println!("|crack - torroid| = {:.4} deg", crack_vs_torroid);
    println!("|crack - thin|    = {:.4} deg", crack_vs_thin);
    println!("|crack - wide|    = {:.4} deg", crack_vs_wide);
    println!("|bond  - thin|    = {:.4} deg", bond_vs_thin);

    // --- Step 7: torroid-crack alignment ---
    println!("\n=== STEP 7: TORROID-CRACK ALIGNMENT ===");
    let phi_rad = phi_torroid.to_radians();
    let crack_rad = theta_small_deg.to_radians();
    let cos_diff = (phi_rad - crack_rad).cos();
    let sin_diff = (phi_rad - crack_rad).sin();
    println!("cos^2(phi - theta_crack) = {:.6} ({:.2}% along crack)", cos_diff*cos_diff, cos_diff*cos_diff*100.0);
    println!("sin^2(phi - theta_crack) = {:.6} ({:.2}% along bond)", sin_diff*sin_diff, sin_diff*sin_diff*100.0);

    if cos_diff*cos_diff > 0.5 {
        println!("-> TORROID COUPLES PREFERENTIALLY ALONG THE CRACK DIRECTION");
    } else {
        println!("-> TORROID COUPLES PREFERENTIALLY ALONG THE BOND DIRECTION");
    }

    // --- Step 8: exact eigenvalues ---
    println!("\n=== STEP 8: EXACT EIGENVALUES ===");
    // lam = (trace +/- sqrt_disc) / 2
    let lambda_large = (&trace + &sqrt_disc) / BigInt::from(2);
    let lambda_small = (&trace - &sqrt_disc) / BigInt::from(2);
    println!("lambda_large = (trace + sqrt_disc)/2");
    println!("  sign: {}, {}d", if lambda_large.is_negative() { "-" } else { "+" }, digit_count(&lambda_large));
    println!("lambda_small = (trace - sqrt_disc)/2");
    println!("  sign: {}, {}d", if lambda_small.is_negative() { "-" } else { "+" }, digit_count(&lambda_small));

    let ratio_str = fp_div(&lambda_small.abs(), &lambda_large.abs(), 20);
    println!("|lambda_small|/|lambda_large| = {}", &ratio_str[..30.min(ratio_str.len())]);

    // Verify: lam_large + lam_small = trace
    let eig_sum = &lambda_large + &lambda_small;
    println!("\nVerify: lam_large + lam_small = trace? {}", if eig_sum == trace { "YES" } else { "NO" });

    // Verify: lam_large * lam_small = det (up to sqrt rounding)
    let eig_prod = &lambda_large * &lambda_small;
    // det_exact = (trace^2 - disc)/4 but disc might not be perfect square
    // so eig_prod = (trace^2 - sqrt_disc^2)/4
    // error = eig_prod - det = (trace^2 - sqrt_disc^2)/4 - (Q00*Q11 - Q01^2)
    //       = (trace^2 - sqrt_disc^2 - 4*det)/4
    //       = (trace^2 - 4*det - sqrt_disc^2)/4
    //       = (disc - sqrt_disc^2)/4
    let det_error = &eig_prod - &det;
    println!("lambda_large * lambda_small - det has {}d (should be ~0 or from sqrt rounding)", digit_count(&det_error));

    // --- Step 9: torroid coupling in eigenbasis ---
    println!("\n=== STEP 9: TORROID COUPLING DECOMPOSITION IN EIGENBASIS ===");
    // Torroid coupling vector is [Q02, Q12] in {brace, box} basis.
    // Project onto eigenvectors of the 2x2 block.
    let dot_large = q02 * &v_large_x + q12 * &v_large_y;
    let dot_small = q02 * &v_small_x + q12 * &v_small_y;
    println!("Torroid dot v_large (bond): {}d, sign {}", digit_count(&dot_large), if dot_large.is_negative() { "-" } else { "+" });
    println!("Torroid dot v_small (crack): {}d, sign {}", digit_count(&dot_small), if dot_small.is_negative() { "-" } else { "+" });

    let norm_large_sq = &v_large_x * &v_large_x + &v_large_y * &v_large_y;
    let norm_small_sq = &v_small_x * &v_small_x + &v_small_y * &v_small_y;

    // Normalized projection magnitudes: (dot)^2 / norm^2
    // Compare: f_L / f_S = (dot_L^2 / norm_L_sq) / (dot_S^2 / norm_S_sq)
    //                     = (dot_L^2 * norm_S_sq) / (dot_S^2 * norm_L_sq)
    let f_large = &dot_large * &dot_large * &norm_small_sq;
    let f_small = &dot_small * &dot_small * &norm_large_sq;
    let f_total = &f_large + &f_small;
    let pct_large = fp_div(&f_large, &f_total, 20);
    let pct_small = fp_div(&f_small, &f_total, 20);
    println!("\nTorroid coupling energy split (in 2x2 eigenbasis):");
    println!("  Along BOND (large lam): {}", &pct_large[..20.min(pct_large.len())]);
    println!("  Along CRACK (small lam): {}", &pct_small[..20.min(pct_small.len())]);

    let pct_large_f = pct_large.parse::<f64>().unwrap_or(0.0) * 100.0;
    let pct_small_f = pct_small.parse::<f64>().unwrap_or(0.0) * 100.0;
    println!("  -> {:.2}% bond / {:.2}% crack", pct_large_f, pct_small_f);

    if pct_small_f > pct_large_f {
        println!("  -> TORROID PREFERENTIALLY COUPLES ALONG THE CRACK");
    } else {
        println!("  -> TORROID PREFERENTIALLY COUPLES ALONG THE BOND");
    }

    // --- Step 10: The crack eigenvector's exact ratio ---
    println!("\n=== STEP 10: CRACK EIGENVECTOR EXACT RATIO ===");
    // v_small = [2*Q01, (Q11-Q00) - sqrt_disc]
    // The ratio v_small_y / v_small_x tells us the exact tilt
    let crack_ratio = fp_div(&v_small_y, &v_small_x, 40);
    println!("v_small_y / v_small_x = {}", &crack_ratio[..50.min(crack_ratio.len())]);
    println!("This is tan(theta_crack) to 40 decimal places");

    // Also compute for large eigenvector
    let bond_ratio = fp_div(&v_large_y, &v_large_x, 40);
    println!("v_large_y / v_large_x = {}", &bond_ratio[..50.min(bond_ratio.len())]);

    // --- Step 11: Exact digit counts for all intermediate values ---
    println!("\n=== STEP 11: DIGIT COUNT SUMMARY ===");
    println!("trace:       {}d", digit_count(&trace));
    println!("det:         {}d", digit_count(&det));
    println!("disc:        {}d", digit_count(&disc));
    println!("sqrt_disc:   {}d", digit_count(&sqrt_disc));
    println!("sqrt_error:  {}d", digit_count(&sqrt_error));
    println!("v_large_x:   {}d", digit_count(&v_large_x));
    println!("v_large_y:   {}d", digit_count(&v_large_y));
    println!("v_small_x:   {}d", digit_count(&v_small_x));
    println!("v_small_y:   {}d", digit_count(&v_small_y));
    println!("lam_large:   {}d", digit_count(&lambda_large));
    println!("lam_small:   {}d", digit_count(&lambda_small));
    println!("dot_large:   {}d", digit_count(&dot_large));
    println!("dot_small:   {}d", digit_count(&dot_small));

    println!("\n=== SUMMARY ===");
    println!("2x2 block eigenvectors:");
    println!("  Bond direction:  {:.4} deg from brace axis", theta_large_deg);
    println!("  Crack direction: {:.4} deg from brace axis", theta_small_deg);
    println!("  Orthogonal check: sum = {:.4} deg (should be ~-90)", theta_large_deg + theta_small_deg);
    println!("Torroid coupling direction (H132): {:.2} deg", phi_torroid);
    println!("Crack-to-torroid angle: {:.4} deg", crack_vs_torroid);
    println!("Torroid energy: {:.2}% bond / {:.2}% crack", pct_large_f, pct_small_f);
    println!("VERDICT: torroid couples preferentially along the {}",
        if pct_small_f > pct_large_f { "CRACK (unlocked direction)" } else { "BOND (locked direction)" });
}
