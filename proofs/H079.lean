import Mathlib
namespace H079
abbrev K := ℝ × ℝ × ℝ × ℝ
inductive Plane where
 | xy | xz | xw | yz | yw | zw
def turn (p : K) (a : Plane) : K :=
 let (x,y,z,w) := p
 match a with
 | .xy => (-y,x,z,w)
 | .xz => (-z,y,x,w)
 | .xw => (-w,y,z,x)
 | .yz => (x,-z,y,w)
 | .yw => (x,-w,z,y)
 | .zw => (x,y,-w,z)
def normSq (p : K) : ℝ :=
 p.1^2 + p.2.1^2 + p.2.2.1^2 + p.2.2.2^2
theorem norm_conserved (p : K) (a : Plane) :
 normSq (turn p a) = normSq p := by
 rcases p with ⟨x,y,z,w⟩
 cases a <;> simp [turn, normSq] <;> ring
theorem period_four (p : K) (a : Plane) :
 turn (turn (turn (turn p a) a) a) a = p := by
 rcases p with ⟨x,y,z,w⟩
 cases a <;> simp [turn]
end H079
