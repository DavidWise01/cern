import Mathlib
namespace H128
theorem hidden_bound (r d cap : ℝ) (h : r+d ≤ cap) :
    r+d ≤ cap := h
theorem visible_bound (eps cap : ℝ) (h : eps ≤ cap) :
    eps ≤ cap := h
theorem coordinate_containment {r d : Fin 6 → ℝ} (i : Fin 6)
    (eps cap : ℝ) (he : eps ≤ cap) (hh : r i+d i ≤ cap) :
    ∀ j : Fin 6, (if j = i then r j+d j else eps) ≤ cap := by
  intro j
  by_cases h : j = i
  · subst j
    simpa using hh
  · simp [h, he]
end H128
