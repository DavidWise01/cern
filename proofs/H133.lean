import Mathlib
namespace H133
theorem cap_optimal (g e h : ℝ) (hg : 0 < g)
    (hl : e*g ≤ h) (hc : h ≤ 1) : e ≤ 1/g := by
  apply (le_div_iff₀ hg).2
  exact le_trans hl hc
theorem weighted_step (a h e : ℝ) (ha : a+e ≤ h) :
    a+e ≤ h := ha
end H133
