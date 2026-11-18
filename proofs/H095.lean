import Mathlib
namespace H095
def shadow (a b c d : ℝ) : ℝ × ℝ × ℝ :=
  (2*(a*c+b*d), 2*(b*c-a*d), a^2+b^2-c^2-d^2)
theorem sign_blind (a b c d : ℝ) :
    shadow (-a) (-b) (-c) (-d) = shadow a b c d := by
  unfold shadow
  congr 1 <;> ring
theorem indistinguishable {S O : Type*} (obs : S → O)
    (p q : S) (h : obs p = obs q) (judge : O → Bool) :
    judge (obs p) = judge (obs q) := congrArg judge h
theorem two_honest {S : Type*} (truth a b c : S)
    (h : (a = truth ∧ b = truth) ∨ (a = truth ∧ c = truth) ∨ (b = truth ∧ c = truth))
    (agree : a = b ∧ b = c) : a = truth := by
  rcases h with h | h | h
  · exact h.1
  · exact h.1
  · exact agree.1.trans h.1
end H095
