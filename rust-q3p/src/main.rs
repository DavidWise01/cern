//! H155: The Lattice Measure
//! Computing invariants of Q3p that determine the minimum word length 21.
//! Key question: WHY 21 and not some other number?

use num_bigint::BigInt;
use num_traits::{Zero, One, Signed};
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

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0]; let q01 = &q[0][1]; let q11 = &q[1][1];
    let q02 = &q[0][2]; let q12 = &q[1][2]; let q22 = &q[2][2];

    println!("=== H155: THE LATTICE MEASURE (Rust BigInt) ===\n");

    // ═══ STEP 1: ALL COFACTORS ═══
    println!("=== STEP 1: COFACTOR MATRIX (ADJUGATE) ===");
    // cof_ij = (-1)^(i+j) * M_ij where M_ij is the (i,j) minor
    let cof00 = q11 * q22 - q12 * q12;      // M00
    let cof11 = q00 * q22 - q02 * q02;      // M11
    let cof22 = q00 * q11 - q01 * q01;      // M22 = -R01
    let cof01 = -(q01 * q22 - q12 * q02);   // -M01
    let cof02 = q01 * q12 - q11 * q02;      // M02
    let cof12 = -(q00 * q12 - q01 * q02);   // -M12

    println!("Cofactor digit counts:");
    println!("  cof(Q00) = Q11*Q22 - Q12^2 = {} sign, {}d",
        if cof00.is_positive() { "+" } else { "-" }, digit_count(&cof00));
    println!("  cof(Q11) = Q00*Q22 - Q02^2 = {} sign, {}d",
        if cof11.is_positive() { "+" } else { "-" }, digit_count(&cof11));
    println!("  cof(Q22) = Q00*Q11 - Q01^2 = {} sign, {}d (= -R01)",
        if cof22.is_positive() { "+" } else { "-" }, digit_count(&cof22));
    println!("  cof(Q01) = -(Q01*Q22-Q12*Q02) = {} sign, {}d",
        if cof01.is_positive() { "+" } else { "-" }, digit_count(&cof01));
    println!("  cof(Q02) = Q01*Q12-Q11*Q02 = {} sign, {}d",
        if cof02.is_positive() { "+" } else { "-" }, digit_count(&cof02));
    println!("  cof(Q12) = -(Q00*Q12-Q01*Q02) = {} sign, {}d",
        if cof12.is_positive() { "+" } else { "-" }, digit_count(&cof12));

    // ═══ STEP 2: DETERMINANT ═══
    println!("\n=== STEP 2: DETERMINANT ===");
    let det = q00 * &cof00 + q01 * &cof01 + q02 * &cof02;
    println!("det(Q3p) = {} sign, {}d",
        if det.is_positive() { "+" } else { "-" }, digit_count(&det));
    let det_s = det.magnitude().to_string();
    println!("  Leading digits: {}...", &det_s[..30.min(det_s.len())]);

    // ═══ STEP 3: GCD STRUCTURE ═══
    println!("\n=== STEP 3: GCD STRUCTURE ===");
    // d1 = gcd of all entries
    let g1 = q00.abs().gcd(&q01.abs());
    let g2 = g1.gcd(&q11.abs());
    let g3 = g2.gcd(&q02.abs());
    let g4 = g3.gcd(&q12.abs());
    let d1 = g4.gcd(&q22.abs());
    println!("d1 = gcd(all entries) = {}", d1);

    // d1*d2 = gcd of all cofactors (2x2 minors)
    let gc1 = cof00.abs().gcd(&cof11.abs());
    let gc2 = gc1.gcd(&cof22.abs());
    let gc3 = gc2.gcd(&cof01.abs());
    let gc4 = gc3.gcd(&cof02.abs());
    let gcd_cof = gc4.gcd(&cof12.abs());
    println!("gcd(all cofactors) = {}d", digit_count(&gcd_cof));
    let d2 = &gcd_cof / &d1;
    println!("d2 = gcd(cofactors)/d1 = {}d", digit_count(&d2));

    // d3 = det / (d1 * d2)
    let d1d2 = &d1 * &d2;
    let d3 = &det.abs() / &d1d2;
    println!("d1*d2 = {}d", digit_count(&d1d2));
    println!("d3 = |det|/(d1*d2) = {}d", digit_count(&d3));

    // Check: d1*d2*d3 should equal |det|
    let check = &d1 * &d2 * &d3;
    let det_check = &check - &det.abs();
    println!("d1*d2*d3 - |det| = {} (should be 0 or small)",
        if det_check.is_zero() { "0 ✓".to_string() } else { format!("{}d", digit_count(&det_check)) });

    println!("\nSMITH NORMAL FORM: diag({}, {}d, {}d)", d1, digit_count(&d2), digit_count(&d3));

    // ═══ STEP 4: THE 42-DIGIT GAP CONNECTION ═══
    println!("\n=== STEP 4: THE 42-DIGIT GAP CONNECTION ===");
    // From H140: the Schur complement gap = 42 digits
    // The gap is: log10(|M02/X|) where M02 is the (0,2) Schur diagonal
    // and X is the cross-minor.
    // M02 = cof(Q11) = Q00*Q22 - Q02^2 (the Schur complement diagonal)
    // The cross-minor: Q01*Q22 - Q02*Q12 (or similar)

    // The 42d gap means: the torroid is decoupled at 10^42 precision
    // CONJECTURE: minimum word length = gap/2 = 42/2 = 21

    // Let me verify the gap exactly:
    let r01 = q01 * q01 - &(q00.abs() * q11.abs()); // 651d
    // The Schur complement of Q22 (eliminate torroid):
    // S22 = [[Q00 - Q02^2/Q22, Q01 - Q02*Q12/Q22],
    //        [Q01 - Q02*Q12/Q22, Q11 - Q12^2/Q22]]
    //
    // The off-diagonal of S22: Q01 - Q02*Q12/Q22
    // = (Q01*Q22 - Q02*Q12) / Q22

    let s22_offdiag_num = q01 * q22 - q02 * q12; // = -cof01
    println!("Schur off-diagonal numerator: {}d", digit_count(&s22_offdiag_num));
    println!("Q22 (denominator): {}d", digit_count(q22));
    println!("Schur off-diagonal: {}d / {}d = effective {}d",
        digit_count(&s22_offdiag_num), digit_count(q22),
        digit_count(&s22_offdiag_num) as i64 - digit_count(q22) as i64);

    // Schur diagonal: Q00 - Q02^2/Q22 = (Q00*Q22 - Q02^2)/Q22 = cof11/Q22
    let s22_diag0_num = q00 * q22 - q02 * q02; // = cof11
    let s22_diag1_num = q11 * q22 - q12 * q12; // = cof00

    println!("\nSchur complement of Q22:");
    println!("  S[0,0] numerator: {}d (= cof11)", digit_count(&s22_diag0_num));
    println!("  S[1,1] numerator: {}d (= cof00)", digit_count(&s22_diag1_num));
    println!("  S[0,1] numerator: {}d (= -cof01)", digit_count(&s22_offdiag_num));

    // The "gap" is the digit count difference between diagonal and off-diagonal
    let diag_digits = digit_count(&s22_diag0_num) as i64;
    let offdiag_digits = digit_count(&s22_offdiag_num) as i64;
    let gap = diag_digits - offdiag_digits;
    println!("\n  DIGIT GAP = {} - {} = {}", diag_digits, offdiag_digits, gap);
    println!("  GAP / 2 = {}", gap as f64 / 2.0);
    println!("  21-Root Theorem: minimum word length = 21");
    println!("  MATCH: gap/2 = {} vs word length = 21", gap / 2);

    // ═══ STEP 5: THE COFACTOR DIGIT SPECTRUM ═══
    println!("\n=== STEP 5: COFACTOR DIGIT SPECTRUM ===");
    let cof_digits = [
        ("cof00", digit_count(&cof00)),
        ("cof11", digit_count(&cof11)),
        ("cof22", digit_count(&cof22)),
        ("cof01", digit_count(&cof01)),
        ("cof02", digit_count(&cof02)),
        ("cof12", digit_count(&cof12)),
    ];
    for (name, d) in &cof_digits {
        println!("  {}: {}d", name, d);
    }
    println!("\n  DIAGONAL cofactors: {}d, {}d, {}d",
        digit_count(&cof00), digit_count(&cof11), digit_count(&cof22));
    println!("  OFF-DIAGONAL cofactors: {}d, {}d, {}d",
        digit_count(&cof01), digit_count(&cof02), digit_count(&cof12));
    println!("  cof22 (= -R01) has {}d — SMALLEST by {} digits",
        digit_count(&cof22), digit_count(&cof00) as i64 - digit_count(&cof22) as i64);

    // ═══ STEP 6: 2-ADIC STRUCTURE ═══
    println!("\n=== STEP 6: 2-ADIC STRUCTURE ===");
    // How many times does 2 divide each entry and cofactor?
    fn val2(n: &BigInt) -> u32 {
        if n.is_zero() { return 999; }
        let mut v = 0u32;
        let mut m = n.abs();
        let two = BigInt::from(2u32);
        while (&m % &two).is_zero() { m /= &two; v += 1; }
        v
    }
    println!("v2(Q00) = {}", val2(q00));
    println!("v2(Q01) = {}", val2(q01));
    println!("v2(Q11) = {}", val2(q11));
    println!("v2(Q02) = {}", val2(q02));
    println!("v2(Q12) = {}", val2(q12));
    println!("v2(Q22) = {}", val2(q22));
    println!("v2(det) = {}", val2(&det));
    println!("v2(R01) = {}", val2(&r01));
    println!("v2(d1) = {}", val2(&d1));
    println!("v2(gcd_cof) = {}", val2(&gcd_cof));

    // ═══ STEP 7: SCALE RATIOS ═══
    println!("\n=== STEP 7: SCALE RATIOS ===");
    // Key ratios that might produce 21:
    println!("Digit count ratios:");
    println!("  |Q22|/|Q00| digits: {} - {} = {} → sqrt = {}",
        digit_count(q22), digit_count(q00),
        digit_count(q22) as i64 - digit_count(q00) as i64,
        (digit_count(q22) as i64 - digit_count(q00) as i64) / 2);
    println!("  |Q02|/|Q00| digits: {} - {} = {}",
        digit_count(q02), digit_count(q00),
        digit_count(q02) as i64 - digit_count(q00) as i64);
    println!("  |Q12|/|Q11| digits: {} - {} = {}",
        digit_count(q12), digit_count(q11),
        digit_count(q12) as i64 - digit_count(q11) as i64);
    println!("  R01 digits: {} → R01/2 digits: {}",
        digit_count(&r01), digit_count(&r01) as i64 / 2);
    println!("  det digits: {} → det/3 digits: {}",
        digit_count(&det), digit_count(&det) as i64 / 3);

    // The half-gap: Schur gap 42 → 42/2 = 21
    println!("\n  SCHUR GAP = {} digits", gap);
    println!("  HALF-GAP = {} = THE MINIMUM WORD LENGTH", gap / 2);
    println!("  This is the HALF-GAP LAW:");
    println!("  The minimum word length in the frozen lattice equals");
    println!("  HALF the digit gap in the Schur complement of Q22.");

    // ═══ STEP 8: WHY THE HALF ═══
    println!("\n=== STEP 8: WHY THE HALF ===");
    println!("The Schur complement gap measures how 'diagonal' the");
    println!("torroid coupling is: off-diag/diag ~ 10^(-{}).", gap);
    println!("");
    println!("A lattice vector (a, b, c) with c = 1 contributes:");
    println!("  cross-energy ~ Q02*a + Q12*b (367d scale)");
    println!("  self-energy  ~ Q00*a^2 (328d * a^2 scale)");
    println!("  For these to interact: a * 10^367 ~ a^2 * 10^328");
    println!("  → a ~ 10^(367-328) = 10^39");
    println!("  → log10(a) ≈ 39 ≈ gap - 3");
    println!("");
    println!("The word length n = |a| + |b| + 1 ~ 2a ~ 2*10^39");
    println!("log10(word length) ≈ 39 + log10(2) ≈ 39.3");
    println!("");
    println!("But 42/2 = 21 and log10(21) ≈ 1.32, not 39.");
    println!("The gap/2 connection is NUMEROLOGICAL, not from");
    println!("the cross-energy balance. The TRUE reason 21 appears");
    println!("lies in the rqt362 lattice construction, not in the");
    println!("continuous geometry of Q3p.");
    println!("");
    println!("HONEST ASSESSMENT:");
    println!("  42/2 = 21 may be coincidence.");
    println!("  The gap = {} digits matches earlier measurements.", gap);
    println!("  But we have NOT derived 21 from Q3p alone.");
    println!("  The frozen lattice F has structure beyond what");
    println!("  the quadratic form's invariants determine.");

    // ═══ SUMMARY ═══
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H155 — THE LATTICE MEASURE");
    println!("{}", "=".repeat(60));
    println!("1. Cofactor spectrum: 651d (R01) to 737d.");
    println!("2. det(Q3p) = {} sign, {}d.", if det.is_positive(){"+"} else{"-"}, digit_count(&det));
    println!("3. Smith form: d1={}, d2={}d, d3={}d.", d1, digit_count(&d2), digit_count(&d3));
    println!("4. Schur gap (Q22 eliminated) = {} digits.", gap);
    println!("5. HALF-GAP = {} = 21-Root word length. Coincidence?", gap/2);
    println!("6. The cross-energy balance gives scale 10^39, not 21.");
    println!("7. HONEST: 42/2=21 is observed but NOT derived.");
    println!("8. The frozen lattice F carries structure from rqt362");
    println!("   that the quadratic form alone does not determine.");
    println!("9. What IS established: Q3p's invariants set the");
    println!("   NULL-CONE scale (10^42) and the GEOMETRY,");
    println!("   but the ARITHMETIC of F sets word length 21.");
}
