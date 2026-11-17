import Mathlib
namespace H083
variable {G : Type*} [Group G]
def link (a b u : G) : G := a * u * b⁻¹
theorem triangle_covariant (a b c u v w : G) :
    link a b u * link b c v * link c a w =
    a * (u * v * w) * a⁻¹ := by
  unfold link
  group
theorem triangle_invariant {O : Type*} (f : G → O)
    (hf : ∀ g p : G, f (g * p * g⁻¹) = f p)
    (a b c u v w : G) :
    f (link a b u * link b c v * link c a w) =
    f (u * v * w) := by
  rw [triangle_covariant]
  exact hf a _
end H083
