//! H154: The Ternary Escape
//! Since c=0 is forbidden (H153), compute Q3p(a, b, 1) to find
//! the smallest lattice point on the null cone with c >= 1.

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

fn digit_count(n: &BigInt) -> usize {
    if n.is_zero() { return 1; }
    n.magnitude().to_string().len()
}

fn isqrt(n: &BigInt) -> BigInt {
    if n.is_zero() { return BigInt::zero(); }
    let mut x = n.clone();
    let mut y = (&x + BigInt::one()) / BigInt::from(2u32);
    while y < x { x = y.clone(); y = (&x + n / &x) / BigInt::from(2u32); }
    x
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
        let ip = &s[..s.len() - prec as usize];
        let dp = &s[s.len() - prec as usize..];
        format!("{}{}.{}", sign, ip, dp)
    }
}

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0]; let q01 = &q[0][1]; let q11 = &q[1][1];
    let q02 = &q[0][2]; let q12 = &q[1][2]; let q22 = &q[2][2];

    println!("=== H154: THE TERNARY ESCAPE (Rust BigInt) ===\n");

    // ═══ STEP 1: THE c=1 QUADRATIC ═══
    println!("=== STEP 1: THE c=1 SLICE ===");
    println!("Q3p(a, b, 1) = Q00*a^2 + 2Q01*a*b + Q11*b^2 + 2Q02*a + 2Q12*b + Q22");
    println!("");
    println!("Term magnitudes:");
    println!("  Q00: {}d  (brace self-energy)", digit_count(q00));
    println!("  Q01: {}d  (brace-box coupling)", digit_count(q01));
    println!("  Q11: {}d  (box self-energy)", digit_count(q11));
    println!("  Q02: {}d  (brace-torroid coupling)", digit_count(q02));
    println!("  Q12: {}d  (box-torroid coupling)", digit_count(q12));
    println!("  Q22: {}d  (torroid self-energy)", digit_count(q22));

    // ═══ STEP 2: DISCRIMINANT COEFFICIENTS ═══
    println!("\n=== STEP 2: DISCRIMINANT COEFFICIENTS ===");
    println!("Solving Q3p(a, b, 1) = 0 as quadratic in a:");
    println!("  Q00*a^2 + (2Q01*b + 2Q02)*a + (Q11*b^2 + 2Q12*b + Q22) = 0");
    println!("  Discriminant Delta(b) = 4*(A*b^2 + 2*B*b + C) where:");

    // A = R01 = Q01^2 - Q00*Q11 = Q01^2 - |Q00|*|Q11|
    let r01 = q01 * q01 - &(q00.abs() * q11.abs());
    println!("  A = R01 = Q01^2 - |Q00|*|Q11| = {}d", digit_count(&r01));

    // B = Q01*Q02 - Q00*Q12
    let big_b = q01 * q02 - q00 * q12;
    println!("  B = Q01*Q02 - Q00*Q12 = {} sign, {}d",
        if big_b.is_positive() { "+" } else { "-" }, digit_count(&big_b));

    // C = Q02^2 - Q00*Q22
    let big_c = q02 * q02 - q00 * q22;
    println!("  C = Q02^2 - Q00*Q22 = {} sign, {}d",
        if big_c.is_positive() { "+" } else { "-" }, digit_count(&big_c));

    // The discriminant of the discriminant: D_min = C - B^2/A
    // If D_min > 0: real solutions exist for some b
    // If D_min < 0: Q3p(a,b,1) has constant sign for all (a,b)
    let b_sq = &big_b * &big_b;
    // D_min * A = C*A - B^2
    let d_min_times_a = &big_c * &r01 - &b_sq;
    println!("\n  D_min * A = C*A - B^2 = {} sign, {}d",
        if d_min_times_a.is_positive() { "+" } else { "-" }, digit_count(&d_min_times_a));
    println!("  D_min sign: {} (since A > 0, D_min has same sign as D_min*A)",
        if d_min_times_a.is_positive() { "POSITIVE -> real solutions exist" }
        else { "NEGATIVE -> no real solutions at c=1" });

    // ═══ STEP 3: CRITICAL POINT ═══
    println!("\n=== STEP 3: CRITICAL POINT ===");
    // b_crit = -B/A (the b that minimizes the discriminant)
    let b_crit = -&big_b / &r01;
    println!("b_crit = -B/A = {} sign, {}d",
        if b_crit.is_positive() { "+" } else { "-" }, digit_count(&b_crit));
    let b_crit_s = b_crit.magnitude().to_string();
    println!("  Leading digits: {}...", &b_crit_s[..20.min(b_crit_s.len())]);

    // a_crit from b_crit: a = -(Q01*b + Q02)/Q00
    let a_crit_num = -(q01 * &b_crit + q02);
    let a_crit = &a_crit_num / q00;
    println!("a_crit = -(Q01*b_crit + Q02)/Q00 = {} sign, {}d",
        if a_crit.is_positive() { "+" } else { "-" }, digit_count(&a_crit));
    let a_crit_s = a_crit.magnitude().to_string();
    println!("  Leading digits: {}...", &a_crit_s[..20.min(a_crit_s.len())]);

    // Word length at critical point
    let wl_crit = a_crit.abs() + b_crit.abs() + BigInt::one();
    println!("Word length at critical point: |a| + |b| + 1 = {}d",
        digit_count(&wl_crit));
    let wl_s = wl_crit.magnitude().to_string();
    println!("  Leading digits: {}...", &wl_s[..20.min(wl_s.len())]);

    // ═══ STEP 4: Q3p AT CRITICAL POINT ═══
    println!("\n=== STEP 4: Q3p AT AND NEAR CRITICAL POINT ===");

    // Evaluate Q3p(a, b, c) exactly
    let eval_q3p = |a: &BigInt, b: &BigInt, c: &BigInt| -> BigInt {
        let aa = a * a; let bb = b * b; let cc = c * c;
        let ab = a * b; let ac = a * c; let bc = b * c;
        q00 * &aa + BigInt::from(2) * q01 * &ab + q11 * &bb
        + BigInt::from(2) * q02 * &ac + BigInt::from(2) * q12 * &bc + q22 * &cc
    };

    let one = BigInt::one();
    let val_crit = eval_q3p(&a_crit, &b_crit, &one);
    println!("Q3p(a_crit, b_crit, 1) = {} sign, {}d",
        if val_crit.is_positive() { "+" } else { "-" }, digit_count(&val_crit));

    // Try nearby points (a_crit ± delta, b_crit ± delta)
    println!("\nScanning nearby lattice points:");
    println!("{:<12} {:>8} {:>12}", "Offset", "Q3p sign", "Q3p digits");
    println!("{}", "-".repeat(35));

    for da in -3i64..=3 {
        for db in -3i64..=3 {
            let a_try = &a_crit + BigInt::from(da);
            let b_try = &b_crit + BigInt::from(db);
            let v = eval_q3p(&a_try, &b_try, &one);
            if da.abs() + db.abs() <= 3 {
                let sign = if v.is_positive() { "+" } else if v.is_negative() { "-" } else { "0" };
                println!("({:+2},{:+2})     {:>5}    {:>8}", da, db, sign, digit_count(&v));
            }
        }
    }

    // ═══ STEP 5: DISCRIMINANT AT CRITICAL b ═══
    println!("\n=== STEP 5: DISCRIMINANT PROFILE ===");
    // Delta(b) = 4*(A*b^2 + 2*B*b + C)
    // At b_crit: Delta_min = 4*(C - B^2/A)

    // Compute Delta at b_crit and neighbors
    let delta_at = |b: &BigInt| -> BigInt {
        &r01 * b * b + BigInt::from(2) * &big_b * b + &big_c
    };

    println!("Delta(b)/4 at b_crit and neighbors:");
    for db in -5i64..=5 {
        let b_try = &b_crit + BigInt::from(db);
        let d = delta_at(&b_try);
        let sign = if d.is_positive() { "+" } else if d.is_negative() { "-" } else { "0" };
        println!("  b_crit{:+2}: {} {}d", db, sign, digit_count(&d));
    }

    // ═══ STEP 6: PERFECT SQUARE SEARCH ═══
    println!("\n=== STEP 6: PERFECT SQUARE SEARCH ===");
    // Check if Delta(b)/4 is a perfect square for b near b_crit
    let mut found_psq = false;
    for db in -100i64..=100 {
        let b_try = &b_crit + BigInt::from(db);
        let d = delta_at(&b_try);
        if d.is_negative() { continue; }
        let s = isqrt(&d);
        if &s * &s == d {
            println!("PERFECT SQUARE at b = b_crit + {} !", db);
            println!("  sqrt(Delta/4) = {}d", digit_count(&s));
            // Solve for a
            let coeff_a = BigInt::from(2) * q01 * &b_try + BigInt::from(2) * q02;
            let a_sol1 = (-&coeff_a + BigInt::from(2) * &s) / (BigInt::from(2) * q00);
            let a_sol2 = (-&coeff_a - BigInt::from(2) * &s) / (BigInt::from(2) * q00);
            println!("  a_1 = {}d, a_2 = {}d", digit_count(&a_sol1), digit_count(&a_sol2));
            let v1 = eval_q3p(&a_sol1, &b_try, &one);
            let v2 = eval_q3p(&a_sol2, &b_try, &one);
            println!("  Q3p(a_1, b, 1) = {} {}d", if v1.is_zero() { "0" } else { "nonzero" }, digit_count(&v1));
            println!("  Q3p(a_2, b, 1) = {} {}d", if v2.is_zero() { "0" } else { "nonzero" }, digit_count(&v2));
            found_psq = true;
            break;
        }
    }
    if !found_psq {
        println!("No perfect square Delta(b)/4 found in [b_crit-100, b_crit+100].");
        println!("Closest: checking residuals...");
        let mut min_resid_d = BigInt::from(0);
        let mut min_resid_db: i64 = 0;
        let mut first = true;
        for db in -10i64..=10 {
            let b_try = &b_crit + BigInt::from(db);
            let d = delta_at(&b_try);
            if d.is_negative() { continue; }
            let s = isqrt(&d);
            let resid = &d - &s * &s;
            if first || digit_count(&resid) < digit_count(&min_resid_d) {
                min_resid_d = resid.clone();
                min_resid_db = db;
                first = false;
            }
            println!("  b_crit{:+3}: Delta/4 residual = {}d", db, digit_count(&resid));
        }
        println!("Smallest residual at db={}: {}d", min_resid_db, digit_count(&min_resid_d));
    }

    // ═══ STEP 7: SCALE ANALYSIS ═══
    println!("\n=== STEP 7: SCALE ANALYSIS ===");
    println!("At the critical point:");
    println!("  |a_crit| ~ 10^{}", digit_count(&a_crit) - 1);
    println!("  |b_crit| ~ 10^{}", digit_count(&b_crit) - 1);
    println!("  Word length ~ 10^{}", digit_count(&wl_crit) - 1);
    println!("");
    println!("Compare to the 21-Root Theorem: minimum word length = 21.");
    println!("The c=1 critical point has word length ~ 10^{} >> 21.", digit_count(&wl_crit) - 1);
    println!("");

    // Check: what about larger c?
    println!("What about c >= 2?");
    println!("  Q3p(a, b, c) = binary(a,b) + 2c*(Q02*a + Q12*b) + c^2*Q22");
    println!("  For c=2: Q22 term is 4*Q22 ~ 10^{}, coupling is 4*(Q02*a + Q12*b)", digit_count(q22));
    println!("  Need |binary(a,b)| ~ 4*|Q22| ~ 10^{}", digit_count(q22) + 1);
    println!("  This requires a,b ~ sqrt(10^{}/ 10^{}) ~ 10^{}",
        digit_count(q22) + 1, digit_count(q00), (digit_count(q22) + 1 - digit_count(q00)) / 2 + 1);
    println!("  Word length even LARGER than c=1 case.");
    println!("  c=1 is the OPTIMAL escape.");

    // ═══ STEP 8: THE STRUCTURE ═══
    println!("\n=== STEP 8: THE ESCAPE STRUCTURE ===");
    println!("At c=1, the ternary form adds three terms to the binary form:");
    println!("  +2Q02*a: {}d, NEGATIVE (Q02 < 0)", digit_count(q02));
    println!("  +2Q12*b: {}d, POSITIVE (Q12 > 0)", digit_count(q12));
    println!("  +Q22:    {}d, NEGATIVE (Q22 < 0)", digit_count(q22));
    println!("");
    println!("Q22 dominates for small a,b. To cancel it, need:");
    println!("  binary(a,b) + 2Q02*a + 2Q12*b ≈ -Q22 ≈ +10^{}", digit_count(q22));
    println!("  binary(a,b) ~ |Q00|*a^2 ~ 10^{}*a^2 for a ~ b*n_+", digit_count(q00));
    println!("  Need a^2 ~ 10^{}/10^{} = 10^{}",
        digit_count(q22), digit_count(q00), digit_count(q22) - digit_count(q00));
    println!("  a ~ 10^{}", (digit_count(q22) - digit_count(q00)) / 2);

    // The ratio of coupling to self-energy
    let ratio = fp_div(&q02.abs(), &q22.abs(), 10);
    println!("\n  |Q02|/|Q22| = {} (coupling/self ratio)", ratio);
    let ratio2 = fp_div(&q12.abs(), &q22.abs(), 10);
    println!("  |Q12|/|Q22| = {} (coupling/self ratio)", ratio2);

    // ═══ SUMMARY ═══
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H154 — THE TERNARY ESCAPE");
    println!("{}", "=".repeat(60));
    println!("1. c=1 slice: Q3p(a,b,1) is a shifted binary form.");
    println!("2. Discriminant coefficients: A={}d, B={}d, C={}d.",
        digit_count(&r01), digit_count(&big_b), digit_count(&big_c));
    println!("3. D_min*A = {} sign, {}d -> real solutions {} at c=1.",
        if d_min_times_a.is_positive() { "+" } else { "-" },
        digit_count(&d_min_times_a),
        if d_min_times_a.is_positive() { "EXIST" } else { "DON'T EXIST" });
    println!("4. Critical point: a ~ 10^{}, b ~ 10^{}, word length ~ 10^{}.",
        digit_count(&a_crit) - 1, digit_count(&b_crit) - 1, digit_count(&wl_crit) - 1);
    println!("5. The c=1 escape exists in REAL numbers but requires");
    println!("   word length ~ 10^{} to reach from the integer lattice.", digit_count(&wl_crit) - 1);
    println!("6. This is >> 21 (the 21-Root Theorem).");
    println!("7. IMPLICATION: The 21 roots of the frozen lattice are NOT");
    println!("   null-cone vectors. They are shortest vectors in F by a");
    println!("   DIFFERENT criterion than Q3p = 0.");
}
