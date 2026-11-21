import Mathlib
namespace H170
theorem charging_balance (fuel oldK newK exhaust : ℚ) :
 (fuel - ((newK-oldK)+exhaust)) + newK + exhaust = fuel + oldK := by ring
theorem load_balance (oldK newK : ℚ) :
 newK + (3/4:ℚ)*(oldK-newK) + (1/4:ℚ)*(oldK-newK) = oldK := by ring
theorem angular_transfer (rotor receiver removed : ℚ) :
 (rotor-removed)+(receiver+removed)=rotor+receiver := by ring
theorem recharge_is_not_free (oldSupply recharge : ℚ) :
 oldSupply + recharge = recharge + oldSupply := by ring
end H170
