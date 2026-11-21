import Mathlib
namespace H173
theorem one_shared_input (q a b c : ℚ) (h : a+b+c=1) :
 q*a+q*b+q*c=q := by rw [← mul_add, ← mul_add, h]; ring
theorem redistribution_conserves (a b c t : ℚ) :
 let avg := (a+b+c)/3
 ((1-t)*a+t*avg)+((1-t)*b+t*avg)+((1-t)*c+t*avg)=a+b+c := by
 dsimp
 ring
theorem channel_ledger (e k w h x : ℚ) :
 (e+k+w+h+x)-(e+k+w+h+x)=0 := by ring
end H173
