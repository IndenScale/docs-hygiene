# Adapter

An adapter lets Docs Hygiene invoke an external documentation tool without
reimplementing that tool's rule system.

Adapters are intentionally thin. They run a command, observe its exit status,
and surface failures as Docs Hygiene diagnostics.
