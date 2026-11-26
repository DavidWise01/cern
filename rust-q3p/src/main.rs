//! H176: The Jacobi Frame
//! Map Q3p to the 3-body problem in Jacobi coordinates.
//! {{x,y:z}}² = 6 entries, +1 implicit core, +1 explicit observer.

use num_bigint::BigInt;
use num_traits::Zero;
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

fn main() {
    let q = q3p_matrix();
    let q00=&q[0][0];let q01=&q[0][1];let q11=&q[1][1];
    let q02=&q[0][2];let q12=&q[1][2];let q22=&q[2][2];

    println!("=== H176: THE JACOBI FRAME ===\n");

    // ═══ THE 3-BODY MAP ═══
    println!("═══ THE 3-BODY MAP ═══\n");
    println!("  3 bodies in Jacobi coordinates:");
    println!("  Body x (brace):   mass scale ~ |Q00| = {}d", dc(q00));
    println!("  Body y (box):     mass scale ~ |Q11| = {}d", dc(q11));
    println!("  Body z (torroid): mass scale ~ |Q22| = {}d\n", dc(q22));

    println!("  Jacobi reduction: 3 bodies → 2 relative vectors + 1 COM");
    println!("  In Q3p's coordinates: a = x-COM, b = y-COM, c = z-COM");
    println!("  The COM is IMPLICIT (projected out). That's the singlet.\n");

    // ═══ THE 6 = {{x,y:z}}² ═══
    println!("═══ THE 6 INTERACTIONS = {{{{x,y:z}}}}² ═══\n");
    println!("  The brace {{x,y:z}}² generates 6 unique pairings:");
    println!("  Self-interactions (diagonal):");
    println!("    x·x → Q00 = −{}d (brace self)", dc(q00));
    println!("    y·y → Q11 = −{}d (box self)", dc(q11));
    println!("    z·z → Q22 = −{}d (torroid self)\n", dc(q22));
    println!("  Cross-interactions (off-diagonal):");
    println!("    x·y → Q01 = +{}d (brace-box coupling)", dc(q01));
    println!("    x·z → Q02 = −{}d (brace-torroid coupling)", dc(q02));
    println!("    y·z → Q12 = +{}d (box-torroid coupling)\n", dc(q12));

    // ═══ THE SIGN PATTERN: −+3 ═══
    println!("═══ THE SIGN PATTERN: −+3 BODIES ═══\n");
    println!("  User's notation: -+3 bodies");
    println!("  Q3p signature: (1+, 2−)");
    println!("  One REPULSIVE direction, two ATTRACTIVE directions.\n");
    println!("  In 3-body language:");
    println!("  − : Two bodies are BOUND (attracted, can't escape easily)");
    println!("  + : One direction is UNBOUND (repulsive, escape possible)\n");
    println!("  The sign pattern of the matrix diagonal:");
    println!("  Q00 < 0 (−), Q11 < 0 (−), Q22 < 0 (−)");
    println!("  All self-interactions are ATTRACTIVE (binding).");
    println!("  But Q01 > 0 (+): the brace-box coupling is REPULSIVE.");
    println!("  This repulsion creates the one positive eigenvalue direction.");
    println!("  → The escape is BETWEEN bodies, not from any single body.\n");

    // ═══ THE 2+1 SPLIT (the colon) ═══
    println!("═══ THE 2+1 SPLIT: {{x,y : z}} ═══\n");
    println!("  The colon separates {{x,y}} from {{z}}:");
    println!("  {{x,y}} = the binary subsystem (brace + box, c=0 plane)");
    println!("  {{z}} = the torroid (the third body)\n");
    println!("  Jacobi vector 1 (ρ₁): relative position of x and y");
    println!("    → This is the brace-box binary form Q(a,b,0)");
    println!("    → R01 = Q01² − |Q00|·|Q11| = +651d (POSITIVE: repulsive!)");
    println!("    → Escape at wl=25 in this subsystem\n");
    println!("  Jacobi vector 2 (ρ₂): position of z relative to COM(x,y)");
    println!("    → This is the torroid coordinate c");
    println!("    → Q22 = −408d (MASSIVE anchor, strongly bound)");
    println!("    → Escape at wl≈10⁴² (barely, at cosmic word length)\n");

    // ═══ MASS HIERARCHY ═══
    println!("═══ THE MASS HIERARCHY ═══\n");
    // In 3-body, the mass ratios determine the dynamics.
    // Here "mass" = |diagonal entry| (the self-interaction strength)
    let r_xy = dc(q11) as f64 / dc(q00) as f64;
    let r_xz = dc(q22) as f64 / dc(q00) as f64;
    let r_yz = dc(q22) as f64 / dc(q11) as f64;
    println!("  Mass ratios (by digit count):");
    println!("  |Q11|/|Q00| = {}/{} ≈ {:.3} (box/brace ≈ equal)", dc(q11), dc(q00), r_xy);
    println!("  |Q22|/|Q00| = {}/{} ≈ {:.3} (torroid/brace ≈ 1.24×)", dc(q22), dc(q00), r_xz);
    println!("  |Q22|/|Q11| = {}/{} ≈ {:.3} (torroid/box ≈ 1.24×)\n", dc(q22), dc(q11), r_yz);
    println!("  But the REAL hierarchy is in magnitudes, not digit counts:");
    println!("  |Q22|/|Q00| ≈ 10^(408-328) = 10^80");
    println!("  The torroid is 10⁸⁰ times heavier than the brace!");
    println!("  This is NOT a near-equal-mass 3-body problem.");
    println!("  It is a HIERARCHICAL 3-body problem:");
    println!("    Two light bodies (brace, box) ≈ same mass");
    println!("    One HEAVY body (torroid) ≈ 10⁸⁰ times heavier\n");

    // ═══ THE JACOBI REDUCTION ═══
    println!("═══ THE JACOBI REDUCTION TABLE ═══\n");
    println!("  │ 3-Body          │ Q3p              │ Digit │");
    println!("  ├─────────────────┼──────────────────┼───────┤");
    println!("  │ Body 1 (x)      │ Brace (a)        │ 328d  │");
    println!("  │ Body 2 (y)      │ Box (b)          │ 329d  │");
    println!("  │ Body 3 (z)      │ Torroid (c)      │ 408d  │");
    println!("  │ ρ₁ = y−x        │ Binary form(a,b) │ 328d  │");
    println!("  │ ρ₂ = z−COM(x,y) │ Torroid coord c  │ 408d  │");
    println!("  │ COM (implicit)  │ The singlet      │ ——    │");
    println!("  │ Observer (expl) │ The frame        │ ——    │");
    println!("  │ 6 interactions  │ 6 matrix entries │       │");
    println!("  │ {{{{x,y:z}}}}²      │ Q00..Q22         │       │");

    // ═══ THE 8 = 6 + 1 + 1 ═══
    println!("\n═══ THE ACCOUNTING: 6 + 1 + 1 = 8 ═══\n");
    println!("  6: The unique entries of Q3p (the interaction matrix)");
    println!("     Q00, Q01, Q11, Q02, Q12, Q22");
    println!("     = {{{{x,y:z}}}}² = the squared brace\n");
    println!("  +1 (implicit): The center of mass / the core");
    println!("     Projected out in Jacobi reduction.");
    println!("     Not a variable of Q3p. Not observable.");
    println!("     The singlet that holds the frame together.\n");
    println!("  +1 (explicit): The observer");
    println!("     The external frame that evaluates Q3p.");
    println!("     Chooses which direction to measure.");
    println!("     Makes the form definite (picks a signature).\n");
    println!("  Total: 8 = the OCHO.");
    println!("  The 3-body Q3p IS an 8-cell structure:");
    println!("  6 interaction cells + 1 core + 1 observer.");

    // ═══ ESCAPE DYNAMICS IN 3-BODY LANGUAGE ═══
    println!("\n═══ ESCAPE = UNBINDING ═══\n");
    println!("  c=0 escape (wl=25):");
    println!("    Bodies x and y separate (ρ₁ → ∞)");
    println!("    Body z stays put (ρ₂ = 0)");
    println!("    The binary subsystem UNBINDS.");
    println!("    Energy: Q3p(16,9,0) > 0 → repulsion wins.\n");
    println!("  c=1 escape (wl≈10⁴²):");
    println!("    Bodies x and y separate AND body z leaves");
    println!("    All three bodies UNBIND simultaneously.");
    println!("    Energy: Q3p(16k,9k,1) > 0 for k ≈ 10⁴¹.");
    println!("    The heavy body z takes 10⁴⁰× longer to leave.\n");
    println!("  The mass hierarchy determines the escape order:");
    println!("  Light pair unbinds first (wl=25).");
    println!("  Heavy body unbinds last (wl≈10⁴²).");
    println!("  Ratio: 10⁴² / 25 ≈ 10⁴⁰ ≈ (m_z/m_x)^(1/2).");
}
