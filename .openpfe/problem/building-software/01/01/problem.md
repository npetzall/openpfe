---
problems:
  "01": "Too little guidance on boundaries and ownership—the system stays one undifferentiated whole until scale hurts."
  "02": "Tooling and habitual workflow don’t scaffold separation; easy to treat everything as one thing."
---

# Problem

## Summary

The codebase reads as a big ball of mud—boundaries are weak, dependencies sprawl, and parts that should be separable feel glued together.

## Longer problem

"Small change" in intent does not match small change in surface area: ripples run wide because modules are not meaningful seams. That makes it hard to load only the slice that matters; the architecture itself keeps forcing a wider view.

## Open questions

- Is entanglement mostly layering violations, shared state, or fan-out through utilities?
- Did the shape emerge gradually, or jump from rapid AI-assisted edits without refactors?

## Navigation

**Deeper:**
- [Weak guidance—boundaries and ownership underspecified](./01/problem.md)
- [Tooling and workflow favor one undifferentiated mass](./02/problem.md)
