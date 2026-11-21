import Mathlib
namespace H168
theorem kinetic_impulse_pair (j : ℚ) : j + -j = 0 := by ring
theorem angular_recoil (a : ℚ) : a + -a = 0 := by ring
theorem kinetic_budget (E previousK nextK ejectaK : ℚ) :
  (E - ((nextK - previousK) + ejectaK)) + nextK + ejectaK = E + previousK := by ring
theorem two_port_torque (r j : ℚ) :
  r*j + (-r)*(-j) = 2*r*j := by ring
end H168
