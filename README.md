# Concept: Safe SSH logins using SSH certificates and host-scoped SSH principals

## Goals
- Users cannot use a cert on a host unless that host has approved them
- Approval happens on the host itself
- For simplicity, same user CA can be used everywhere
- Malicious clients gain nothing
- No cert is ever “bound” to a host — instead the host binds itself to the cert

## Prerequisites

**sshd-config**

```
TrustedUserCAKeys /etc/ssh/user_ca.pub

AuthorizedPrincipalsCommand /usr/libexec/ssh-principals %u %h
AuthorizedPrincipalsCommandUser ssh-principals
```

**Host-scoped principals in all user certs**

```
alice@web01
bob@host123
```

## Policies

### [Time-limited approvals (host-enforced TTL)](src/time-based/bin/main.rs)

**Policy idea**

`“Alice may log in to this host for the next 2 hours.”`
