import Mathlib
namespace H121
theorem dual_shadow_injective (a b c d e f g h : ℝ)
    (hA : (a,b,c) = (e,f,g))
    (hB : (b,c,d) = (f,g,h)) :
    (a,b,c,d) = (e,f,g,h) := by
  have ha := congrArg Prod.fst hA
  have hb := congrArg (fun p => p.2.1) hA
  have hc := congrArg (fun p => p.2.2) hA
  have hd := congrArg (fun p => p.2.2) hB
  simp_all
theorem mixed_recovery (a b q m : ℝ)
    (hm : m = a+b+2*q) : q = (m-a-b)/2 := by linarith
end H121
