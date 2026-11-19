import Mathlib
namespace H115
theorem monotone_step (a x y f g : ℚ)
    (ha : 0 ≤ a) (hxy : x ≤ y) (hfg : f ≤ g) :
    a*x+f ≤ a*y+g :=
  add_le_add (mul_le_mul_of_nonneg_left hxy ha) hfg

theorem unsafe_certificate (p q r : ℚ)
    (h : r < p) (hm : p ≤ q) : r < q :=
  lt_of_lt_of_le h hm

theorem value_certificate (v u best descendant : ℚ)
    (h : descendant ≤ v+u) (hb : v+u ≤ best) :
    descendant ≤ best := le_trans h hb
end H115
