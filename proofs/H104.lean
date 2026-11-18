import Mathlib
namespace H104
-- A supporting algebraic identity, not a formalization of Lanczos.
theorem projected_mode {V : Type*} [AddCommGroup V] [Module ℝ V]
    (L : V →ₗ[ℝ] V) (u v w : V) (d h a b c : ℝ)
    (hu : L u = 0) (hv : L v = d • v) (hw : L w = h • w) :
    L (a • u + b • v + c • w) =
      (b*d) • v + (c*h) • w := by
  simp [hu, hv, hw, smul_smul]
end H104
