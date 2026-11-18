import Mathlib
namespace H099
def addr (a b : Fin 2) (c : Fin 6) : Nat := 12*a.val + 6*b.val + c.val
theorem address_bound (a b : Fin 2) (c : Fin 6) : addr a b c < 24 := by
  unfold addr
  omega
theorem address_unique (a b a' b' : Fin 2) (c c' : Fin 6)
    (h : addr a b c = addr a' b' c') : a = a' ∧ b = b' ∧ c = c' := by
  unfold addr at h
  have ha : a.val = a'.val := by omega
  have hb : b.val = b'.val := by omega
  have hc : c.val = c'.val := by omega
  exact ⟨Fin.ext ha, Fin.ext hb, Fin.ext hc⟩
def polarity (k : Nat) : Int := if k % 2 = 0 then -1 else 1
theorem quarter_polarity (k : Nat) : polarity (k+6) = polarity k := by
  unfold polarity
  split <;> split <;> omega
end H099
