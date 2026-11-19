import Mathlib
namespace H117
theorem anchored_identity {D : Type*} (trusted a b : D)
    (ha : a = trusted) (hb : b = trusted) : a = b :=
  ha.trans hb.symm
theorem verification_conjunction (integrity valid : Prop)
    (h : integrity ∧ valid) : valid := h.2
end H117
