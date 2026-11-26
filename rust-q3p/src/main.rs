//! H161: The p-Adic Depth
//! Q3p modulo prime powers 3^k and 7^k.
//! Does the minimum word length climb toward 21?

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

fn mod_pos(a: &BigInt, m: &BigInt) -> i64 {
    let r = a % m;
    let r = if r.is_negative() { r + m } else { r };
    r.to_string().parse().unwrap_or(0)
}

fn main() {
    let q = q3p_matrix();
    let q00=&q[0][0]; let q01=&q[0][1]; let q11=&q[1][1];
    let q02=&q[0][2]; let q12=&q[1][2]; let q22=&q[2][2];

    println!("=== H161: THE p-ADIC DEPTH (Rust BigInt) ===\n");

    // ═══ STEP 1: POWERS OF 3 ═══
    println!("=== STEP 1: POWERS OF 3 ===");
    let powers3: Vec<i64> = vec![3, 9, 27, 81, 243];

    for &pk in &powers3 {
        let m = BigInt::from(pk);
        let mm = [
            [mod_pos(q00,&m),mod_pos(q01,&m),mod_pos(q02,&m)],
            [mod_pos(q01,&m),mod_pos(q11,&m),mod_pos(q12,&m)],
            [mod_pos(q02,&m),mod_pos(q12,&m),mod_pos(q22,&m)]
        ];

        let mut min_wl = pk + 1;
        let mut iso_count = 0i64;
        let mut min_vec = (0i64,0i64,0i64);

        for a in 0..pk { for b in 0..pk { for c in 0..pk {
            if a==0&&b==0&&c==0{continue;}
            let v = (mm[0][0]*a*a + 2*mm[0][1]*a*b + mm[1][1]*b*b
                + 2*mm[0][2]*a*c + 2*mm[1][2]*b*c + mm[2][2]*c*c) % pk;
            let v = ((v % pk) + pk) % pk;
            if v == 0 {
                iso_count += 1;
                let al=if a>pk/2{pk-a}else{a};
                let bl=if b>pk/2{pk-b}else{b};
                let cl=if c>pk/2{pk-c}else{c};
                let wl=al+bl+cl;
                if wl < min_wl { min_wl = wl; min_vec = (al,bl,cl); }
            }
        }}}

        println!("mod 3^{} = {:>4}: iso={:>8}, min_wl={:>3} at ({:+},{:+},{:+})",
            (pk as f64).log(3.0).round() as i32, pk, iso_count, min_wl,
            min_vec.0, min_vec.1, min_vec.2);
    }

    // ═══ STEP 2: POWERS OF 7 ═══
    println!("\n=== STEP 2: POWERS OF 7 ===");
    let powers7: Vec<i64> = vec![7, 49, 343];

    for &pk in &powers7 {
        let m = BigInt::from(pk);
        let mm = [
            [mod_pos(q00,&m),mod_pos(q01,&m),mod_pos(q02,&m)],
            [mod_pos(q01,&m),mod_pos(q11,&m),mod_pos(q12,&m)],
            [mod_pos(q02,&m),mod_pos(q12,&m),mod_pos(q22,&m)]
        ];

        let mut min_wl = pk + 1;
        let mut iso_count = 0i64;
        let mut min_vec = (0i64,0i64,0i64);
        let mut short_vecs: Vec<(i64,i64,i64,i64)> = Vec::new();

        for a in 0..pk { for b in 0..pk { for c in 0..pk {
            if a==0&&b==0&&c==0{continue;}
            let v = (mm[0][0]*a*a + 2*mm[0][1]*a*b + mm[1][1]*b*b
                + 2*mm[0][2]*a*c + 2*mm[1][2]*b*c + mm[2][2]*c*c) % pk;
            let v = ((v % pk) + pk) % pk;
            if v == 0 {
                iso_count += 1;
                let al=if a>pk/2{pk-a}else{a};
                let bl=if b>pk/2{pk-b}else{b};
                let cl=if c>pk/2{pk-c}else{c};
                let wl=al+bl+cl;
                if wl < min_wl { min_wl = wl; min_vec = (al,bl,cl); short_vecs.clear(); }
                if wl == min_wl { short_vecs.push((al,bl,cl,wl)); }
            }
        }}}

        let k = (pk as f64).log(7.0).round() as i32;
        println!("mod 7^{} = {:>4}: iso={:>8}, min_wl={:>3} at ({:+},{:+},{:+})",
            k, pk, iso_count, min_wl, min_vec.0, min_vec.1, min_vec.2);

        if pk <= 49 && short_vecs.len() <= 20 {
            for sv in &short_vecs {
                println!("    ({:+},{:+},{:+}) wl={}", sv.0, sv.1, sv.2, sv.3);
            }
        }
    }

    // ═══ STEP 3: CRT COMBINATIONS OF HIGHER POWERS ═══
    println!("\n=== STEP 3: CRT OF HIGHER POWERS ===");
    // mod 9×7 = 63, mod 27×7 = 189, mod 9×49 = 441
    let combos: Vec<(i64, &str)> = vec![
        (63, "9×7"), (189, "27×7"), (441, "9×49")
    ];

    for &(pk, label) in &combos {
        if pk > 500 {
            println!("mod {} = {}: SKIPPED (too large to enumerate)", pk, label);
            continue;
        }
        let m = BigInt::from(pk);
        let mm = [
            [mod_pos(q00,&m),mod_pos(q01,&m),mod_pos(q02,&m)],
            [mod_pos(q01,&m),mod_pos(q11,&m),mod_pos(q12,&m)],
            [mod_pos(q02,&m),mod_pos(q12,&m),mod_pos(q22,&m)]
        ];

        let mut min_wl = pk + 1;
        let mut iso_count = 0i64;
        let mut min_vec = (0i64,0i64,0i64);

        for a in 0..pk { for b in 0..pk { for c in 0..pk {
            if a==0&&b==0&&c==0{continue;}
            let ab=a as i128; let bb=b as i128; let cc=c as i128;
            let m0=mm[0][0] as i128; let m1=mm[0][1] as i128; let m2=mm[1][1] as i128;
            let m3=mm[0][2] as i128; let m4=mm[1][2] as i128; let m5=mm[2][2] as i128;
            let v = (m0*ab*ab + 2*m1*ab*bb + m2*bb*bb
                + 2*m3*ab*cc + 2*m4*bb*cc + m5*cc*cc) % (pk as i128);
            let v = ((v % pk as i128) + pk as i128) % pk as i128;
            if v == 0 {
                iso_count += 1;
                let al=if a>pk/2{pk-a}else{a};
                let bl=if b>pk/2{pk-b}else{b};
                let cl=if c>pk/2{pk-c}else{c};
                let wl=al+bl+cl;
                if wl < min_wl { min_wl = wl; min_vec = (al,bl,cl); }
            }
        }}}

        println!("mod {} = {:>4}: iso={:>8}, min_wl={:>3} at ({:+},{:+},{:+})",
            label, pk, iso_count, min_wl, min_vec.0, min_vec.1, min_vec.2);
    }

    // ═══ STEP 4: THE DEPTH TABLE ═══
    println!("\n=== STEP 4: THE DEPTH TABLE ===");
    println!("{:<16} {:>6} {:>8} {:>8}", "Modulus", "Value", "Min WL", "Trend");
    println!("{}", "-".repeat(42));
    // Reprint key results in table form

    // ═══ STEP 5: 2-ADIC DEPTH ═══
    println!("\n=== STEP 5: 2-ADIC DEPTH ===");
    let powers2: Vec<i64> = vec![2, 4, 8, 16, 32];
    for &pk in &powers2 {
        let m = BigInt::from(pk);
        let mm = [
            [mod_pos(q00,&m),mod_pos(q01,&m),mod_pos(q02,&m)],
            [mod_pos(q01,&m),mod_pos(q11,&m),mod_pos(q12,&m)],
            [mod_pos(q02,&m),mod_pos(q12,&m),mod_pos(q22,&m)]
        ];

        let mut min_wl = pk + 1;
        let mut iso_count = 0i64;

        for a in 0..pk { for b in 0..pk { for c in 0..pk {
            if a==0&&b==0&&c==0{continue;}
            let v = (mm[0][0]*a*a + 2*mm[0][1]*a*b + mm[1][1]*b*b
                + 2*mm[0][2]*a*c + 2*mm[1][2]*b*c + mm[2][2]*c*c) % pk;
            let v = ((v % pk) + pk) % pk;
            if v == 0 {
                iso_count += 1;
                let al=if a>pk/2{pk-a}else{a};
                let bl=if b>pk/2{pk-b}else{b};
                let cl=if c>pk/2{pk-c}else{c};
                let wl=al+bl+cl;
                if wl < min_wl { min_wl = wl; }
            }
        }}}

        let k = (pk as f64).log2().round() as i32;
        println!("mod 2^{} = {:>4}: iso={:>8}, min_wl={:>3}", k, pk, iso_count, min_wl);
    }

    // ═══ STEP 6: THE COMBINED 3^a × 7^b TABLE ═══
    println!("\n=== STEP 6: COMBINED 3^a × 7^b TABLE ===");
    println!("{:<6} {:<6} {:>8} {:>8}", "3^a", "7^b", "mod", "min_wl");
    println!("{}", "-".repeat(32));

    let p3_vals: Vec<i64> = vec![1, 3, 9, 27];
    let p7_vals: Vec<i64> = vec![1, 7, 49];

    for &p3 in &p3_vals {
        for &p7 in &p7_vals {
            let pk = p3 * p7;
            if pk > 500 || pk <= 1 { continue; }

            let m = BigInt::from(pk);
            let mm = [
                [mod_pos(q00,&m),mod_pos(q01,&m),mod_pos(q02,&m)],
                [mod_pos(q01,&m),mod_pos(q11,&m),mod_pos(q12,&m)],
                [mod_pos(q02,&m),mod_pos(q12,&m),mod_pos(q22,&m)]
            ];

            let mut min_wl = pk + 1;
            for a in 0..pk { for b in 0..pk { for c in 0..pk {
                if a==0&&b==0&&c==0{continue;}
                let ab=a as i128; let bb=b as i128; let cc=c as i128;
                let m0=mm[0][0]as i128;let m1=mm[0][1]as i128;let m2=mm[1][1]as i128;
                let m3=mm[0][2]as i128;let m4=mm[1][2]as i128;let m5=mm[2][2]as i128;
                let v=(m0*ab*ab+2*m1*ab*bb+m2*bb*bb+2*m3*ab*cc+2*m4*bb*cc+m5*cc*cc)%(pk as i128);
                let v=((v%(pk as i128))+(pk as i128))%(pk as i128);
                if v==0{
                    let al=if a>pk/2{pk-a}else{a};
                    let bl=if b>pk/2{pk-b}else{b};
                    let cl=if c>pk/2{pk-c}else{c};
                    let wl=al+bl+cl;
                    if wl<min_wl{min_wl=wl;}
                }
            }}}

            let a3 = (p3 as f64).log(3.0).round() as i32;
            let a7 = (p7 as f64).log(7.0).round() as i32;
            println!("3^{}    7^{}    {:>6}  {:>6}{}", a3, a7, pk, min_wl,
                if min_wl >= 21 { " ← ≥21!" } else { "" });
        }
    }

    // ═══ SUMMARY ═══
    println!("\n{}", "=".repeat(60));
    println!("SUMMARY: H161 — THE p-ADIC DEPTH");
    println!("{}", "=".repeat(60));
    println!("The minimum word length at each p-adic level:");
    println!("  3-adic: mod 3→9→27→81→243 (how does min wl grow?)");
    println!("  7-adic: mod 7→49→343 (how does min wl grow?)");
    println!("  Combined 3^a × 7^b: the product table");
    println!("");
    println!("FINDING: The p-adic depth measures how many levels of");
    println!("prime-power structure the frozen lattice uses beyond");
    println!("the first-power CRT (which gives min wl = 4).");
}
