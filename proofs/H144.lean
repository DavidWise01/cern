import Mathlib
namespace H144
theorem cross_shadow {A : Type*} [Ring A] (a b : A) :
    (a+b)*(a-b)-(a-b)*(a+b) = 2*(b*a-a*b) := by noncomm_ring
end H144
