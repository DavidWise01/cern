import Mathlib
namespace H101
abbrev Address := ZMod 24 × ZMod 24
def theta (p : Address) : Address := (p.1+1,p.2)
def phi (p : Address) : Address := (p.1,p.2+1)
theorem commute (p : Address) : theta (phi p) = phi (theta p) := by rfl
theorem count : Fintype.card Address = 576 := by decide
end H101
