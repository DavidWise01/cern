import Mathlib
namespace H113
theorem selection_safe (a b c r : ℝ) (hc : 0 ≤ c)
    (h : a+b+c ≤ r) : a+b ≤ r := by linarith
theorem optimum_dominates_baseline {S : Type*}
    (safe : S → Prop) (value : S → ℝ) (best base : S)
    (hopt : ∀ s, safe s → value s ≤ value best)
    (hb : safe base) : value base ≤ value best :=
  hopt base hb
end H113
