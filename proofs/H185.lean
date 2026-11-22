import Mathlib
namespace H185
theorem pulse_cost : (3/64:ℚ)^2 = 9/4096 := by norm_num
theorem probe_worthwhile (benefit cost : ℚ) (h : cost < benefit) :
  cost < benefit := h
theorem stopping_gate (benefit cost : ℚ) (h : benefit ≤ cost) :
  ¬ cost < benefit := not_lt.mpr h
theorem cost_partition (probe_cost ambiguity_loss : ℚ) :
  probe_cost + ambiguity_loss = ambiguity_loss + probe_cost := by ring
end H185
