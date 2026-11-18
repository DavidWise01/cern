import Mathlib
namespace H094
theorem nonvanishing_chart (a b c d : ℝ)
    (h : a^2+b^2+c^2+d^2=1) :
    1/2 ≤ a^2+b^2 ∨ 1/2 ≤ c^2+d^2 := by
  by_contra hn
  push_neg at hn
  linarith

theorem chart_one (a b c d : ℝ) :
    a*(2*(a*c+b*d))+b*(2*(b*c-a*d)) =
      2*(a^2+b^2)*c ∧
    b*(2*(a*c+b*d))-a*(2*(b*c-a*d)) =
      2*(a^2+b^2)*d := by
  constructor <;> ring

theorem chart_two (a b c d : ℝ) :
    c*(2*(a*c+b*d))-d*(2*(b*c-a*d)) =
      2*(c^2+d^2)*a ∧
    d*(2*(a*c+b*d))+c*(2*(b*c-a*d)) =
      2*(c^2+d^2)*b := by
  constructor <;> ring
end H094
