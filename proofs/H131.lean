import Mathlib
namespace H131
theorem scale_transport (s t a u d : ℝ) (hs : s ≠ 0) :
    (t/s)*(a*(s*u)+s*d) = t*(a*u+d) := by
  field_simp
  <;> ring
theorem containment (r d : ℝ) (hr : r ≤ 1) (hd : d ≤ 1/4) :
    (3/4)*r+d ≤ 1 := by linarith
end H131
