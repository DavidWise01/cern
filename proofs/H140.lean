import Mathlib
namespace H140
theorem exchange_bound (a b g : ℝ) (ha : |a| ≤ 1) (hb : |b| ≤ 1)
    (hg : 0 ≤ g) : |g*((a+b)/2)| ≤ g := by
  rw [abs_mul, abs_of_nonneg hg]
  have hs : |(a+b)/2| ≤ 1 := by
    have ht := abs_add_le a b
    rw [abs_div]
    norm_num
    linarith
  simpa using mul_le_mul_of_nonneg_left hs hg
theorem reserve_split (e x y h a : ℝ)
    (ha : a+e ≤ h) (hx : x+y ≤ e) : a+x+y ≤ h := by linarith
end H140
