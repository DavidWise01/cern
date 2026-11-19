import Mathlib
namespace H114
theorem unsafe_superset (a b r : ℝ) (hab : a ≤ b) (h : r < a) :
    r < b := lt_of_lt_of_le h hab
theorem value_prune (v u best candidate : ℝ)
    (hc : candidate ≤ v+u) (hb : v+u ≤ best) :
    candidate ≤ best := le_trans hc hb
end H114
