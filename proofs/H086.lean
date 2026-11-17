import Mathlib
namespace H086
-- Abstract fixed-vector exclusion; invertibility must be established separately.
theorem fixed_vector_exclusion {V : Type*} [AddCommGroup V]
    [Module ℂ V] (H : V →ₗ[ℂ] V)
    (hinj : Function.Injective (H - LinearMap.id))
    (v : V) (hv : H v = v) : v = 0 := by
  apply hinj
  simp [hv]
end H086
