import Mathlib
namespace H093
def hopf (a b c d : ℝ) : ℝ × ℝ × ℝ :=
  (2*(a*c+b*d), 2*(b*c-a*d), a^2+b^2-c^2-d^2)
theorem hopf_antipodes (a b c d : ℝ) :
    hopf (-a) (-b) (-c) (-d) = hopf a b c d := by
  unfold hopf
  congr 1 <;> ring
theorem hopf_norm (a b c d : ℝ) :
    (2*(a*c+b*d))^2 + (2*(b*c-a*d))^2 +
    (a^2+b^2-c^2-d^2)^2 = (a^2+b^2+c^2+d^2)^2 := by
  ring
end H093
