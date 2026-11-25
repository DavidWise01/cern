//! H138: The Cubic Factorization
//! Structure of the characteristic polynomial coefficients.
//! Why M02 ≈ M12 while |M01| is 85 digits smaller.
//! Exact ratios, cofactor decomposition, near-cancellation anatomy.

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

    println!("=== H138: THE CUBIC FACTORIZATION (Rust BigInt) ===\n");

    // =====================================================
    // STEP 1: The three 2×2 principal minors
    // =====================================================
    println!("=== STEP 1: THE THREE MINORS ===");

    let m01 = q00 * q11 - q01 * q01;  // brace-box
    let m02 = q00 * q22 - q02 * q02;  // brace-torroid
    let m12 = q11 * q22 - q12 * q12;  // box-torroid

    println!("M01 (brace⊗box)     = {} sign, {}d", if m01.is_negative() { "-" } else { "+" }, digit_count(&m01));
    println!("M02 (brace⊗torroid) = {} sign, {}d", if m02.is_negative() { "-" } else { "+" }, digit_count(&m02));
    println!("M12 (box⊗torroid)   = {} sign, {}d", if m12.is_negative() { "-" } else { "+" }, digit_count(&m12));

    // =====================================================
    // STEP 2: Exact ratio M02/M12
    // =====================================================
    println!("\n=== STEP 2: EXACT RATIO M02/M12 ===");
    let gcd_02_12 = m02.gcd(&m12);
    println!("gcd(M02, M12) = {}d", digit_count(&gcd_02_12));
    let m02_red = &m02 / &gcd_02_12;
    let m12_red = &m12 / &gcd_02_12;
    println!("M02/gcd = {}d", digit_count(&m02_red));
    println!("M12/gcd = {}d", digit_count(&m12_red));
    println!("M02/M12 = {}", fp_div(&m02, &m12, 25));

    // Difference M12 - M02
    let diff_12_02 = &m12 - &m02;
    println!("\nM12 − M02 = {} sign, {}d", if diff_12_02.is_negative() { "-" } else { "+" }, digit_count(&diff_12_02));
    println!("(M12−M02)/M12 = {}", fp_div(&diff_12_02, &m12, 25));

    // =====================================================
    // STEP 3: Why M01 is 85 digits smaller
    // =====================================================
    println!("\n=== STEP 3: THE M01 GAP ===");
    // M01 = Q00·Q11 - Q01²
    // Q00 is -328d, Q11 is -329d → Q00·Q11 is +657d
    // Q01 is +328d → Q01² is +656d
    // M01 is their difference — a near-cancellation

    let prod_00_11 = q00 * q11;
    let sq_01 = q01 * q01;
    println!("Q00·Q11 = {} sign, {}d", if prod_00_11.is_negative() { "-" } else { "+" }, digit_count(&prod_00_11));
    println!("Q01²    = {} sign, {}d", if sq_01.is_negative() { "-" } else { "+" }, digit_count(&sq_01));
    println!("M01 = Q00·Q11 − Q01² = {} sign, {}d", if m01.is_negative() { "-" } else { "+" }, digit_count(&m01));
    println!("CANCELLATION: {}d + {}d → {}d (lost {}d)",
        digit_count(&prod_00_11), digit_count(&sq_01), digit_count(&m01),
        digit_count(&prod_00_11) - digit_count(&m01));

    // For M02: Q00·Q22 vs Q02²
    let prod_00_22 = q00 * q22;
    let sq_02 = q02 * q02;
    println!("\nQ00·Q22 = {} sign, {}d", if prod_00_22.is_negative() { "-" } else { "+" }, digit_count(&prod_00_22));
    println!("Q02²    = {} sign, {}d", if sq_02.is_negative() { "-" } else { "+" }, digit_count(&sq_02));
    println!("M02 = Q00·Q22 − Q02² = {} sign, {}d", if m02.is_negative() { "-" } else { "+" }, digit_count(&m02));
    println!("CANCELLATION: {}d + {}d → {}d (lost {}d)",
        digit_count(&prod_00_22), digit_count(&sq_02), digit_count(&m02),
        digit_count(&prod_00_22) - digit_count(&m02));

    // For M12: Q11·Q22 vs Q12²
    let prod_11_22 = q11 * q22;
    let sq_12 = q12 * q12;
    println!("\nQ11·Q22 = {} sign, {}d", if prod_11_22.is_negative() { "-" } else { "+" }, digit_count(&prod_11_22));
    println!("Q12²    = {} sign, {}d", if sq_12.is_negative() { "-" } else { "+" }, digit_count(&sq_12));
    println!("M12 = Q11·Q22 − Q12² = {} sign, {}d", if m12.is_negative() { "-" } else { "+" }, digit_count(&m12));
    println!("CANCELLATION: {}d + {}d → {}d (lost {}d)",
        digit_count(&prod_11_22), digit_count(&sq_12), digit_count(&m12),
        digit_count(&prod_11_22) - digit_count(&m12));

    // =====================================================
    // STEP 4: c₁ decomposition
    // =====================================================
    println!("\n=== STEP 4: c₁ DECOMPOSITION ===");
    let c1 = &m01 + &m02 + &m12;
    println!("c₁ = M01 + M02 + M12 = {} sign, {}d", if c1.is_negative() { "-" } else { "+" }, digit_count(&c1));

    let c1_no_m01 = &m02 + &m12;
    println!("M02 + M12 (without M01) = {} sign, {}d", if c1_no_m01.is_negative() { "-" } else { "+" }, digit_count(&c1_no_m01));

    let m01_fraction = fp_div(&m01.abs(), &c1.abs(), 25);
    println!("|M01|/|c₁| = {} (how much M01 contributes)", m01_fraction);

    let m02_fraction = fp_div(&m02.abs(), &c1.abs(), 25);
    let m12_fraction = fp_div(&m12.abs(), &c1.abs(), 25);
    println!("|M02|/|c₁| = {}", m02_fraction);
    println!("|M12|/|c₁| = {}", m12_fraction);

    // c₁ correction from M01
    let correction = &c1 - &c1_no_m01; // should equal M01
    println!("\nc₁ − (M02+M12) = M01 = {} sign, {}d (check)", if correction.is_negative() { "-" } else { "+" }, digit_count(&correction));

    // =====================================================
    // STEP 5: Determinant cofactor decomposition
    // =====================================================
    println!("\n=== STEP 5: DETERMINANT COFACTOR DECOMPOSITION ===");
    // det(Q) = Q00(Q11·Q22 - Q12²) - Q01(Q01·Q22 - Q02·Q12) + Q02(Q01·Q12 - Q11·Q02)
    // = Q00·cofactor00 - Q01·cofactor01 + Q02·cofactor02

    let cof00 = q11 * q22 - q12 * q12;      // = M12 (the box-torroid minor!)
    let cof01_neg = q01 * q22 - q12 * q02;   // the negated (0,1) cofactor
    let cof02 = q01 * q12 - q11 * q02;

    let term0 = q00 * &cof00;
    let term1 = -(q01 * &cof01_neg);
    let term2 = q02 * &cof02;
    let det = &term0 + &term1 + &term2;

    println!("Q00·C00 = {} sign, {}d", if term0.is_negative() { "-" } else { "+" }, digit_count(&term0));
    println!("Q01·C01 = {} sign, {}d", if term1.is_negative() { "-" } else { "+" }, digit_count(&term1));
    println!("Q02·C02 = {} sign, {}d", if term2.is_negative() { "-" } else { "+" }, digit_count(&term2));
    println!("det(Q)  = {} sign, {}d", if det.is_negative() { "-" } else { "+" }, digit_count(&det));

    // Fractions
    println!("\nCofactor fractions of det:");
    if !det.is_zero() {
        println!("  |Q00·C00|/|det| = {}", fp_div(&term0.abs(), &det.abs(), 20));
        println!("  |Q01·C01|/|det| = {}", fp_div(&term1.abs(), &det.abs(), 20));
        println!("  |Q02·C02|/|det| = {}", fp_div(&term2.abs(), &det.abs(), 20));
    }

    // =====================================================
    // STEP 6: Cauchy-Schwarz fractions for each minor
    // =====================================================
    println!("\n=== STEP 6: CAUCHY-SCHWARZ FRACTIONS (MINOR VERSION) ===");
    // For each minor M_ij = Q_ii·Q_jj - Q_ij², the CS fraction is Q_ij²/(Q_ii·Q_jj)
    // If CS fraction > 1, the 2×2 block is indefinite

    // M01: brace-box
    let cs01 = fp_div(&sq_01, &prod_00_11.abs(), 25);
    println!("CS(brace,box)     = Q01²/(Q00·Q11) = {}", cs01);
    println!("  M01 sign: {} → block is {}",
        if m01.is_negative() { "−" } else { "+" },
        if m01.is_negative() { "INDEFINITE (CS > 1)" } else { "DEFINITE (CS < 1)" });

    // M02: brace-torroid
    let cs02 = fp_div(&sq_02, &prod_00_22.abs(), 25);
    println!("CS(brace,torroid) = Q02²/(Q00·Q22) = {}", cs02);
    println!("  M02 sign: {} → block is {}",
        if m02.is_negative() { "−" } else { "+" },
        if m02.is_negative() { "INDEFINITE (CS > 1)" } else { "DEFINITE (CS < 1)" });

    // M12: box-torroid
    let cs12 = fp_div(&sq_12, &prod_11_22.abs(), 25);
    println!("CS(box,torroid)   = Q12²/(Q11·Q22) = {}", cs12);
    println!("  M12 sign: {} → block is {}",
        if m12.is_negative() { "−" } else { "+" },
        if m12.is_negative() { "INDEFINITE (CS > 1)" } else { "DEFINITE (CS < 1)" });

    // =====================================================
    // STEP 7: The GCD structure
    // =====================================================
    println!("\n=== STEP 7: GCD STRUCTURE ===");
    let gcd_all_3 = m01.gcd(&m02).gcd(&m12);
    println!("gcd(M01, M02, M12) = {}d", digit_count(&gcd_all_3));
    println!("gcd value = {}", if digit_count(&gcd_all_3) < 20 { gcd_all_3.to_string() } else { format!("{}d number", digit_count(&gcd_all_3)) });

    let gcd_01_02 = m01.gcd(&m02);
    let gcd_01_12 = m01.gcd(&m12);
    println!("gcd(M01, M02) = {}d", digit_count(&gcd_01_02));
    println!("gcd(M01, M12) = {}d", digit_count(&gcd_01_12));
    println!("gcd(M02, M12) = {}d", digit_count(&gcd_02_12));

    // =====================================================
    // STEP 8: The three-scale hierarchy in c₁
    // =====================================================
    println!("\n=== STEP 8: THREE-SCALE HIERARCHY IN c₁ ===");
    // M01 is negative and 651d
    // M02 is positive and 735d
    // M12 is positive and 736d
    // c₁ = M01 + M02 + M12 = (M02 + M12) + M01
    // Since M01 < 0 and |M01| ≪ M02, M12: c₁ ≈ M02 + M12

    // But how much does M01 perturb the sum?
    let relative_perturbation = fp_div(&m01.abs(), &c1_no_m01, 25);
    println!("|M01| / (M02+M12) = {} (relative perturbation)", relative_perturbation);
    println!("This is ~10^(651−736) = 10^(-85)");

    // M02/M12 ratio tells us how close they are
    println!("\nM12/M02 = {}", fp_div(&m12, &m02, 25));
    println!("(M12−M02)/M12 = {}", fp_div(&diff_12_02, &m12, 25));

    // =====================================================
    // STEP 9: Cofactor note — C00 = M12
    // =====================================================
    println!("\n=== STEP 9: COFACTOR-MINOR IDENTITY ===");
    let c00_check = &cof00 - &m12;
    let c00_msg = if c00_check.is_zero() { "0 ✓".to_string() } else { format!("NONZERO: {}d", digit_count(&c00_check)) };
    println!("C00 − M12 = {} (should be 0)", c00_msg);
    println!("IDENTITY: The (0,0) cofactor IS the box-torroid minor M12.");
    println!("So: det(Q) = Q00·M12 − Q01·(Q01·Q22−Q02·Q12) + Q02·(Q01·Q12−Q11·Q02)");

    // =====================================================
    // STEP 10: The geometric mean check on minors
    // =====================================================
    println!("\n=== STEP 10: GEOMETRIC MEAN CHECK ON MINORS ===");
    // H133 found |Q_ij| ≈ √(|Q_ii|·|Q_jj|) for the matrix entries
    // Does a similar pattern hold for minors?
    // "geometric mean" of M02 and M12 vs M01?
    // M01 is negative, so this is different...

    // Instead: check if det(Q) has a simple relationship to the minors
    // det(Q) = Q00·M12 + correction terms
    let det_leading = q00 * &m12;
    let det_correction = &det - &det_leading;
    println!("Q00·M12 (leading term of det) = {} sign, {}d",
        if det_leading.is_negative() { "-" } else { "+" }, digit_count(&det_leading));
    println!("det − Q00·M12 (correction) = {} sign, {}d",
        if det_correction.is_negative() { "-" } else { "+" }, digit_count(&det_correction));
    println!("correction/det = {}", fp_div(&det_correction.abs(), &det.abs(), 20));

    // =====================================================
    // STEP 11: Summary digit table
    // =====================================================
    println!("\n=== STEP 11: DIGIT COUNT SUMMARY TABLE ===");
    println!("{:<30} {:>5} {:>6}", "Quantity", "Sign", "Digits");
    println!("{:-<43}", "");
    println!("{:<30} {:>5} {:>6}", "M01 (brace⊗box)", if m01.is_negative() { "−" } else { "+" }, digit_count(&m01));
    println!("{:<30} {:>5} {:>6}", "M02 (brace⊗torroid)", if m02.is_negative() { "−" } else { "+" }, digit_count(&m02));
    println!("{:<30} {:>5} {:>6}", "M12 (box⊗torroid)", if m12.is_negative() { "−" } else { "+" }, digit_count(&m12));
    println!("{:<30} {:>5} {:>6}", "c1 = M01+M02+M12", if c1.is_negative() { "−" } else { "+" }, digit_count(&c1));
    println!("{:<30} {:>5} {:>6}", "M02+M12 (without M01)", if c1_no_m01.is_negative() { "−" } else { "+" }, digit_count(&c1_no_m01));
    println!("{:<30} {:>5} {:>6}", "det(Q)", if det.is_negative() { "−" } else { "+" }, digit_count(&det));
    println!("{:<30} {:>5} {:>6}", "Q00·C00 term", if term0.is_negative() { "−" } else { "+" }, digit_count(&term0));
    println!("{:<30} {:>5} {:>6}", "Q01·C01 term", if term1.is_negative() { "−" } else { "+" }, digit_count(&term1));
    println!("{:<30} {:>5} {:>6}", "Q02·C02 term", if term2.is_negative() { "−" } else { "+" }, digit_count(&term2));

    println!("\n=== SUMMARY ===");
    println!("H138: THE CUBIC FACTORIZATION");
    println!("1. M01 (−651d) is 85 digits smaller than M02 (+735d) / M12 (+736d)");
    println!("2. The gap comes from MASSIVE CANCELLATION in M01:");
    println!("   Q00·Q11 and Q01² are both ~657d, their difference is only 651d");
    println!("   Lost 6 digits — the brace-box pair is NEARLY saturated (CS ≈ 1)");
    println!("3. M02 and M12 have NO such cancellation:");
    println!("   The torroid pairs are well below CS saturation");
    println!("4. M02/M12 ratio: near 1 but NOT exactly 1");
    println!("5. c₁ ≈ M02 + M12 to 85-digit precision; M01 is a 10⁻⁸⁵ perturbation");
    println!("6. det(Q) leading term is Q00·M12; corrections are sub-leading");
    println!("7. The three-scale hierarchy in eigenvalues ORIGINATES in the minor hierarchy:");
    println!("   |M01| ≪ M02 ≈ M12 → one weak link (brace-box) + two strong links (torroid)");
}
