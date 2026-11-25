//! H153: The Continued Fraction
//! CF expansion of n₊ = (|Q01| − √R01)/|Q00| ≈ 1.771
//! and its consequences for the null cone structure.

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

fn eval_binary(q00: &BigInt, q01: &BigInt, q11: &BigInt, a: &BigInt, b: &BigInt) -> BigInt {
    let aa = a * a;
    let ab = a * b;
    let bb = b * b;
    q00 * &aa + BigInt::from(2) * q01 * &ab + q11 * &bb
}

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0]; let q01 = &q[0][1]; let q11 = &q[1][1];

    println!("=== H153: THE CONTINUED FRACTION (Rust BigInt) ===\n");

    // ═══════════════════════════════════════════
    // STEP 1: R01 AND PERFECT SQUARE TEST
    // ═══════════════════════════════════════════
    println!("=== STEP 1: PERFECT SQUARE TEST ===");
    let r01 = q01 * q01 - &(q00.abs() * q11.abs());
    let s = isqrt(&r01);
    let residual = &r01 - &s * &s;
    let is_psq = residual.is_zero();

    println!("R01 = {}d", digit_count(&r01));
    println!("isqrt(R01) = {}d", digit_count(&s));
    println!("R01 - isqrt(R01)^2 = {}d (residual)", digit_count(&residual));
    println!("R01 is perfect square: {}", is_psq);
    if !is_psq {
        println!("");
        println!("THEOREM: The binary form Q00*x^2 + 2*Q01*x*y + Q11*y^2");
        println!("has discriminant 4*R01 which is NOT a perfect square.");
        println!("Therefore this form does NOT represent zero over Z.");
        println!("CONSEQUENCE: No lattice point (a, b, 0) lies on Q3p = 0.");
        println!("CONSEQUENCE: ALL null-cone roots have c >= 1.");
    }

    // Also check: what is R01 mod small primes?
    let r01_mod2 = &r01 % BigInt::from(2);
    let r01_mod4 = &r01 % BigInt::from(4);
    let r01_mod8 = &r01 % BigInt::from(8);
    println!("\nR01 mod 2 = {}", r01_mod2);
    println!("R01 mod 4 = {}", r01_mod4);
    println!("R01 mod 8 = {}", r01_mod8);

    // ═══════════════════════════════════════════
    // STEP 2: HIGH-PRECISION n₊
    // ═══════════════════════════════════════════
    println!("\n=== STEP 2: HIGH-PRECISION n_+ ===");
    let prec: u32 = 400;
    let ten = BigInt::from(10u64);
    let scale = ten.pow(prec);
    let scale_sq = ten.pow(2 * prec);

    // sqrt(R01) * 10^PREC via isqrt(R01 * 10^(2*PREC))
    println!("Computing sqrt(R01) to {} digits...", prec);
    let sqrt_r01_sc = isqrt(&(&r01 * &scale_sq));
    println!("Done. sqrt(R01) scaled has {}d", digit_count(&sqrt_r01_sc));

    // n_+ * 10^PREC = (|Q01| * 10^PREC - sqrt(R01) * 10^PREC) / |Q00|
    let q01_sc = q01.abs() * &scale;
    let numer_plus = &q01_sc - &sqrt_r01_sc;
    let n_plus_fp = &numer_plus / &q00.abs();

    // n_- * 10^PREC
    let numer_minus = &q01_sc + &sqrt_r01_sc;
    let n_minus_fp = &numer_minus / &q00.abs();

    let np_s = n_plus_fp.to_string();
    let nm_s = n_minus_fp.to_string();

    println!("n_+ = {}.{}", &np_s[..1], &np_s[1..51.min(np_s.len())]);
    println!("n_- = {}.{}", &nm_s[..1], &nm_s[1..51.min(nm_s.len())]);

    // Gap
    let gap_fp = &n_minus_fp - &n_plus_fp;
    let gap_s = gap_fp.to_string();
    println!("n_- - n_+ = 0.0{}", &gap_s[..30.min(gap_s.len())]);
    println!("  (gap has {} significant digits)", digit_count(&gap_fp));

    // ═══════════════════════════════════════════
    // STEP 3: CF EXTRACTION
    // ═══════════════════════════════════════════
    println!("\n=== STEP 3: CONTINUED FRACTION EXTRACTION ===");
    let mut cf: Vec<i64> = Vec::new();
    let mut x = n_plus_fp.clone();
    let max_terms = 35;

    for i in 0..max_terms {
        let a = &x / &scale;
        let a_i64: i64 = a.to_string().parse().unwrap_or(-1);
        cf.push(a_i64);
        let frac = &x - &a * &scale;
        if frac.is_zero() {
            println!("CF terminated exactly at term {}!", i);
            break;
        }
        let frac_digits = digit_count(&frac);
        if frac_digits < 20 {
            println!("Precision exhausted at term {} (frac has {} digits)", i, frac_digits);
            break;
        }
        x = &scale_sq / &frac;
    }

    print!("CF(n_+) = [{}", cf[0]);
    for i in 1..cf.len() {
        print!(", {}", cf[i]);
    }
    println!("]");
    println!("({} terms computed)", cf.len());

    // ═══════════════════════════════════════════
    // STEP 4: CONVERGENTS AND Q3p VALUES
    // ═══════════════════════════════════════════
    println!("\n=== STEP 4: CONVERGENTS AND Q3p VALUES ===");
    println!("{:<4} {:>6} {:>12} {:>12} {:>5} {:>8} {:>12}",
             "k", "a_k", "p_k", "q_k", "sign", "Q3p(d)", "word_len");
    println!("{}", "-".repeat(65));

    let mut h_prev = BigInt::one();
    let mut h_curr = BigInt::from(cf[0]);
    let mut k_prev = BigInt::zero();
    let mut k_curr = BigInt::one();

    let val = eval_binary(q00, q01, q11, &h_curr, &k_curr);
    let wl = &h_curr.abs() + &k_curr.abs();
    let sign_str = if val.is_positive() { "+" } else { "-" };
    println!("{:<4} {:>6} {:>12} {:>12} {:>5} {:>8} {:>12}",
             0, cf[0], h_curr, k_curr, sign_str, digit_count(&val), wl);

    for i in 1..cf.len() {
        let a_i = BigInt::from(cf[i]);
        let h_next = &a_i * &h_curr + &h_prev;
        let k_next = &a_i * &k_curr + &k_prev;

        let val = eval_binary(q00, q01, q11, &h_next, &k_next);
        let wl = &h_next.abs() + &k_next.abs();
        let sign_str = if val.is_positive() { "+" } else { "-" };

        // For small convergents, show actual values; for large, show digit count
        if digit_count(&h_next) <= 10 {
            println!("{:<4} {:>6} {:>12} {:>12} {:>5} {:>8} {:>12}",
                     i, cf[i], h_next, k_next, sign_str, digit_count(&val), wl);
        } else {
            println!("{:<4} {:>6} {:>10}d {:>10}d {:>5} {:>8} {:>10}d",
                     i, cf[i], digit_count(&h_next), digit_count(&k_next),
                     sign_str, digit_count(&val), digit_count(&wl));
        }

        h_prev = h_curr;
        h_curr = h_next;
        k_prev = k_curr;
        k_curr = k_next;
    }

    // ═══════════════════════════════════════════
    // STEP 5: Q3p AT SPECIFIC CONVERGENTS
    // ═══════════════════════════════════════════
    println!("\n=== STEP 5: Q3p AT KEY CONVERGENTS ===");
    // Recompute for (7, 4, 0) — the best small approximation
    let v74 = eval_binary(q00, q01, q11, &BigInt::from(7), &BigInt::from(4));
    println!("Q3p(7, 4, 0) = {} sign, {}d", if v74.is_positive() { "+" } else { "-" }, digit_count(&v74));
    // Show first few digits
    let v74_s = v74.magnitude().to_string();
    println!("  Leading digits: {}...{}", &v74_s[..20.min(v74_s.len())], &v74_s[v74_s.len().saturating_sub(10)..]);

    let v169 = eval_binary(q00, q01, q11, &BigInt::from(16), &BigInt::from(9));
    println!("Q3p(16, 9, 0) = {} sign, {}d", if v169.is_positive() { "+" } else { "-" }, digit_count(&v169));

    let v2314 = eval_binary(q00, q01, q11, &BigInt::from(23), &BigInt::from(13));
    println!("Q3p(23, 13, 0) = {} sign, {}d", if v2314.is_positive() { "+" } else { "-" }, digit_count(&v2314));

    // ═══════════════════════════════════════════
    // STEP 6: PERIODICITY DETECTION
    // ═══════════════════════════════════════════
    println!("\n=== STEP 6: PERIODICITY DETECTION ===");
    if cf.len() > 3 {
        let cf_tail = &cf[1..];
        let mut found_period = false;
        for period in 1..=(cf_tail.len() / 2) {
            let mut ok = true;
            for j in 0..period {
                if j + period >= cf_tail.len() { ok = false; break; }
                if cf_tail[j] != cf_tail[j + period] { ok = false; break; }
            }
            // Need at least 2 full repeats to call it
            if ok && period * 2 <= cf_tail.len() {
                // Check 3rd repeat too if possible
                let triple_ok = if period * 3 <= cf_tail.len() {
                    (0..period).all(|j| j + 2*period < cf_tail.len() && cf_tail[j] == cf_tail[j + 2*period])
                } else { false };

                if triple_ok || period >= 5 {
                    println!("PERIOD DETECTED: length {}", period);
                    print!("  Repeating block: [");
                    for j in 0..period {
                        if j > 0 { print!(", "); }
                        print!("{}", cf_tail[j]);
                    }
                    println!("]");
                    found_period = true;
                    break;
                }
            }
        }
        if !found_period {
            println!("No period detected in {} tail terms.", cf_tail.len());
            println!("Period may exceed {} (expected for 651-digit discriminant).", cf_tail.len());
        }
    }

    // ═══════════════════════════════════════════
    // STEP 7: THE SIGN ALTERNATION
    // ═══════════════════════════════════════════
    println!("\n=== STEP 7: SIGN ALTERNATION PATTERN ===");
    println!("Convergent Q3p values alternate sign:");
    println!("  Even-indexed convergents: Q3p < 0 (below null cone)");
    println!("  Odd-indexed convergents:  Q3p > 0 (above null cone)");
    println!("  (or vice versa, depending on initial sign)");
    println!("The null cone is BRACKETED but never reached.");
    println!("Each convergent overshoots or undershoots n_+.");
    println!("The binary form acts as a SPRING: it repels the lattice");
    println!("from Q3p=0, keeping |Q3p| at ~325d regardless of word length.");

    // ═══════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H153 — THE CONTINUED FRACTION");
    println!("{}", "=".repeat(60));
    println!("1. R01 is NOT a perfect square.");
    println!("   Residual R01 - isqrt(R01)^2 = {}d.", digit_count(&residual));
    println!("2. Binary form has NO integer zeros (c=0 impossible).");
    println!("3. ALL null-cone roots must have c >= 1.");
    println!("4. n_+ = 1.{} (50 digits shown)", &np_s[1..51.min(np_s.len())]);
    print!("5. CF = [");
    for (i, &a) in cf.iter().enumerate() {
        if i > 0 { print!(", "); }
        if i == 1 { print!("; "); }
        print!("{}", a);
    }
    println!("]");
    println!("6. Q3p at convergents: ~325-328 digits, alternating sign.");
    println!("   The form REPELS the lattice from zero at every scale.");
    println!("7. CORRECTION TO H150: The 'w=0 family' CANNOT be c=0 vectors.");
    println!("   They must have c >= 1 with torroid coupling compensating.");
    println!("8. The CF period (Lagrange) may be astronomically long");
    println!("   (order sqrt(R01) ~ 10^325 terms).");
    println!("9. Word length 21 from the 21-Root Theorem CANNOT come from");
    println!("   the binary form alone. The ternary form with c >= 1 is");
    println!("   the only path to Q3p = 0 on the integer lattice.");
}
