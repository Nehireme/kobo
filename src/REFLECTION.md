QUESTION 1
At 'src/scanner.rs:133', I use if self.peek() == '.' && self.peek_next().is_ascii_digit()to ensure that the scanner only takes in the '.' if it follwed by a digit therefore the'.' is part of the number. If the input is 5.,number() reads 5, then sees'.' but peek_next() returns '\0' at the end of input, so the condition fails. Section 1.4 requires exactly that a number is digits with an optional '.' and one or more digits, and 5. scans as 5 followed by an error, because . begins no token in Kobo.

QUESTION 2
The counter starts at 1 (src/scanner.rs:12) and changes in two places: src/scanner.rs:98 for a newline between tokens, and src/scanner.rs:110 for a newline inside a string. For a file ending print 1; followed by two blank lines, the counter reaches 3 by the end. The EOF token still carries line 1, because src/scanner.rs:35 takes the line from self.tokens.last(), not from self.line. Section 6.1 asks for this so that trailing newlines left by an editor don't move a line number. This matches the rule for at end parse errors, which are reported at the last real token. A file with no tokens falls back to 1 through map_or(1, ...).

QUESTION 3
The test for 'unexpected_character.kobo' failed at first because the message did not match the requirement. The wrong line was src/scanner.rs:112  _ => self.error(self.line, "Unexpected character"), in commit [completed scan token].
The fixed line is src/scanner.rs:112 in commit [completed test], here I tried to run my code and was met with "stderr: expected '[line 1] Error: Character is not part of any token.'"
I made changes to my code replacing "unexpected character" with "character is not part of my token" and this time all tests passed with ease


PARSER REFLECTION 

1. The grammar rule for if statements is ambiguous when nested without explicit block syntax: if (cond) stmt (else stmt)?. When an inner if statement is parsed recursively inside the then branch of an outer if, the inner call to if_statement executes self.matches(&[TokenType::Else]) at src/parser.rs:136 first. Because matches eagerly consumes the else token as soon as it sees it on the stream, the inner if claims the else before the outer if_statement call regains control of the execution stack.
The parser would print (if false (if true (print "a") (else (print "b")))) for if (false)
if (true) print "a"; else print "b". 
The parser would print (if false (if true (print "a")) (else (print "b")))  if the else bound to
the outer if instead

2. src/parser.rs:181 The grammar is not 
LL(1). Looking at a sequence like a = 1; versus a + 1; or a() = 1;, the parser sees the identifier a first in all cases. It cannot know whether a is an assignment target or an expression operand until it parses a as an expression and checks if the following token is =. Because expressions of arbitrary complexity (e.g. a.b.c or a[0]) can appear on the left before encountering =, the parser must construct the AST node first and then validate that it represents a valid L-value (Expr::Variable).
The message that 1=2 produces is [line 1] Error at '=': Invalid assignment target.

3. src/parser.rs:348–364 (fn synchronise(&mut self))
TokenType::Semicolon: The parser stops immediately after consuming a semicolon. Semicolons terminate statements in Lox, so advancing past one cleanly resets the parser state to look for the next statement.
TokenType::Fun, TokenType::Var, TokenType::If, TokenType::While, TokenType::Print, TokenType::Return: The parser stops when peek() is on any of these statement/declaration keywords without consuming them. These keywords strictly mark the start of new top-level declarations or control structures.
With synchronise(): The parser discards the corrupted expression tokens until reaching the next statement boundary, reporting exactly 1 error:[line 1] Error at 'print': Expect ';' after expression.
Without synchronise(): The parser fails to advance past the missing semicolon, cascading into multiple false-positive errors on downstream tokens and failing to make progress on subsequent statements (reporting cascading syntax errors across lines).