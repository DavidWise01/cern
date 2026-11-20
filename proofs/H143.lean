import Mathlib
namespace H143
theorem split (a b : ℚ) : (a+b)/2+(a-b)/2=a := by ring
theorem orthogonal_pair (a b : ℚ) :
 ((a+b)/2)*((a-b)/2)+((a+b)/2)*((b-a)/2)=0 := by ring
theorem no_single_subshell (l : ℕ) : 2*(2*l+1) ≠ 12 := by omega
end H143
