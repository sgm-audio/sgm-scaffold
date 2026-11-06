# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in `sgm-scaffold`, please report it responsibly:

1. **Do NOT** open a public issue
2. Email the maintainers at: security@sgmstudios.com
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 7 days
- **Fix timeline**: Critical (7 days), High (30 days), Medium/Low (90 days)

## Security Practices

This project follows these security practices:
- All dependencies are audited via `cargo audit`
- License compliance checked via `cargo deny`
- Supply chain verified via `cargo vet`
- No unsafe code without `// SAFETY:` documentation
- Generated projects include security scanning in CI