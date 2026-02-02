# Concept: Safe Just-In-Time (JIT) SSH access with dual-control enforcement using SSH certificates and host-scoped SSH principals

## Goals
- Users cannot use a cert on a host unless that host has enabled them
- Users explicitly enable access ahead of time (on the host)
- Same user CA can be used everywhere (for simplicity)
- Both sides can independently control the allowed login time
- Access automatically expires

## Prerequisites

**sshd-config**

```
TrustedUserCAKeys /etc/ssh/user_ca.pub

AuthorizedPrincipalsCommand /usr/libexec/ssh-principals %u
AuthorizedPrincipalsCommandUser ssh-principals
```

**Host-scoped principals in all user certs**

```
alice@web01
bob@host123
```

**sudo rule for all users that are allowed to enable logins**

`%ssh-login-enablers ALL=(root) NOPASSWD: /usr/local/bin/ssh-enable-login [0-9]+[0-9]*[mh]`

