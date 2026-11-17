import Mathlib

namespace H078
abbrev K := ℝ × ℝ × ℝ × ℝ
abbrev S := ℝ × ℝ × ℝ

def shadow (p : K) : S := (p.1, p.2.1, p.2.2.1)
def lift (s : S) (w : ℝ) : K := (s.1, s.2.1, s.2.2, w)
def normSq (p : K) : ℝ := p.1^2 + p.2.1^2 + p.2.2.1^2 + p.2.2.2^2
def turn (p : K) : K := (-p.2.2.2, p.2.1, p.2.2.1, p.1)

theorem shadow_lift (s : S) (w : ℝ) : shadow (lift s w) = s := by
  rcases s with ⟨x,y,z⟩
  rfl

theorem shadow_surjective : Function.Surjective shadow := by
  intro s
  exact ⟨lift s 0, shadow_lift s 0⟩

theorem shadow_not_injective : ¬ Function.Injective shadow := by
  intro h
  have e : lift (0,0,0) 0 = lift (0,0,0) 1 := h (by rfl)
  have ew := congrArg (fun p : K => p.2.2.2) e
  norm_num [lift] at ew

theorem turn_preserves_norm (p : K) : normSq (turn p) = normSq p := by
  simp only [normSq, turn]
  ring

theorem turn_period_four (p : K) : turn (turn (turn (turn p))) = p := by
  rcases p with ⟨x,y,z,w⟩
  simp [turn]

theorem two_shadows_determine_kernel (p q : K)
    (h₁ : shadow p = shadow q) (h₂ : shadow (turn p) = shadow (turn q)) : p = q := by
  rcases p with ⟨x,y,z,w⟩
  rcases q with ⟨a,b,c,d⟩
  simp [shadow, turn, Prod.mk.injEq] at h₁ h₂ ⊢
  exact ⟨h₁.1, h₁.2.1, h₁.2.2, h₂.1⟩

end H078
