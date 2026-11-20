import Mathlib
namespace H129
-- Integer-scaled excess costs. Count of yz runs is at most count
-- of other runs plus one because maximal consecutive runs differ.
theorem run_count_bound (slow other : ℕ)
    (h : slow ≤ other + 1) :
    (36 : ℤ) * (other : ℤ) - 36 * (slow : ℤ) ≥ -36 := by
  omega

theorem cycle_optimum_certificate (T cost : ℚ)
    (h : 5*cost ≥ 99*T - 96) :
    cost ≥ (99/5)*T - 96/5 := by linarith
end H129
