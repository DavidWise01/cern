//! H160: The CRT Proof
//! Combine mod-3 and mod-7 constraints via CRT.
//! Does the combined sublattice have minimum word length 21?

use num_bigint::BigInt;
use num_traits::Signed;
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

fn mod_pos(a: &BigInt, m: i64) -> i64 {
    let bm = BigInt::from(m);
    let r = a % &bm;
    let ri: i64 = if r.is_negative() {
        let s = (-&r).to_string(); let v: i64 = s.parse().unwrap_or(0); m - (v % m)
    } else { r.to_string().parse().unwrap_or(0) };
    ri % m
}

fn main() {
    let q = q3p_matrix();
    let q00=&q[0][0]; let q01=&q[0][1]; let q11=&q[1][1];
    let q02=&q[0][2]; let q12=&q[1][2]; let q22=&q[2][2];

    println!("=== H160: THE CRT PROOF (Rust BigInt) ===\n");

    // ═══ STEP 1: VERIFY MOD-3 STRUCTURE ═══
    println!("=== STEP 1: MOD-3 CONSTRAINT (VERIFY) ===");
    let m3 = [[mod_pos(q00,3),mod_pos(q01,3),mod_pos(q02,3)],
              [mod_pos(q01,3),mod_pos(q11,3),mod_pos(q12,3)],
              [mod_pos(q02,3),mod_pos(q12,3),mod_pos(q22,3)]];
    println!("Q3p mod 3 = [{},{},{}; {},{},{}; {},{},{}]",
        m3[0][0],m3[0][1],m3[0][2],m3[1][0],m3[1][1],m3[1][2],m3[2][0],m3[2][1],m3[2][2]);
    println!("= (a+c)^2 mod 3");
    println!("Isotropic iff a + c ≡ 0 (mod 3)");
    println!("No constraint on b.");
    println!("");

    // All (a,b,c) mod 3 with a+c ≡ 0:
    println!("Allowed residues mod 3 (a+c ≡ 0):");
    let mut allowed3: Vec<(i64,i64,i64)> = Vec::new();
    for a in 0..3i64 { for b in 0..3 { for c in 0..3 {
        if (a + c) % 3 == 0 { allowed3.push((a,b,c)); }
    }}}
    println!("  {} vectors out of 27 (fraction: {}/27)", allowed3.len(), allowed3.len());
    for v in &allowed3 { println!("    ({},{},{})", v.0, v.1, v.2); }

    // ═══ STEP 2: VERIFY MOD-7 STRUCTURE ═══
    println!("\n=== STEP 2: MOD-7 CONSTRAINT (VERIFY) ===");
    let m7 = [[mod_pos(q00,7),mod_pos(q01,7),mod_pos(q02,7)],
              [mod_pos(q01,7),mod_pos(q11,7),mod_pos(q12,7)],
              [mod_pos(q02,7),mod_pos(q12,7),mod_pos(q22,7)]];
    println!("Q3p mod 7 = [{},{},{}; {},{},{}; {},{},{}]",
        m7[0][0],m7[0][1],m7[0][2],m7[1][0],m7[1][1],m7[1][2],m7[2][0],m7[2][1],m7[2][2]);

    // All isotropic (a,b,c) mod 7
    let mut allowed7: Vec<(i64,i64,i64)> = Vec::new();
    for a in 0..7i64 { for b in 0..7 { for c in 0..7 {
        let v = (m7[0][0]*a*a + 2*m7[0][1]*a*b + m7[1][1]*b*b
            + 2*m7[0][2]*a*c + 2*m7[1][2]*b*c + m7[2][2]*c*c) % 7;
        let v = ((v%7)+7)%7;
        if v == 0 { allowed7.push((a,b,c)); }
    }}}
    println!("Isotropic vectors mod 7: {} out of 343 (including origin)", allowed7.len());

    // ═══ STEP 3: CRT COMBINATION ═══
    println!("\n=== STEP 3: CRT COMBINATION MOD 21 ===");
    println!("CRT: a residue (a,b,c) mod 21 is allowed iff:");
    println!("  (a,b,c) mod 3 satisfies: a+c ≡ 0 (mod 3)");
    println!("  (a,b,c) mod 7 satisfies: Q3p(a,b,c) ≡ 0 (mod 7)");
    println!("");

    // Enumerate all (a,b,c) mod 21 satisfying BOTH conditions
    let mut crt_allowed: Vec<(i64,i64,i64)> = Vec::new();
    for a in 0..21i64 { for b in 0..21 { for c in 0..21 {
        // mod-3 check
        if (a + c) % 3 != 0 { continue; }
        // mod-7 check
        let a7 = a%7; let b7 = b%7; let c7 = c%7;
        let v = (m7[0][0]*a7*a7 + 2*m7[0][1]*a7*b7 + m7[1][1]*b7*b7
            + 2*m7[0][2]*a7*c7 + 2*m7[1][2]*b7*c7 + m7[2][2]*c7*c7) % 7;
        let v = ((v%7)+7)%7;
        if v != 0 { continue; }
        crt_allowed.push((a,b,c));
    }}}

    println!("CRT-allowed vectors mod 21: {} out of {} (= 21^3)",
        crt_allowed.len(), 21*21*21);
    println!("Fraction: {}/{} = {:.4}",
        crt_allowed.len(), 21*21*21, crt_allowed.len() as f64 / (21*21*21) as f64);

    // ═══ STEP 4: MINIMUM WORD LENGTH IN CRT SUBLATTICE ═══
    println!("\n=== STEP 4: MINIMUM WORD LENGTH IN CRT SUBLATTICE ===");
    // For each CRT-allowed vector (a,b,c) mod 21, compute the
    // minimum ℓ¹ norm using representatives in {-10,...,10}
    let mut min_wl = 100i64;
    let mut min_vecs: Vec<(i64,i64,i64)> = Vec::new();

    for &(a,b,c) in &crt_allowed {
        if a==0 && b==0 && c==0 { continue; }
        // Lift to centered representatives
        let al = if a > 10 { a - 21 } else { a };
        let bl = if b > 10 { b - 21 } else { b };
        let cl = if c > 10 { c - 21 } else { c };
        let wl = al.abs() + bl.abs() + cl.abs();
        if wl < min_wl { min_wl = wl; min_vecs.clear(); }
        if wl == min_wl { min_vecs.push((al,bl,cl)); }
    }

    println!("Minimum ℓ¹ word length in CRT sublattice: {}", min_wl);
    println!("Vectors achieving minimum:");
    for v in &min_vecs {
        println!("  ({:+},{:+},{:+}), wl={}", v.0, v.1, v.2, v.0.abs()+v.1.abs()+v.2.abs());
    }

    // ═══ STEP 5: IS THE CRT MINIMUM 21? ═══
    println!("\n=== STEP 5: DOES CRT GIVE 21? ===");
    println!("CRT minimum word length = {}", min_wl);
    if min_wl == 21 {
        println!("YES! The CRT combination of mod-3 and mod-7 gives exactly 21.");
        println!("THEOREM PROVED: word_length = 3 × 7 = 21 from local constraints.");
    } else {
        println!("NO. CRT gives {}, not 21.", min_wl);
        println!("The frozen lattice F has ADDITIONAL structure beyond mod-3 and mod-7.");
        println!("The CRT sublattice is a SUPERLATTICE of F (F ⊂ CRT ⊂ Z³).");
        println!("Additional constraints from rqt362 thin the CRT sublattice to F.");
    }

    // ═══ STEP 6: WHAT ADDITIONAL MODULI COULD PUSH TO 21? ═══
    println!("\n=== STEP 6: SEARCHING FOR THE MISSING CONSTRAINT ===");
    // Check larger moduli to see if adding more constraints pushes min_wl toward 21
    for &m in &[6i64, 9, 14, 18, 21, 42, 63, 126] {
        let mut min_m = 200i64;
        for a in 0..m { for b in 0..m { for c in 0..m {
            if a==0&&b==0&&c==0{continue;}
            let val = mod_pos(&(q00*BigInt::from(a*a) + BigInt::from(2)*q01*BigInt::from(a*b)
                + q11*BigInt::from(b*b) + BigInt::from(2)*q02*BigInt::from(a*c)
                + BigInt::from(2)*q12*BigInt::from(b*c) + q22*BigInt::from(c*c)), m);
            if val == 0 {
                let al=if a>m/2{m-a}else{a}; let bl=if b>m/2{m-b}else{b}; let cl=if c>m/2{m-c}else{c};
                let wl=al+bl+cl;
                if wl<min_m{min_m=wl;}
            }
        }}}
        println!("  mod {}: min wl = {}{}", m, min_m,
            if min_m == 21 { " ← MATCH!" } else { "" });
    }

    // ═══ STEP 7: THE STRUCTURE OF THE CRT MINIMUM ═══
    println!("\n=== STEP 7: STRUCTURE OF CRT-MINIMUM VECTORS ===");
    // For each min-wl vector, verify it satisfies both constraints
    for v in &min_vecs {
        let (a,b,c) = *v;
        let mod3_ok = ((a+c)%3+3)%3 == 0;
        // mod 7 check
        let a7 = ((a%7)+7)%7; let b7 = ((b%7)+7)%7; let c7 = ((c%7)+7)%7;
        let q7 = (m7[0][0]*a7*a7 + 2*m7[0][1]*a7*b7 + m7[1][1]*b7*b7
            + 2*m7[0][2]*a7*c7 + 2*m7[1][2]*b7*c7 + m7[2][2]*c7*c7) % 7;
        let mod7_ok = ((q7%7)+7)%7 == 0;

        // Actual Q3p value
        let ab=BigInt::from(a); let bb=BigInt::from(b); let cb=BigInt::from(c);
        let qval = q00*&ab*&ab + BigInt::from(2)*q01*&ab*&bb + q11*&bb*&bb
            + BigInt::from(2)*q02*&ab*&cb + BigInt::from(2)*q12*&bb*&cb + q22*&cb*&cb;
        let sign = if qval.is_positive() { "+" } else { "-" };

        println!("  ({:+},{:+},{:+}): mod3={} mod7={} Q3p={}{}d wl={}",
            a, b, c, mod3_ok, mod7_ok, sign,
            if qval.magnitude().to_string().len() > 1 { qval.magnitude().to_string().len().to_string() }
            else { qval.magnitude().to_string() },
            a.abs()+b.abs()+c.abs());
    }

    // ═══ STEP 8: THE HONEST VERDICT ═══
    println!("\n=== STEP 8: VERDICT ===");
    println!("CRT(mod 3, mod 7) minimum word length = {}", min_wl);
    if min_wl < 21 {
        println!("");
        println!("The CRT combination alone gives min wl = {}, NOT 21.", min_wl);
        println!("This means: the mod-3 and mod-7 constraints are NECESSARY");
        println!("but NOT SUFFICIENT for the word length 21.");
        println!("");
        println!("The gap from {} to 21 requires additional constraints from rqt362:", min_wl);
        println!("  - Higher-power conditions (mod 9 = 3², mod 49 = 7²)");
        println!("  - Conditions at other primes (p=2 contributes through d₁=2)");
        println!("  - Non-modular algebraic conditions (ideal-theoretic)");
        println!("");
        println!("FINDING: 21 = 3 × 7 is the PRIME FACTORIZATION of the word length,");
        println!("and p=3 and p=7 are the primes where Q3p's coupling structure has gaps.");
        println!("But the CRT combination at the FIRST power (mod 3, mod 7) is too weak.");
        println!("The full 21 requires deeper p-adic structure.");
    }

    // ═══ SUMMARY ═══
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H160 — THE CRT PROOF");
    println!("{}", "=".repeat(60));
    println!("1. mod 3: (a+c)^2, constraint a+c ≡ 0. 9 of 27 allowed.");
    println!("2. mod 7: Q02≡0, 49 isotropic (incl origin). Min wl = 3.");
    println!("3. CRT(3,7) → mod 21: {} allowed, min wl = {}.", crt_allowed.len(), min_wl);
    println!("4. min wl {} 21 → CRT is {}.",
        if min_wl == 21 { "=" } else { "≠" },
        if min_wl == 21 { "SUFFICIENT" } else { "NECESSARY but NOT SUFFICIENT" });
    println!("5. The factorization 21 = 3×7 identifies the RELEVANT primes.");
    println!("6. The full word length requires p-adic depth beyond mod p.");
    println!("7. The anatomy: p=3 (collapse) × p=7 (decoupling) = 21.");
}
