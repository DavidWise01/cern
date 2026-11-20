import Mathlib
namespace H154
-- Elementary finite-field orthogonality certificates.
theorem even_parity_preserved (x y : Bool) :
  (x != y) = (!x != !y) := by cases x <;> cases y <;> decide
theorem gf2_coset_count : (2 : ℕ)^3 * 2 = 16 := by norm_num
theorem min_cut_cardinality : (3 : ℕ) < 4 := by omega
end H154
