import Mathlib
namespace H178
-- Supporting algebra only; this is not a full verified phase controller.
theorem clamp_upper (u cap : ℚ) (hc : 0 ≤ cap) :
 min cap (max (-cap) u) ≤ cap := min_le_left _ _
theorem clamp_lower (u cap : ℚ) (hc : 0 ≤ cap) :
 -cap ≤ min cap (max (-cap) u) := by
  exact le_min (le_of_lt (neg_lt_self hc)) (le_max_left _ _)
theorem common_mode_disappears_from_difference (a b drift : ℚ) :
 (a + drift) - (b + drift) = a - b := by ring
theorem estimated_phase_accounting (phase correction drift : ℚ) :
 (phase + drift - correction) + correction = phase + drift := by ring
end H178
