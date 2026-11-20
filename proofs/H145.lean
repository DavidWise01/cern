import Mathlib
namespace H145
theorem invert_twice (x : ℤ) : -(-x)=x := by simp
theorem reverse_twice {α : Type*} (xs : List α) : xs.reverse.reverse=xs := by simp
end H145
