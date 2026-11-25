//! H158: The Ostrowski Bridge
//! The CF partial quotient sum 1+1+3+2+1+2+4+7 = 21 suggests
//! the word length is encoded in the CF of n₊. Exploring via
//! Ostrowski representation and exact Smith invariants.

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

fn isqrt(n: &BigInt) -> BigInt {
    if n.is_zero() { return BigInt::zero(); }
    let mut x = n.clone();
    let mut y = (&x + BigInt::one()) / BigInt::from(2u32);
    while y < x { x = y.clone(); y = (&x + n / &x) / BigInt::from(2u32); }
    x
}

fn main() {
    let q = q3p_matrix();
    let q00 = &q[0][0]; let q01 = &q[0][1]; let q11 = &q[1][1];
    let q02 = &q[0][2]; let q12 = &q[1][2]; let q22 = &q[2][2];

    println!("=== H158: THE OSTROWSKI BRIDGE (Rust BigInt) ===\n");

    // ═══ STEP 1: EXACT SMITH INVARIANT d₂ ═══
    println!("=== STEP 1: EXACT SMITH INVARIANT d2 ===");
    let cof00 = q11 * q22 - q12 * q12;
    let cof11 = q00 * q22 - q02 * q02;
    let cof22 = q00 * q11 - q01 * q01;
    let cof01 = -(q01 * q22 - q12 * q02);
    let cof02 = q01 * q12 - q11 * q02;
    let cof12 = -(q00 * q12 - q01 * q02);

    let gc1 = cof00.abs().gcd(&cof11.abs());
    let gc2 = gc1.gcd(&cof22.abs());
    let gc3 = gc2.gcd(&cof01.abs());
    let gc4 = gc3.gcd(&cof02.abs());
    let gcd_cof = gc4.gcd(&cof12.abs());

    let d1 = BigInt::from(2);
    let d2 = &gcd_cof / &d1;

    println!("gcd(all cofactors) = {}", gcd_cof);
    println!("d1 = {}", d1);
    println!("d2 = gcd_cof / d1 = {}", d2);

    let det = q00 * &cof00 + q01 * &cof01 + q02 * &cof02;
    let d3 = det.abs() / (&d1 * &d2);
    println!("d3 = |det| / (d1*d2) = {}d", digit_count(&d3));
    println!("\nSMITH NORMAL FORM: diag({}, {}, {}d)", d1, d2, digit_count(&d3));

    // Check divisibility: d1 | d2 | d3
    let d2_div_d1 = &d2 % &d1;
    let d3_div_d2 = &d3 % &d2;
    println!("d1 | d2: {} (d2 mod d1 = {})", d2_div_d1.is_zero(), d2_div_d1);
    println!("d2 | d3: {} (d3 mod d2 = {})", d3_div_d2.is_zero(), d3_div_d2);

    // Factor d2
    println!("\nFactoring d2 = {}:", d2);
    let mut n = d2.clone();
    let mut factors = Vec::new();
    for p in [2i64,3,5,7,11,13,17,19,23,29,31,37,41,43] {
        let bp = BigInt::from(p);
        while (&n % &bp).is_zero() { n /= &bp; factors.push(p); }
    }
    if !n.is_one() { println!("  remaining: {}", n); }
    println!("  d2 = {}", factors.iter().map(|f|f.to_string()).collect::<Vec<_>>().join(" × "));

    // ═══ STEP 2: CF CONVERGENT STRUCTURE ═══
    println!("\n=== STEP 2: CF CONVERGENT DENOMINATORS ===");
    // CF(n₊) = [1; 1, 3, 2, 1, 2, 4, 7, 2, 17, ...]
    let cf: Vec<i64> = vec![1, 1, 3, 2, 1, 2, 4, 7, 2, 17, 1, 11, 4, 1, 2, 1, 4, 14, 9, 35];

    // Compute convergent denominators q_k
    let mut q_prev = BigInt::zero();
    let mut q_curr = BigInt::one();
    let mut p_prev = BigInt::one();
    let mut p_curr = BigInt::from(cf[0]);

    println!("{:<4} {:<6} {:>12} {:>12} {:>8} {:>8}",
        "k", "a_k", "p_k", "q_k", "p+q", "cum_a");
    println!("{}", "-".repeat(55));

    let mut cum_a: i64 = cf[0];
    println!("{:<4} {:<6} {:>12} {:>12} {:>8} {:>8}",
        0, cf[0], p_curr.to_string(), q_curr.to_string(),
        (&p_curr + &q_curr).to_string(), cum_a);

    for k in 1..cf.len() {
        let a = BigInt::from(cf[k]);
        let p_next = &a * &p_curr + &p_prev;
        let q_next = &a * &q_curr + &q_prev;
        cum_a += cf[k];

        let wl = &p_next + &q_next;

        if digit_count(&p_next) <= 8 {
            println!("{:<4} {:<6} {:>12} {:>12} {:>8} {:>8}",
                k, cf[k], p_next.to_string(), q_next.to_string(),
                wl.to_string(), cum_a);
        } else {
            println!("{:<4} {:<6} {:>10}d {:>10}d {:>6}d {:>8}",
                k, cf[k], digit_count(&p_next), digit_count(&q_next),
                digit_count(&wl), cum_a);
        }

        p_prev = p_curr; p_curr = p_next;
        q_prev = q_curr; q_curr = q_next;
    }

    // ═══ STEP 3: THE PARTIAL QUOTIENT SUM ═══
    println!("\n=== STEP 3: CUMULATIVE PARTIAL QUOTIENT SUMS ===");
    let mut cum = 0i64;
    for k in 0..cf.len() {
        cum += cf[k];
        let marker = if cum == 21 { " ← MATCH! word length = 21" }
                     else if cum == 42 { " ← 42 = cofactor gap" }
                     else if cum == 84 { " ← 84 = cofactor span" }
                     else { "" };
        println!("  sum(a_0..a_{}) = {}{}", k, cum, marker);
    }

    println!("\nKey observations:");
    println!("  sum(a_0..a_7) = 1+1+3+2+1+2+4+7 = 21 = word length");
    println!("  This occurs at k=7, where a_7 = 7 (the largest quotient so far).");
    println!("  Convergent p_7/q_7 = 1959/1106, word length 3065.");

    // ═══ STEP 4: OSTROWSKI REPRESENTATION OF 21 ═══
    println!("\n=== STEP 4: OSTROWSKI REPRESENTATION OF 21 ===");
    // Ostrowski representation: every n ∈ N can be uniquely written as
    // n = Σ c_k · q_k where 0 ≤ c_k ≤ a_{k+1}, and if c_k = a_{k+1} then c_{k-1} = 0.
    // q_k are the CF convergent denominators.

    // q_0 = 1, q_1 = 1, q_2 = 4, q_3 = 9, q_4 = 13, q_5 = 35
    // For n = 21:
    // 21 < q_5 = 35, so c_5 and above are 0.
    // 21 = c_4 * 13 + remainder. max c_4 = a_5 = 2.
    // c_4 = 1: 21 - 13 = 8. Then 8 = c_3 * 9 + ... max c_3 = a_4 = 1.
    // c_3 = 0 (since 9 > 8). 8 = c_2 * 4 + ... max c_2 = a_3 = 2.
    // c_2 = 2: 8 - 8 = 0. Done! 21 = 1*13 + 2*4 = 13 + 8 = 21. ✓
    // But need to check: c_2 = a_3 = 2, so c_1 must be 0.
    // Check: c_1 = 0 ✓.
    // So 21 = 0·q_0 + 0·q_1 + 2·q_2 + 0·q_3 + 1·q_4
    //       = 0·1 + 0·1 + 2·4 + 0·9 + 1·13 = 8 + 13 = 21 ✓

    println!("CF convergent denominators:");
    println!("  q_0 = 1, q_1 = 1, q_2 = 4, q_3 = 9, q_4 = 13, q_5 = 35");
    println!("");
    println!("Ostrowski representation of 21:");
    println!("  21 = 0·q_0 + 0·q_1 + 2·q_2 + 0·q_3 + 1·q_4");
    println!("     = 0·1 + 0·1 + 2·4 + 0·9 + 1·13");
    println!("     = 8 + 13 = 21 ✓");
    println!("");
    println!("Ostrowski digits: [0, 0, 2, 0, 1] (5 nonzero positions)");
    println!("Active q-values: q_2 = 4 and q_4 = 13");
    println!("Corresponding convergents: 7/4 and 23/13");
    println!("");
    println!("FINDING: The Ostrowski representation of 21 uses EXACTLY the");
    println!("convergents 7/4 and 23/13 — the two best small approximations");
    println!("to n₊ ≈ 1.771. The word length 21 is built from q_2 and q_4.");

    // ═══ STEP 5: THE CONVERGENT-LATTICE CONNECTION ═══
    println!("\n=== STEP 5: CONVERGENT-LATTICE CONNECTION ===");
    println!("Convergent 7/4 (k=2): Q3p(7,4,0) = -326d (best w=0 approach)");
    println!("Convergent 23/13 (k=4): Q3p(23,13,0) = -326d (second best)");
    println!("");
    println!("Ostrowski says: 21 = 2×q_2 + 1×q_4 = 2×4 + 1×13 = 21.");
    println!("If the frozen lattice uses these convergent denominators as");
    println!("building blocks, then the minimum word length is determined by");
    println!("the SMALLEST integer representable using q_2 and q_4 that");
    println!("satisfies the lattice membership criterion.");
    println!("");
    println!("The coefficients [2, 1] mean: take 2 copies of the q_2 step");
    println!("and 1 copy of the q_4 step. Total: 2×4 + 1×13 = 21.");

    // ═══ STEP 6: THE d₂ CONNECTION ═══
    println!("\n=== STEP 6: THE d2 CONNECTION ===");
    println!("Smith invariant d2 = {}", d2);
    println!("Factorization: {}", factors.iter().map(|f|f.to_string()).collect::<Vec<_>>().join(" × "));

    // Check: does d2 divide 21?
    let twenty_one = BigInt::from(21);
    let rem = &twenty_one % &d2;
    println!("21 mod d2 = {} → d2 {} 21", rem, if rem.is_zero() { "divides" } else { "does NOT divide" });

    // Check: does d2 relate to q_2 or q_4?
    println!("q_2 = 4, q_4 = 13");
    println!("d2 mod q_2 = {}", &d2 % BigInt::from(4));
    println!("d2 mod q_4 = {}", &d2 % BigInt::from(13));

    // R01 mod d2
    let r01 = q01 * q01 - &(q00.abs() * q11.abs());
    println!("R01 mod d2 = {}", &r01 % &d2);

    // ═══ STEP 7: THE BINARY FORM VALUE AT OSTROWSKI VECTORS ═══
    println!("\n=== STEP 7: Q3p AT OSTROWSKI-DERIVED VECTORS ===");
    // The Ostrowski representation 21 = 2·q_2 + 1·q_4 = 2×4 + 13
    // suggests testing vectors built from convergents.
    // Convergent k=2: (p_2, q_2) = (7, 4)
    // Convergent k=4: (p_4, q_4) = (23, 13)
    // Linear combination: 2×(7,4) + 1×(23,13) = (14+23, 8+13) = (37, 21)
    // Word length: 37 + 21 = 58 (not 21!)
    // The Ostrowski representation applies to word length, not to the vector coordinates.

    let eval_bin = |a: i64, b: i64| -> BigInt {
        let ab = BigInt::from(a); let bb = BigInt::from(b);
        q00 * &ab * &ab + BigInt::from(2) * q01 * &ab * &bb + q11 * &bb * &bb
    };

    println!("Q3p at Ostrowski-related vectors:");
    for &(a,b,label) in &[(7,4,"conv_2"),(23,13,"conv_4"),(37,21,"2*conv_2+conv_4"),
                           (14,8,"2*conv_2"),(30,17,"conv_2+conv_4"),
                           (16,9,"conv_3"),(62,35,"conv_5")] {
        let v = eval_bin(a, b);
        println!("  ({:>3},{:>3}) {:>18}: {} sign, {}d, wl={}",
            a, b, label, if v.is_positive() { "+" } else { "-" }, digit_count(&v), a+b);
    }

    // ═══ STEP 8: WHERE 21 SITS IN THE CF STRUCTURE ═══
    println!("\n=== STEP 8: WHERE 21 SITS ===");
    println!("The number 21 in the CF structure of n₊:");
    println!("  1. Partial quotient sum: a_0+...+a_7 = 21");
    println!("  2. Ostrowski representation: 21 = 2·q_2 + 1·q_4 = 2×4 + 13");
    println!("  3. Active convergents: 7/4 and 23/13 (the first two good approximations)");
    println!("  4. k=7 is where a_k=7 (largest quotient in first 8 terms)");
    println!("  5. q_2 = 4, q_4 = 13: gcd(4,13) = {}", 4i64.gcd(&13));
    println!("");
    println!("The partial quotient sum at k is NOT a standard CF invariant.");
    println!("But it counts the total 'rotational budget' — the sum of all");
    println!("the floor-steps the CF has taken. When this sum reaches G/2 = 21,");
    println!("the CF has exhausted half the cofactor gap's worth of rotations.");
    println!("");
    println!("CONJECTURE (The Ostrowski Bridge): The minimum word length in F");
    println!("equals the smallest n whose Ostrowski representation (w.r.t. n₊'s");
    println!("CF) uses only convergents up to the k where sum(a_0..a_k) = G/2.");
    println!("For Q3p: G/2 = 21, and 21 = 2·q_2 + q_4 (k ≤ 7).");

    // ═══ SUMMARY ═══
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H158 — THE OSTROWSKI BRIDGE");
    println!("{}", "=".repeat(60));
    println!("1. EXACT d2 = {} = {}", d2, factors.iter().map(|f|f.to_string()).collect::<Vec<_>>().join("×"));
    println!("2. Smith form: diag(2, {}, {}d)", d2, digit_count(&d3));
    println!("3. Partial quotient sum: a_0+...+a_7 = 21 at k=7 (a_7=7).");
    println!("4. Ostrowski(21) = 2·q_2 + 1·q_4 = 2×4 + 13 = 21.");
    println!("5. Active convergents: 7/4 and 23/13.");
    println!("6. The word length 21 is built from the FIRST TWO good");
    println!("   rational approximations to n₊ ≈ 1.771.");
    println!("7. The CF partial quotient sum = G/2 at k=7 connects the");
    println!("   cofactor gap to the CF structure.");
    println!("8. CONJECTURED: Ostrowski Bridge — word length = smallest n");
    println!("   representable by CF convergents up to the G/2 threshold.");
}
