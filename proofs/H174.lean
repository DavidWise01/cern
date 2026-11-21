import Mathlib
namespace H174
theorem routing_conserves (q a b c : ℚ) (h : a + b + c = 1) :
 q*a + q*b + q*c = q := by
 calc
  q*a + q*b + q*c = q*(a+b+c) := by ring
  _ = q := by rw [h]; ring
theorem mixing_conserves (a b c t : ℚ) :
 let avg := (a+b+c)/3
 ((1-t)*a+t*avg)+((1-t)*b+t*avg)+((1-t)*c+t*avg)=a+b+c := by
 dsimp
 ring
theorem extraction_conserves (energy fraction : ℚ) :
 energy*fraction+energy*(1-fraction)=energy := by ring
end H174
