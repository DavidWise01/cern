import Mathlib
namespace H119
theorem unique_commit {N R : Type*} (ledger : N → Option R)
    (nonce : N) (a b : R)
    (ha : ledger nonce = some a) (hb : ledger nonce = some b) :
    a = b := by
  exact Option.some.inj (ha.symm.trans hb)
end H119
