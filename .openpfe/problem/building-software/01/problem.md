---
problems:
  "01": "Architecture reads as a big ball of mud—weak boundaries and entanglement force a wide view."
  "02": "Discovery dominates: context and tokens go to finding what to change, not how or the end state."
---

# Problem

## Summary

In AI-assisted development, the usable context fills too fast, the overall picture gets fuzzy, and token use spikes—so progress feels like it hits a wall even though work is still "happening."

## Longer problem

The team can keep prompting and patching, but the session carries more state than anyone can hold in view at once. That makes trade-offs, dependencies, and intent drift hard to see. Larger prompts and retries also burn tokens quickly, so the same grind shows up as both **cognitive overload** and **cost/throughput pressure**.

## Open questions

- Is the bottleneck mainly model context, human memory, or both?
- Does "big picture" mean architecture, product intent, or task backlog?

## Navigation

**Deeper:**
- [Big ball of mud — unclear boundaries](./01/problem.md)
- [Discovery dominates tokens before "how" or target](./02/problem.md)
