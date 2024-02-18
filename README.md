# Rust_Deduction_Machine
Rust programming language with working toward creating a program which can validate deductions

Lab2 - Deduction Machine for Predicates:

Instructions: 

The objective for this lab is to learn to work with ChatGPT and develop a deeper understanding of the Rust programming language with working toward creating a program which can validate deductions.  The program will be written in Rust using ChatGPT as a programming assistant.   The directions for the program are below.   It is going to take several iterations with GPT to get the program completed.   You are free to do your own coding at any point as well (ChatGPT is only the assistant after all).   Spend sufficient, but not excess time on this lab.    

 

Program Directions:

The program should accept as an input a series of premises followed by a deduction.   The program will then validate the deduction and determine whether it is valid or invalid.   Once a determination has been made, the program will output the demonstration (proof) that the deduction was valid or invalid.

Premises will be entered into the program a single line at a time followed by the deduction.

(m ∧ ¬b) → j

(f ∨ s) → m

b → t

f → ¬t

f

∴ j

Although Rust uses UTF-8 for characters, it is not supported by most terminals. Instead of the UTF-8 characters use the following table of substitutes:

character  Unicode Value  Substitute
→          U+2192          > 
¬          U+00AC          ~
∧          U+2227          *
∨          U+2228          +
∀          U+2200          A
∃          U+2203          E
∴          U+2234          R

 
The program will operate by considering the last premise, that premise will be combined in turn with each known premise and evaluated against known rules.  Each of these may produce a new premise.

If there exists an application of a rule which results in the deduction, then the deduction would be determined to be valid.   If there is a rule which results in the negative of the deduction, then the deduction would be determined to be invalid.  If neither the deduction, nor the negative of the deduction is reached, then the argument would be determined to have insufficient information.

The set of rules to be applied includes any valid rules, especially those in the textbook.  That would include: modus ponens, modus tollens, the Law of Syllogism, the four rules given toward the bottom of page 40 (these also appear in problem 2 on page 44).   

For the example above (Note:  there may be some premises which appear that are not mentioned, but which are not used in reaching a conclusion.):

The program would start with the premise f.   Each of the preceding premises will be paired and all known rules would be applied.   For the first premise, no rules would fit the format (as f does not appear).  For the second premise, (f ∨ s) → m, the result would be a premise m because f ∨ s exists (modus ponens).  This premise would be added to the list.  We will also add the rule and the premise (or identifiers for them) to the list so we can remember how we got the premise. We check whether it matches the deduction or is a negation of the deduction.  For the third premise, no rules would fit. For the fourth premise, we would achieve the new premise ¬t which is added to the list.  

Having applied rules which fit against the premise f, we would now move to the bottom of the expanded list and repeat the process.  In this case we would use the premise ¬t, ¬t and m result in m ∨ ¬t. That premise is added to the list.  ¬t and f → ¬t would match and give f, but since we already have the premise f, we would ignore it.  ¬t and b → t would match the rule modus tollens giving the premise ¬b. This would be checked for validity and added to the list (along with rule and premise information).  

Next the premise at the bottom of the list, ¬b, would be tested against the other premises on the list.   We would gain the premise m ∧ ¬b.  In the next round, that premise along with (m ∧ ¬b) → j would give us the premise j, which is valid.   At this point we would back out of our function calls, providing at each information about the last premise we created and the rule we used to create it.  Those are then printed in the reverse order to give a “proof”.
