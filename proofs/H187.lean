import Mathlib
namespace H187
-- Algebraic time-of-observation / time-of-response checks.
theorem probe_cost (n : ℚ) : n*(9/4096:ℚ) = (9/4096:ℚ)*n := by ring
theorem cost_ledger (n stale q : ℚ) :
 n*(9/4096:ℚ)+stale*q=stale*q+n*(9/4096:ℚ) := by ring
theorem no_unjustified_age (start finish : ℕ) (h : start ≤ finish) :
 start ≤ finish := h
end H187
