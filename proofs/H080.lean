import Mathlib
namespace H080
variable {G : Type*} [Group G]
def link (g h u : G) : G := g * u * h⁻¹
theorem plaquette_covariant (a b c d u v w t : G) :
    link a b u * link b c v * link c d w * link d a t =
      a * (u * v * w * t) * a⁻¹ := by
  unfold link
  group
theorem observable_invariant {O : Type*} (f : G → O)
    (hf : ∀ g p : G, f (g * p * g⁻¹) = f p)
    (a b c d u v w t : G) :
    f (link a b u * link b c v * link c d w * link d a t) =
      f (u * v * w * t) := by
  rw [plaquette_covariant]
  exact hf a _
end H080
