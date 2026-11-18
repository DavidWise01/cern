import Mathlib
namespace H106
theorem containment_scalar (d v r : ℝ) (h : r ≤ -2*d*v)
    (hd : 0 ≤ d) (hv : 0 ≤ v) : r ≤ 0 := by
  have hnd : 0 ≤ d*v := mul_nonneg hd hv
  linarith
theorem skew_pair (a b s : ℝ) : a*(s*b)+b*(-s*a)=0 := by ring
end H106
