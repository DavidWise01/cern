import Mathlib
namespace H146
theorem antipode_twice (a b c d : Bool) :
    (!(!a), !(!b), !(!c), !(!d)) = (a,b,c,d) := by simp
end H146
