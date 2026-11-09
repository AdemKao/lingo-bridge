# Security Policy

## Supported versions

Lingo Bridge is currently an **experimental alpha**. No stable releases are
supported, and no security response-time guarantees are available.

## Report a vulnerability

Please **do not create a public Issue** containing a vulnerability,
unredacted API key, call recording, personal transcript or exploit detail.

If available, use GitHub's **Report a vulnerability / Private vulnerability
reporting** feature in the repository's Security tab. Otherwise, open a
non-sensitive issue requesting a private channel without posting details.

Useful information: affected version, OS, a minimal safe reproduction, and
the impact. Do not include actual API keys or another person's conversations.

## Design expectations

- API keys are user-provided and must not ship inside installers.
- Audio must not start streaming without deliberate activation and consent.
- Third-party cloud translation sends audio outside the LINE channel.
- The app and virtual cable configuration require real-device privacy tests.
- Dependencies, downloaded models and any driver redistribution need review.
