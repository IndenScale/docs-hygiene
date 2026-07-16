---
id: SPEC-002-C-003
status: baselined
---

# C-003 Activation Order

`disabled` always yields inactive and `required` always yields error. In `auto`,
explicit feature policy and structural applicability outrank scale signals;
scale alone yields at most advisory.
