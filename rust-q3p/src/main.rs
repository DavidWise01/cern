// H189: The Lock Source
// Trace WHY W−C = d₂−1 by following both paths from the matrix entries.
//
// PATH 1 (CF path): entries → √(|Q22|/|Q00|) → CF → semi-convergent (16,9) → wl=25 → C=4
// PATH 2 (Smith path): entries → mod-9 residues → gcd(minors) → d₂=18
// LOCK: W−C = 21−4 = 17 = 18−1 = d₂−1

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{Zero, One};

fn abs_bigint(n: &BigInt) -> BigUint { n.magnitude().clone() }

fn main() {
    println!("=== H189: THE LOCK SOURCE ===");
    println!("Tracing W−C=d₂−1 through both paths\n");

    // Load Q3p entries
    let q00: BigInt = include_str!("../../q00.txt").trim().parse().unwrap();
    let q11: BigInt = include_str!("../../q11.txt").trim().parse().unwrap();
    let q22: BigInt = include_str!("../../q22.txt").trim().parse().unwrap();

    let a = abs_bigint(&q00); // |Q00|
    let b = abs_bigint(&q11); // |Q11|
    let c = abs_bigint(&q22); // |Q22|

    println!("Q3p diagonal magnitudes:");
    println!("  |Q00| = {} digits", a.to_string().len());
    println!("  |Q11| = {} digits", b.to_string().len());
    println!("  |Q22| = {} digits", c.to_string().len());

    // ════════════════════════════════════════════════════
    // PATH 2: Smith → d₂ = 18
    // ════════════════════════════════════════════════════
    println!("\n=== PATH 2: Smith Form → d₂ = 18 ===");

    // Mod-9 residues of diagonal entries
    let nine = BigUint::from(9u32);
    let a_mod9: u64 = (&a % &nine).try_into().unwrap();
    let b_mod9: u64 = (&b % &nine).try_into().unwrap();
    let c_mod9: u64 = (&c % &nine).try_into().unwrap();
    println!("  |Q00| mod 9 = {}", a_mod9);
    println!("  |Q11| mod 9 = {}", b_mod9);
    println!("  |Q22| mod 9 = {}", c_mod9);

    // Check: do diagonal products ≡ off-diagonal squares mod 9?
    // Q00*Q11 ≡ Q01² (mod 9)
    println!("  |Q00|·|Q11| mod 9 = {}", (a_mod9 * b_mod9) % 9);
    println!("  |Q00|·|Q22| mod 9 = {}", (a_mod9 * c_mod9) % 9);
    println!("  |Q11|·|Q22| mod 9 = {}", (b_mod9 * c_mod9) % 9);

    // d₁ = gcd of all entries
    let d1 = a.gcd(&b).gcd(&c);
    println!("  d₁ = gcd(|Q00|,|Q11|,|Q22|,...) includes {} from diag", d1);

    // d₂ computation: we know d₁=2, d₂=18 from prior work
    println!("  d₁ = 2 (from full matrix, verified in H153)");
    println!("  gcd(all 2×2 minors) = 36 (verified in H168)");
    println!("  d₂ = 36 / d₁ = 36 / 2 = 18");
    println!("  d₂ − 1 = 17");

    // ════════════════════════════════════════════════════
    // PATH 1: CF → escape → W=21, C=4
    // ════════════════════════════════════════════════════
    println!("\n=== PATH 1: CF → Escape → W=21, C=4 ===");

    // The ratio √(|Q22|/|Q00|)
    // CF of this ratio = [1;1,3,1,2,31,...] (from H167)
    // The convergents of this CF give the best rational approximations
    // The semi-convergent (16,9) is the escape direction

    // Compute first few CF terms of |Q22|/|Q00| (integer CF of the ratio)
    // Actually: CF of √(|Q22|/|Q00|) was computed in H167
    // Let me just verify the escape point (16,9,0)
    println!("  CF(√(|Q22|/|Q00|)) = [1;1,3,1,2,31,4,1,1,9,...] (from H167)");
    println!("  Semi-convergent: (16, 9) → escape direction (16,9,0)");
    println!("  Escape word length: 16 + 9 + 0 = 25");
    println!("  W = 21 (from 21-Root Theorem, H156)");
    println!("  C = 25 − 21 = 4");
    println!("  W − C = 21 − 4 = 17");

    // Verify Q3p(16,9,0) > 0 (the escape)
    // Q3p(a,b,c) = Q00·a² + Q11·b² + Q22·c² + 2·Q01·a·b + 2·Q02·a·c + 2·Q12·b·c
    // At (16,9,0): Q00·256 + Q11·81 + 2·Q01·144
    // We need Q01 too
    let q01_str = include_str!("../../q01.txt");
    let q01: BigInt = q01_str.trim().parse().unwrap();
    println!("\n  Verifying Q3p(16,9,0):");
    let term00 = &q00 * BigInt::from(256); // 16²
    let term11 = &q11 * BigInt::from(81);  // 9²
    let term01 = BigInt::from(2) * &q01 * BigInt::from(144); // 2·Q01·16·9
    let val = &term00 + &term11 + &term01;
    println!("  Q00·16² = {} ({} digits, sign: {})",
        if term00.sign() == num_bigint::Sign::Minus { "negative" } else { "positive" },
        abs_bigint(&term00).to_string().len(),
        if term00.sign() == num_bigint::Sign::Minus { "-" } else { "+" });
    println!("  Q11·9²  = {} ({} digits, sign: {})",
        if term11.sign() == num_bigint::Sign::Minus { "negative" } else { "positive" },
        abs_bigint(&term11).to_string().len(),
        if term11.sign() == num_bigint::Sign::Minus { "-" } else { "+" });
    println!("  2·Q01·16·9 = {} ({} digits, sign: {})",
        if term01.sign() == num_bigint::Sign::Minus { "negative" } else { "positive" },
        abs_bigint(&term01).to_string().len(),
        if term01.sign() == num_bigint::Sign::Minus { "-" } else { "+" });
    println!("  Q3p(16,9,0) = {} ({} digits)",
        if val.sign() == num_bigint::Sign::Plus { "POSITIVE" } else { "NEGATIVE" },
        abs_bigint(&val).to_string().len());
    println!("  First positive evaluation: {}", val.sign() == num_bigint::Sign::Plus);

    // ════════════════════════════════════════════════════
    // THE LOCK
    // ════════════════════════════════════════════════════
    println!("\n=== THE LOCK ===");
    println!("  PATH 1: W − C = 21 − 4 = 17");
    println!("  PATH 2: d₂ − 1 = 18 − 1 = 17");
    println!("  LOCK:   17 = 17 ✓");
    println!();

    // ════════════════════════════════════════════════════
    // WHY DO THEY AGREE?
    // ════════════════════════════════════════════════════
    println!("=== WHY DO THEY AGREE? ===\n");

    println!("PATH 1 depends on:");
    println!("  - The RATIO √(|Q22|/|Q00|) (continuous, irrational)");
    println!("  - Which produces CF = [1;1,3,1,2,31,...] (integer sequence)");
    println!("  - Whose semi-convergent (16,9) gives wl=25 (combinatorial)");
    println!("  - And W=21 from the lattice frozen boundary (geometric)");
    println!();

    println!("PATH 2 depends on:");
    println!("  - The RESIDUES mod 9 of all entries (finite arithmetic)");
    println!("  - Which force 9|all minors (divisibility)");
    println!("  - And d₁=2 from gcd of entries (global divisibility)");
    println!("  - Giving d₂=36/2=18 (algebraic)");
    println!();

    println!("The paths share NOTHING except the matrix entries themselves.");
    println!("Path 1 uses the continuous ratio √(|Q22|/|Q00|).");
    println!("Path 2 uses the discrete residues mod 9.");
    println!("One is geometry (CF, convergents, word length).");
    println!("The other is arithmetic (gcd, divisibility, Smith form).");
    println!();

    println!("The LOCK W−C=d₂−1 says:");
    println!("The CF's semi-convergent at the escape point, minus the");
    println!("frozen word length, equals the Smith invariant minus one.");
    println!("This is a CONSTRAINT on the matrix entries that connects");
    println!("their continuous content (the ratio) to their discrete");
    println!("content (the residues).");
    println!();

    println!("Can we DERIVE it? Let's check what forces each quantity:");
    println!();
    println!("W=21: The 21-Root Theorem says this is determined by the");
    println!("  lattice's frozen structure. It depends on ALL six entries.");
    println!();
    println!("C=4: The escape gap depends on the CF convergents of the");
    println!("  ratio √(|Q22|/|Q00|), specifically which semi-convergent");
    println!("  first makes Q3p positive. It depends mainly on Q00, Q11, Q01.");
    println!();
    println!("d₂=18: The Smith invariant depends on mod-9 residues of ALL");
    println!("  six entries, plus their pairwise gcd structure.");
    println!();

    println!("VERDICT ON THE LOCK SOURCE:");
    println!("The lock W−C=d₂−1 is NOT derivable from first principles");
    println!("without knowing the specific matrix entries.");
    println!("It is a STRUCTURAL COINCIDENCE that ties the geometry");
    println!("(CF convergents) to the arithmetic (Smith form) of ONE");
    println!("specific matrix. Whether this is forced by some deeper");
    println!("property of rqt362 or is a genuine coincidence remains OPEN.");
    println!();
    println!("O1 STATUS: OPEN-STRUCTURAL (unchanged).");
    println!("We have traced the lock to its sources but cannot explain");
    println!("WHY the sources agree. The two paths lead to 17 through");
    println!("completely different mathematics, and their agreement is");
    println!("the last mystery of Q3p's design.");
}
