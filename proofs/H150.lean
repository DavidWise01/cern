import Mathlib
namespace H150
-- For r=1/2, two Walsh characters bound the failed-domain gap
-- by 6a and 3b; the budget then implies gap <= 3/2.
theorem half_independent_upper
    (a b gap : ℚ)
    (budget : 4*a+6*b=4)
    (ha : gap<=6*a)
    (hb : gap<=3*b) : gap<=3/2 := by
  linarith
theorem half_independent_attained :
    4*(1/4:ℚ)+6*(1/2:ℚ)=4 ∧
    6*(1/4:ℚ)=3/2 ∧
    3*(1/2:ℚ)=3/2 := by norm_num
end H150
