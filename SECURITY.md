# Security Policy

Idalion is security-sensitive infrastructure.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting / Security Advisory flow for this
repository.

## Adapter security review

Adapter PRs must document:

- peer identity and authentication assumptions;
- transport encryption;
- relay/bootstrap trust assumptions;
- metadata leakage;
- persistence of keys or peer state;
- dependency/runtime risks.

Idalion does not treat transport encryption as equivalent to application-level
end-to-end encryption.
