QUESTION 1
At 'src/scanner.rs:133', I use if self.peek() == '.' && self.peek_next().is_ascii_digit()to ensure that the scanner only takes in the '.' if it follwed by a digit therefore the'.' is part of the number. If the input is 5.,number() reads 5, then sees'.' but peek_next() returns '\0' at the end of input, so the condition fails. Section 1.4 requires exactly that a number is digits with an optional '.' and one or more digits, and 5. scans as 5 followed by an error, because . begins no token in Kobo.

QUESTION 2
The counter starts at 1 (src/scanner.rs:12) and changes in two places: src/scanner.rs:98 for a newline between tokens, and src/scanner.rs:110 for a newline inside a string. For a file ending print 1; followed by two blank lines, the counter reaches 3 by the end. The EOF token still carries line 1, because src/scanner.rs:35 takes the line from self.tokens.last(), not from self.line. Section 6.1 asks for this so that trailing newlines left by an editor don't move a line number. This matches the rule for at end parse errors, which are reported at the last real token. A file with no tokens falls back to 1 through map_or(1, ...).

QUESTION 3
The test for 'unexpected_character.kobo' failed at first because the message did not match the requirement. The wrong line was src/scanner.rs:112  _ => self.error(self.line, "Unexpected character"), in commit [completed scan token].
The fixed line is src/scanner.rs:112 in commit [completed test], here I tried to run my code and was met with "stderr: expected '[line 1] Error: Character is not part of any token.'"
I made changes to my code replacing "unexpected character" with "character is not part of my token" and this time all tests passed with ease
