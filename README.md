This is just some notes I've taken while working this, not necessarily final
thoughts or conclusions.

Principles:

- Don't ignore errors
- That includes not catching things you didn't intend to catch
- Don't rely on undefined behavior, including undocumented exceptions
- Don't lose control, so maintain control loop at least

History:

- https://en.wikipedia.org/wiki/Kernel_panic
- https://man.freebsd.org/cgi/man.cgi?panic(9) - "bring down system on fatal error" - "The panic() and vpanic()	functions do not return."
- Lua 2002/04/16 - "\`panic' function configurable via API" - https://github.com/lua/lua/commit/c11d374c592d10b8ed649ffe501191039ee18757
- Also used less formally in Perl
