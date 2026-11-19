import Mathlib
namespace H118
theorem authorization_gate (integrity validity signature scope fresh : Prop)
    (h : integrity ∧ validity ∧ signature ∧ scope ∧ fresh) :
    validity ∧ scope ∧ fresh := ⟨h.2.1, h.2.2.2⟩
theorem stale_epoch (issued current : ℕ) (h : issued < current) :
    issued ≠ current := Nat.ne_of_lt h
end H118
