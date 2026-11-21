import Mathlib
namespace H180
-- Algebraic reconstruction when a constant drift and the applied command are known.
theorem drift_recovery (theta_prev theta_next u drift : ℚ)
  (h : theta_next = theta_prev + drift - u) :
  theta_next - theta_prev + u = drift := by linarith

theorem history_reconstruction (initial velocity used : ℚ) :
  (initial + velocity - used) + used = initial + velocity := by ring

theorem bounded_correction_upper (c proposed : ℚ) :
  min c (max (-c) proposed) ≤ c := min_le_left _ _
end H180
