# minigrep

minigrep is a simpler version of the CLT grep/ripgrep

This is an exercise of the rust programming language book by the Brown university



## Env. variables - higher precedence than CL args
- CASE_SENSITIVE: the same as default         (+)
- IGNORE_CASE: disables case sensitive search (-)

## Command-line arguments - less precedence than Env. variables
- -ic: changes flag to disable case sensitive search (+)
- all other: by default, the match is case sensitive (-)

p.s: "+" means higher precedence than other(s) and "-" means less