import Mathlib
namespace H176
theorem spread_contract_no_noise (a b k : ℚ) :
 ((1-k)*a+k*(a+b)/2)-((1-k)*b+k*(a+b)/2) = (1-k)*(a-b) := by ring
theorem rotor_housing_reaction (r h j : ℚ) :
 (r+j)+(h-j)=r+h := by ring
theorem shaft_reaction (r e d : ℚ) :
 (r-d)+(e+d)=r+e := by ring
theorem energy_ledger (supplied fuel kinetic work heat : ℚ)
 (h : supplied = fuel+kinetic+work+heat) :
 supplied-(fuel+kinetic+work+heat)=0 := by rw [h]; ring
end H176
