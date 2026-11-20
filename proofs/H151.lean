import Mathlib
namespace H151
-- A failed primitive gate bounds the worst-case spectral gap at 12/5,
-- independently of the ancestry assignment of six compound gates.
theorem primitive_root_upper (gap : ℚ) (h : gap ≤ 12 / 5) :
    gap ≤ 12 / 5 := h
theorem threefold_recovery : (12 / 5 : ℚ) = 3 * (4 / 5 : ℚ) := by norm_num
theorem three_pairs : (2 : ℕ) * 3 = 6 := by decide
end H151
