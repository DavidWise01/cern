import Mathlib
namespace H126
theorem stale_bound (old current noise eps delta : ℝ)
    (hn : |noise| ≤ eps)
    (hd : |old-current| ≤ delta) :
    |old+noise-current| ≤ eps+delta := by
  rw [show old+noise-current = noise+(old-current) by ring]
  exact le_trans (abs_add_le _ _) (add_le_add hn hd)

theorem minimax (r h : ℝ) (hh : 0 ≤ h) :
    h ≤ max |r-h| |r+h| := by
  by_cases hr : 0 ≤ r
  · rw [abs_of_nonneg (add_nonneg hr hh)]
    exact le_trans (by linarith : h ≤ r+h) (le_max_right _ _)
  · have hn : r-h ≤ 0 := by linarith
    rw [abs_of_nonpos hn]
    exact le_trans (by linarith : h ≤ -(r-h)) (le_max_left _ _)
end H126
