import Mathlib
namespace H186
theorem diagnostic_cost : (3/64 : ℚ)^2 = (9/4096 : ℚ) := by norm_num
theorem no_cost_creation (probe stale : ℚ) :
 probe + stale = stale + probe := by ring
theorem probe_gate (risk cost : ℚ) (h : cost < risk) : cost < risk := h
theorem coast_gate (risk cost : ℚ) (h : risk ≤ cost) : ¬ cost < risk := not_lt.mpr h
end H186
