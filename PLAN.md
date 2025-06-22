# Implementation Plan: `jig`

## NOTES

- Encryption and decryption with `age` via `rage`
  - <https://github.com/str4d/rage>
- can we have it so you only need to type enough of a command to disambiguate it? Could bodge this with aliases at a push.


## Command Tree

```mermaid
graph TD
  A[ROOT] --> 1(crypt)
  A --> 3(generate)
  A --> 4(network)
  4 --> 4A(dns)
  A --> 5(format,fmt)
  A --> 6(convert)
  A --> 7(mcp)
```


