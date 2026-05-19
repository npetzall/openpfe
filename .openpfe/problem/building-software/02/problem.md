---
problems: {}
---

# Problem

## Summary

Using OpenSpec (or similar), the team added many features and ended up with overlapping, awkward specs—so the spec set stopped being a trustworthy map of the system.

## Longer problem

Feature-level specs multiplied in parallel without a clear rule for boundaries and ownership. Descriptions started to cover the same behaviors in different words, or implied conflicting expectations. The spec corpus became expensive to read and risky to follow: hard to know which file is authoritative, and easy to "specify" the same slice of product twice.

## Open questions

- Were overlaps introduced by branching, copy-paste, or unclear feature seams?
- Is the pain worse at write time, review time, or execution (implementation) time?
