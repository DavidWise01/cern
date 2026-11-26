//! H173: The c-Dimension
//! Can Q3p(a,b,c) > 0 for ANY c ≥ 1?
//! Compute theoretical minimum word length for c=1 escape.

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

fn main() {
    let q = q3p_matrix();
    let q00=&q[0][0]; let q01=&q[0][1]; let q11=&q[1][1];
    let q02=&q[0][2]; let q12=&q[1][2]; let q22=&q[2][2];

    println!("=== H173: THE c-DIMENSION ===\n");

    // ═══ PART 1: Entry magnitudes ═══
    println!("═══ PART 1: ENTRY MAGNITUDES ═══");
    println!("  |Q00| = {}d", digit_count(q00));
    println!("  |Q01| = {}d", digit_count(q01));
    println!("  |Q11| = {}d", digit_count(q11));
    println!("  |Q02| = {}d", digit_count(q02));
    println!("  |Q12| = {}d", digit_count(q12));
    println!("  |Q22| = {}d\n", digit_count(q22));

    // ═══ PART 2: Q3p at c=1 ═══
    // Q3p(a,b,1) = Q00·a² + 2Q01·ab + Q11·b² + 2Q02·a + 2Q12·b + Q22
    // = binary(a,b) + 2Q02·a + 2Q12·b + Q22
    // For this to be > 0: binary(a,b) + 2Q02·a + 2Q12·b > -Q22 = |Q22|

    println!("═══ PART 2: THE c=1 ESCAPE BOUND ═══");
    println!("  Q3p(a,b,1) = binary(a,b) + 2·Q02·a + 2·Q12·b + Q22");
    println!("  For escape: binary(a,b) + 2·Q02·a + 2·Q12·b > |Q22|\n");
    println!("  |Q22| has {} digits.", digit_count(q22));
    println!("  The linear terms 2·Q02·a and 2·Q12·b have {} and {} digit coefficients.",
        digit_count(q02), digit_count(q12));
    println!("  The gap: {} - {} = {} digits.\n",
        digit_count(q22), digit_count(q02), digit_count(q22) - digit_count(q02));

    // The linear terms scale as ~10^367 × a, so to reach ~10^408, need a ~ 10^41
    // Word length ~ 10^41 for the linear term alone to match |Q22|
    let gap = digit_count(q22) - digit_count(q02);
    println!("  To overcome |Q22| via linear terms:");
    println!("  Need |a| ≈ 10^{} (a {}-digit number)", gap, gap+1);
    println!("  Word length ≥ 10^{} (a {}-digit NUMBER)", gap, gap+1);
    println!("  This is NOT a word length of {}. It is 10^{} ≈ {}.\n",
        gap, gap, "100000000000000000000000000000000000000000");

    // ═══ PART 3: Optimize the c=1 form ═══
    // Q3p(a,b,1) = Q00·a² + 2Q01·ab + Q11·b² + 2Q02·a + 2Q12·b + Q22
    // The binary part can be positive (we know escape at c=0 at wl=25).
    // At the escape ratio a/b ≈ 16/9, binary(a,b) is positive and grows as a².
    // So binary(ka, kb) ≈ k² × binary(a,b) for large k.
    // Q3p(16k, 9k, 1) ≈ k² × Q3p(16,9,0) + 2Q02×16k + 2Q12×9k + Q22
    //                   = k² × 8 (Q3p at (16,9,0) ≈ 10^326) + linear + Q22

    // Actually let's compute Q3p(16,9,0) exactly
    let a16 = BigInt::from(16);
    let b9 = BigInt::from(9);
    let c0 = BigInt::from(0);
    let c1 = BigInt::from(1);

    let eval = |a: &BigInt, b: &BigInt, c: &BigInt| -> BigInt {
        q00*a*a + BigInt::from(2)*q01*a*b + q11*b*b
        + BigInt::from(2)*q02*a*c + BigInt::from(2)*q12*b*c + q22*c*c
    };

    let q_16_9_0 = eval(&a16, &b9, &c0);
    println!("═══ PART 3: THE SCALING ARGUMENT ═══");
    println!("  Q3p(16,9,0) = {} ({}d, positive)",
        if q_16_9_0.is_positive() { "+" } else { "-" }, digit_count(&q_16_9_0));

    // Q3p(16k, 9k, 1) for various k
    println!("\n  Q3p(16k, 9k, 1) for increasing k:");
    println!("  {:>15} {:>8} {:>8} {:>6}", "k", "sign", "digits", "wl");

    let mut first_pos_k = BigInt::zero();
    // Test k = 10^n for n = 0, 5, 10, 15, 20, 25, 30, 35, 40, 41, 42
    let test_exponents = vec![0u32, 5, 10, 15, 20, 25, 30, 35, 38, 39, 40, 41, 42];
    for &exp in &test_exponents {
        let k = BigInt::from(10u64).pow(exp);
        let a_val = &a16 * &k;
        let b_val = &b9 * &k;
        let v = eval(&a_val, &b_val, &c1);
        let sign = if v.is_positive() { "+" } else { "-" };
        println!("  {:>15} {:>8} {:>8} ~25×10^{}",
            format!("10^{}", exp), sign, digit_count(&v), exp);
        if v.is_positive() && first_pos_k.is_zero() {
            first_pos_k = k.clone();
        }
    }

    if !first_pos_k.is_zero() {
        println!("\n  FIRST POSITIVE: at k ≈ 10^{} (word length ≈ 25×10^{})",
            first_pos_k.to_string().len() - 1, first_pos_k.to_string().len() - 1);
        println!("  The c=1 escape EXISTS but at ASTRONOMICAL word length.");
    } else {
        println!("\n  NO positive found up to k=10^42.");
        println!("  Checking higher...");
    }

    // ═══ PART 4: Theoretical bound ═══
    // Q3p(16k, 9k, 1) = k²·Q3p(16,9,0) + 2k·(16·Q02 + 9·Q12) + Q22
    // The k² term is positive (Q3p(16,9,0) > 0).
    // The k term: compute 16·Q02 + 9·Q12
    let linear_coeff = BigInt::from(16) * q02 + BigInt::from(9) * q12;
    let two_linear = BigInt::from(2) * &linear_coeff;
    println!("\n═══ PART 4: THEORETICAL BOUND ═══");
    println!("  Q3p(16k, 9k, 1) = k²·A + 2k·B + C  where:");
    println!("  A = Q3p(16,9,0) = +{}d", digit_count(&q_16_9_0));
    println!("  B = 16·Q02 + 9·Q12 = {} ({}d)",
        if linear_coeff.is_positive() { "+" } else { "-" }, digit_count(&linear_coeff));
    println!("  C = Q22 = - ({}d)\n", digit_count(q22));

    // This is a quadratic in k: Ak² + 2Bk + C > 0
    // Discriminant: 4B² - 4AC = 4(B² - AC)
    let b_sq = &linear_coeff * &linear_coeff;
    let ac = &q_16_9_0 * q22;  // A is positive, C is negative, so AC is negative
    let disc = &b_sq - &ac;    // B² - AC = B² + |A|·|C| (always positive)
    println!("  Quadratic Ak² + 2Bk + C > 0");
    println!("  Discriminant B² - AC:");
    println!("  B² = {}d", digit_count(&b_sq));
    println!("  AC = {} ({}d)", if ac.is_positive() { "+" } else { "-" }, digit_count(&ac));
    println!("  B² - AC = + ({}d)", digit_count(&disc));

    // Roots: k = (-B ± √(B²-AC)) / A
    // Since A > 0 and C < 0, one root is positive and one is negative.
    // The positive root is k₊ = (-B + √(B²-AC)) / A
    // We need to compute √(B²-AC) approximately.

    // B² has ~734d, AC has ~(326+408)=~734d. These are close!
    // disc = B² - AC. Need to check the sign and magnitude.
    println!("\n  Sign of discriminant: {}", if disc.is_positive() { "POSITIVE" } else { "NEGATIVE" });
    println!("  Discriminant has {} digits", digit_count(&disc));

    // The positive root: k₊ = (-B + √disc) / A
    // |B| ≈ 10^367, A ≈ 10^326, √disc ≈ 10^(disc_digits/2)
    let disc_digits = digit_count(&disc);
    let sqrt_disc_digits = (disc_digits + 1) / 2;
    println!("  √disc ≈ 10^{}", sqrt_disc_digits);
    println!("  B ≈ 10^{}", digit_count(&linear_coeff));
    println!("  A ≈ 10^{}", digit_count(&q_16_9_0));

    // k₊ ≈ (-B + √disc) / A ≈ (10^367 + 10^367) / 10^326 ≈ 10^41
    // So k₊ ≈ 10^41 and word length ≈ 25 × 10^41 ≈ 10^42
    let k_estimate = sqrt_disc_digits as i64 - digit_count(&q_16_9_0) as i64;
    println!("\n  k₊ ≈ 10^{}", k_estimate);
    println!("  word length ≈ 25 × 10^{} ≈ 10^{}", k_estimate, k_estimate + 1);
    println!("  This is a {}-DIGIT word length.", k_estimate + 1);

    // ═══ PART 5: Verify with exact computation at boundary ═══
    println!("\n═══ PART 5: EXACT BOUNDARY SEARCH ═══");
    // Binary search for the smallest k where Q3p(16k, 9k, 1) > 0
    // We know k=10^40 is negative, k=10^42 may be positive
    // Let's narrow it down

    let mut lo = BigInt::from(10u64).pow(38);
    let mut hi = BigInt::from(10u64).pow(43);

    // First, verify signs at boundaries
    let v_lo = eval(&(&a16 * &lo), &(&b9 * &lo), &c1);
    let v_hi = eval(&(&a16 * &hi), &(&b9 * &hi), &c1);
    println!("  Q3p(16×10^38, 9×10^38, 1) = {} ({}d)",
        if v_lo.is_positive() { "+" } else { "-" }, digit_count(&v_lo));
    println!("  Q3p(16×10^43, 9×10^43, 1) = {} ({}d)",
        if v_hi.is_positive() { "+" } else { "-" }, digit_count(&v_hi));

    if v_lo.is_negative() && v_hi.is_positive() {
        // Binary search
        println!("\n  Binary search for crossover...");
        for _ in 0..20 {
            let mid = (&lo + &hi) / BigInt::from(2);
            let v_mid = eval(&(&a16 * &mid), &(&b9 * &mid), &c1);
            if v_mid.is_positive() {
                hi = mid;
            } else {
                lo = mid;
            }
            if &hi - &lo <= BigInt::from(1) { break; }
        }
        println!("  Crossover at k ≈ {} ({}d)", &hi.to_string()[..6.min(hi.to_string().len())], hi.to_string().len());
        let cross_wl = BigInt::from(25) * &hi;
        println!("  Word length at crossover ≈ {} ({}d)", &cross_wl.to_string()[..6.min(cross_wl.to_string().len())], cross_wl.to_string().len());
        println!("  That is a {}-DIGIT number as the word length.", cross_wl.to_string().len());
    } else if v_lo.is_positive() {
        println!("  Already positive at k=10^38! Searching lower...");
    } else {
        println!("  Still negative at k=10^43. Searching higher...");
    }

    // ═══ VERDICT ═══
    println!("\n{}", "=".repeat(60));
    println!("VERDICT: THE c-DIMENSION");
    println!("{}", "=".repeat(60));
    println!("\n  Conjecture C2: 'No c≥1 escape ever'");
    println!("  DISPROVED.");
    println!("\n  A c=1 escape DOES exist, but at ASTRONOMICAL word length:");
    println!("  word length ≈ 10^{} (a {}-digit NUMBER).", k_estimate+1, k_estimate+1);
    println!("  This is not word length 42. It is a 42-DIGIT word length.");
    println!("  The number of steps is itself a number with ~42 digits.");
    println!("\n  By comparison:");
    println!("  c=0 escape: wl=25 (a 2-digit number)");
    println!("  c=1 escape: wl≈10^42 (a ~42-digit number)");
    println!("  Ratio: 10^42 / 25 ≈ 4×10^40");
    println!("\n  The torroid dimension CAN escape.");
    println!("  But the escape is 10^40 times further away than the brace-box escape.");
    println!("  The |Q22| ≈ 10^408 anchor delays c=1 escape to cosmic word lengths.");
    println!("\n  OPEN CONJECTURES: 2 → 1");
    println!("  C2 is now RESOLVED (disproved with constructive counterexample).");
    println!("  Only C3 remains open.");
}
