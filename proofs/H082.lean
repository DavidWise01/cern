import Mathlib
namespace H082
def sign (b : Bool) : ℤ := if b then 1 else -1
abbrev K := ℤ × ℤ × ℤ × ℤ
def normSq (p : K) : ℤ := p.1^2 + p.2.1^2 + p.2.2.1^2 + p.2.2.2^2
def embed (band : Fin 3) (a b c : Bool) : K :=
 let x := sign a
 let y := sign b
 let z := sign c
 if band = 0 then (x,y,z,x*y*z)
 else if band = 1 then (x,y,z,-(x*y*z))
 else if b then (if c then (0,0,0,2*x) else (0,0,2*x,0))
 else (if c then (0,2*x,0,0) else (2*x,0,0,0))
theorem shell (t : Fin 3) (a b c : Bool) :
 normSq (embed t a b c) = 4 := by
 fin_cases t <;> cases a <;> cases b <;> cases c <;> decide
end H082
