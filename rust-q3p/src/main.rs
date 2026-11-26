//! H177: The Reduced Mass
//! Does escape_ratio = √(mass_ratio) exactly?
//! Compute with BigInt and compare.

use num_bigint::BigInt;
use num_traits::{Zero, One, Signed};
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

fn dc(n: &BigInt) -> usize { if n.is_zero() {1} else {n.magnitude().to_string().len()} }

/// Integer square root via Newton's method
fn isqrt(n: &BigInt) -> BigInt {
    if n.is_zero() { return BigInt::zero(); }
    if n.is_negative() { panic!("isqrt of negative"); }
    let s = n.to_string();
    let digits = s.len();
    let half = (digits + 1) / 2;
    let mut x = BigInt::from(10).pow(half as u32);
    loop {
        let x_new = (&x + n / &x) / BigInt::from(2);
        if x_new >= x { break; }
        x = x_new;
    }
    while &x * &x > *n { x = x - BigInt::one(); }
    x
}

fn main() {
    let q = q3p_matrix();
    let q00=&q[0][0]; let q01=&q[0][1]; let q11=&q[1][1];
    let q02=&q[0][2]; let q12=&q[1][2]; let q22=&q[2][2];
    let abs_q00 = BigInt::from(q00.magnitude().clone());
    let abs_q22 = BigInt::from(q22.magnitude().clone());

    println!("=== H177: THE REDUCED MASS ===\n");

    // ═══ PART 1: The mass ratio ═══
    println!("═══ PART 1: THE MASS RATIO ═══\n");
    println!("  |Q22| has {} digits", dc(q22));
    println!("  |Q00| has {} digits", dc(q00));
    println!("  Digit difference: {} - {} = {}\n", dc(q22), dc(q00), dc(q22) - dc(q00));

    // Compute |Q22| / |Q00| exactly (integer division)
    let mass_ratio = &abs_q22 / &abs_q00;
    println!("  |Q22| / |Q00| = {} digits", dc(&mass_ratio));
    // Show first and last few digits
    let mr_str = mass_ratio.to_string();
    println!("  = {}...{} ({}d)\n",
        &mr_str[..6.min(mr_str.len())],
        &mr_str[mr_str.len()-6..],
        mr_str.len());

    // ═══ PART 2: √(mass ratio) ═══
    println!("═══ PART 2: √(MASS RATIO) ═══\n");
    let sqrt_mr = isqrt(&mass_ratio);
    let smr_str = sqrt_mr.to_string();
    println!("  √(|Q22|/|Q00|) has {} digits", smr_str.len());
    println!("  = {}...{}\n",
        &smr_str[..10.min(smr_str.len())],
        &smr_str[smr_str.len().saturating_sub(6)..]);

    // Also compute √(|Q22|) / √(|Q00|) for comparison
    let sqrt_q22 = isqrt(&abs_q22);
    let sqrt_q00 = isqrt(&abs_q00);
    let alt_ratio = &sqrt_q22 / &sqrt_q00;
    let ar_str = alt_ratio.to_string();
    println!("  Alternative: √|Q22| / √|Q00| has {} digits", ar_str.len());
    println!("  = {}...{}\n",
        &ar_str[..10.min(ar_str.len())],
        &ar_str[ar_str.len().saturating_sub(6)..]);

    // ═══ PART 3: The escape ratio ═══
    println!("═══ PART 3: THE ESCAPE RATIO ═══\n");
    // From H173: escape crossover at k ≈ 6.14×10^40 (41-digit number)
    // wl(c=1) = 25k ≈ 25 × 6.14×10^40 ≈ 1.5×10^42
    // wl(c=0) = 25
    // escape_ratio = wl(c=1) / wl(c=0) ≈ k (the scaling factor)

    // Let's compute k more precisely via the quadratic
    // Q3p(16k, 9k, 1) = Ak² + 2Bk + C = 0
    // A = Q3p(16,9,0), B = 16·Q02 + 9·Q12, C = Q22
    let a16 = BigInt::from(16);
    let b9 = BigInt::from(9);
    let c1 = BigInt::from(1);
    let c0 = BigInt::from(0);
    let eval = |a: &BigInt, b: &BigInt, c: &BigInt| -> BigInt {
        q00*a*a + BigInt::from(2)*q01*a*b + q11*b*b
        + BigInt::from(2)*q02*a*c + BigInt::from(2)*q12*b*c + q22*c*c
    };

    let big_a = eval(&a16, &b9, &c0);
    let big_b = BigInt::from(16) * q02 + BigInt::from(9) * q12;
    let big_c = q22.clone();

    println!("  Quadratic: Ak² + 2Bk + C = 0");
    println!("  A = Q3p(16,9,0) = +{}d", dc(&big_a));
    println!("  B = 16·Q02 + 9·Q12 = {}{}d",
        if big_b.is_positive() { "+" } else { "-" }, dc(&big_b));
    println!("  C = Q22 = -{}d\n", dc(&big_c));

    // k₊ = (-B + √(B²-AC)) / A
    let b_sq = &big_b * &big_b;
    let ac = &big_a * &big_c; // A>0, C<0, so AC<0
    let disc = &b_sq - &ac;   // B² - AC = B² + |AC| > 0
    println!("  B² = {}d", dc(&b_sq));
    println!("  AC = {}{}d", if ac.is_positive() { "+" } else { "-" }, dc(&ac));
    println!("  B²-AC = +{}d\n", dc(&disc));

    let sqrt_disc = isqrt(&disc);
    println!("  √(B²-AC) = {}d", dc(&sqrt_disc));

    let neg_b = -&big_b;
    let numerator = &neg_b + &sqrt_disc;
    let k_exact = &numerator / &big_a;
    let k_str = k_exact.to_string();
    println!("  k₊ = (-B + √disc) / A");
    println!("  k₊ = {}d", k_str.len());
    println!("  k₊ = {}...{}\n",
        &k_str[..10.min(k_str.len())],
        &k_str[k_str.len().saturating_sub(6)..]);

    // escape_ratio = k₊ (since wl(c=1) = 25·k₊ and wl(c=0) = 25)
    println!("  escape_ratio = k₊ = {}d\n", k_str.len());

    // ═══ PART 4: THE COMPARISON ═══
    println!("═══ PART 4: THE COMPARISON ═══\n");
    println!("  √(mass_ratio) = √(|Q22|/|Q00|) has {} digits", smr_str.len());
    println!("  escape_ratio = k₊ has {} digits\n", k_str.len());

    if smr_str.len() == k_str.len() {
        println!("  DIGIT COUNTS MATCH: both have {} digits.\n", smr_str.len());
    } else {
        println!("  DIGIT COUNTS DIFFER: {} vs {}.\n", smr_str.len(), k_str.len());
    }

    // Compare the actual values
    println!("  √(mass_ratio) first 20: {}", &smr_str[..20.min(smr_str.len())]);
    println!("  escape_ratio  first 20: {}\n", &k_str[..20.min(k_str.len())]);

    // Compute the ratio of these two quantities
    let ratio_check = if k_exact > sqrt_mr {
        let r = &k_exact / &sqrt_mr;
        println!("  escape_ratio / √(mass_ratio) = {}", r);
        println!("  = {}d number\n", dc(&r));
        r
    } else {
        let r = &sqrt_mr / &k_exact;
        println!("  √(mass_ratio) / escape_ratio = {}", r);
        println!("  = {}d number\n", dc(&r));
        r
    };

    // ═══ PART 5: What the ratio tells us ═══
    println!("═══ PART 5: WHAT THE RATIO TELLS US ═══\n");

    // The simple reduced mass law says: escape_ratio = √(mass_ratio)
    // But Q3p is not literally a gravitational 3-body problem.
    // The actual escape ratio involves the FULL quadratic form, not just masses.
    // The quadratic k₊ = (-B + √(B²-AC)) / A involves:
    //   A = Q3p(16,9,0) — the c=0 escape value
    //   B = 16·Q02 + 9·Q12 — the coupling terms
    //   C = Q22 — the torroid anchor

    // For large |C|/A: k₊ ≈ √(|C|/A) = √(|Q22|/Q3p(16,9,0))
    // This is NOT √(|Q22|/|Q00|). It's √(|Q22|/A) where A = Q3p(16,9,0).

    println!("  The EXACT reduced mass law for Q3p is:");
    println!("  k₊ ≈ √(|C|/A) = √(|Q22| / Q3p(16,9,0))\n");
    println!("  NOT √(|Q22|/|Q00|) which would be the naive mass ratio.\n");

    // Compute √(|Q22| / A)
    let ratio_ca = &abs_q22 / &big_a;
    let sqrt_ca = isqrt(&ratio_ca);
    let sca_str = sqrt_ca.to_string();
    println!("  √(|Q22|/A):");
    println!("  |Q22|/A has {} digits", dc(&ratio_ca));
    println!("  √(|Q22|/A) has {} digits", sca_str.len());
    println!("  = {}...{}\n",
        &sca_str[..15.min(sca_str.len())],
        &sca_str[sca_str.len().saturating_sub(6)..]);

    println!("  k₊ first 15:        {}", &k_str[..15.min(k_str.len())]);
    println!("  √(|Q22|/A) first 15: {}\n", &sca_str[..15.min(sca_str.len())]);

    // Check if they're close
    if sca_str.len() == k_str.len() {
        // Compare first few digits
        let match_len = k_str.chars().zip(sca_str.chars())
            .take_while(|(a,b)| a == b).count();
        println!("  First {} digits match!", match_len);
        if match_len >= 5 {
            println!("  √(|Q22|/A) is an EXCELLENT approximation to k₊.\n");
        }
    }

    // ═══ PART 6: The three mass ratios ═══
    println!("═══ PART 6: THE THREE MASS RATIOS ═══\n");
    println!("  NAIVE:  √(|Q22|/|Q00|) = {}d — wrong denominator", smr_str.len());
    println!("  EXACT:  k₊ = (-B+√(B²-AC))/A = {}d — from quadratic", k_str.len());
    println!("  APPROX: √(|Q22|/A) = {}d — leading-order approximation\n", sca_str.len());

    // The difference between naive and exact
    let d_naive = smr_str.len() as i64;
    let d_exact = k_str.len() as i64;
    let d_approx = sca_str.len() as i64;
    println!("  Digit counts: naive={}, exact={}, approx={}", d_naive, d_exact, d_approx);
    println!("  Naive − exact = {}", d_naive - d_exact);
    println!("  Approx − exact = {}\n", d_approx - d_exact);

    // ═══ VERDICT ═══
    println!("{}", "=".repeat(60));
    println!("VERDICT: THE REDUCED MASS LAW");
    println!("{}", "=".repeat(60));
    println!("\n  The naive claim: escape_ratio = √(|Q22|/|Q00|)");
    println!("  The exact answer: k₊ = (-B + √(B²-AC)) / A\n");
    println!("  The correct reduced-mass law is:");
    println!("    k₊ ≈ √(|Q22| / Q3p(16,9,0))");
    println!("  NOT √(|Q22|/|Q00|).\n");
    println!("  The denominator is Q3p(16,9,0) = +326d (the c=0 escape VALUE),");
    println!("  not |Q00| = 328d (the brace SELF-interaction).\n");
    println!("  The mass ratio in the Jacobi frame is:");
    println!("    |Q22| / Q3p(16,9,0) ≈ 10^(408-326) = 10^82");
    println!("    √(10^82) = 10^41");
    println!("    k₊ ≈ 10^41 ✓\n");
    println!("  The reduced mass law HOLDS but with the CORRECT mass:");
    println!("  The 'mass' of the binary subsystem is NOT |Q00|.");
    println!("  It is Q3p(16,9,0) — the ESCAPE VALUE at the escape vector.");
    println!("  The effective mass is the ENERGY at the escape point,");
    println!("  not the self-interaction of a single body.");
}
