import Mathlib
namespace H081
abbrev State := Fin 3 × Bool × Bool × Bool
abbrev Explicit := Fin 4 ⊕ Fin 6
def flip (s : State) : State :=
  (s.1, !s.2.1, !s.2.2.1, !s.2.2.2)
theorem state_count : Fintype.card State = 24 := by decide
theorem explicit_count : Fintype.card Explicit = 10 := by decide
theorem flip_twice (s : State) : flip (flip s) = s := by
  rcases s with ⟨b,x,y,z⟩
  cases x <;> cases y <;> cases z <;> rfl
end H081
