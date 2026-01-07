# Security Guidelines - Sleep Monitoring System

**Last Updated:** January 7, 2026

---

## Current Security Status

### Implemented Security Features

1. **JWT Authentication**
 - Pi devices must authenticate to send data
 - Token-based access control
 - 24-hour token expiration
 - Signed with JWT_SECRET

2. **Input Validation**
 - Range checks on all sensor values
 - Type validation via Rust type system
 - Prevents malformed data

3. **SQL Injection Prevention**
 - All queries use parameterized statements (SQLx)
 - No string concatenation in SQL
 - Type-safe database queries

4. **Structured Logging**
 - All authentication attempts logged
 - Error tracking with context
 - Request tracing for debugging

### Security Warnings - Action Required

#### 1. JWT Secret Configuration

**Current Risk:** Default development secrets in configuration files

**Impact:** Anyone with the default secret can generate valid tokens

**Fix Required:**
```bash
# Generate secure random secret
openssl rand -base64 32

# Set in .env file
JWT_SECRET=<generated-secret-here>

# For Docker, set in docker-compose.yml or .env
JWT_SECRET=<generated-secret-here>
```

**Files to Check:**
- `backend/.env` (if running locally)
- `docker-compose.yml` (if using Docker)
- Never use: `dev-secret-key-for-docker-CHANGE-IN-PRODUCTION`

#### 2. CORS Configuration

**Current Risk:** Allows requests from any origin

**Impact:** Any website can make API requests to your backend

**Fix Required:**

Edit `backend/src/main.rs`:
```rust
// Current (insecure):
.allow_origin(Any)

// Production (secure):
.allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
```

#### 3. Database Credentials

**Current Risk:** Default PostgreSQL password (`password`)

**Impact:** Easy to guess database credentials

**Fix Required:**
```bash
# Generate strong password
openssl rand -base64 24

# Update in docker-compose.yml
POSTGRES_PASSWORD=<generated-password>

# Update in .env
DATABASE_URL=postgres://postgres:<generated-password>@localhost/sleep_monitor
```

---

## Production Deployment Recommendations

Before deploying to production, consider these security improvements:

### Critical Security Actions

1. **Change JWT_SECRET** to strong random value (32+ characters)
 ```bash
 openssl rand -base64 32
 ```

2. **Change database password** from default

3. **Restrict CORS** to specific frontend domain(s)

4. **Enable HTTPS/TLS** on all endpoints (no HTTP)

5. **Enable WSS** for WebSocket (not WS)

6. **Review and remove** any test credentials

### Additional Security Enhancements

- Add rate limiting per device
- Set RUST_LOG=warn or error (not debug)
- Enable Redis password if exposed to network
- Use Docker secrets instead of environment variables
- Set up database backups (automated daily)
- Configure firewall rules
- Add audit logging for sensitive operations

---

## Secure Configuration Examples

### Local Development (.env)

```bash
# Database - local development
DATABASE_URL=postgres://postgres:devpassword123@localhost/sleep_monitor

# JWT Secret - development only
JWT_SECRET=dev-key-not-for-production-use-only

# Redis - local
REDIS_URL=redis://127.0.0.1:6379

# Logging - verbose for development
RUST_LOG=debug
```

### Production (.env or secrets manager)

```bash
# Database - production with TLS
DATABASE_URL=postgres://appuser:$(cat /run/secrets/db_password)@db.prod.internal:5432/sleep_monitor?sslmode=require

# JWT Secret - from secrets manager
JWT_SECRET=$(cat /run/secrets/jwt_secret)

# Redis - with password
REDIS_URL=redis://:$(cat /run/secrets/redis_password)@redis.prod.internal:6379

# Logging - production level
RUST_LOG=warn
```

### Docker Production (docker-compose.prod.yml)

```yaml
services:
 postgres:
 environment:
 POSTGRES_PASSWORD_FILE: /run/secrets/db_password
 secrets:
 - db_password
 restart: always

 redis:
 command: redis-server --requirepass "${REDIS_PASSWORD}"
 restart: always

 backend:
 environment:
 JWT_SECRET_FILE: /run/secrets/jwt_secret
 RUST_LOG: warn
 secrets:
 - jwt_secret
 - db_password
 restart: always

secrets:
 jwt_secret:
 external: true
 db_password:
 external: true
```

---

## Security Best Practices

### 1. Secrets Management

**Never:**
- Commit secrets to Git
- Use default/example credentials in production
- Share secrets via email/chat
- Log secrets in application logs

**Always:**
- Use environment variables or secrets managers
- Rotate secrets regularly (quarterly recommended)
- Use different secrets per environment
- Document where secrets are stored

### 2. Network Security

**Development:**
- Localhost only (127.0.0.1)
- No external access needed

**Production:**
- Use HTTPS/TLS everywhere
- Configure firewall rules
- Use private networks for database/Redis
- Expose only necessary ports

### 3. Database Security

**Access Control:**
- Create application-specific user (not `postgres`)
- Grant minimum required permissions
- Use different credentials per environment

**Example:**
```sql
-- Create limited user
CREATE USER sleep_app WITH PASSWORD 'secure-password';

-- Grant only needed permissions
GRANT CONNECT ON DATABASE sleep_monitor TO sleep_app;
GRANT SELECT, INSERT ON sensor_readings TO sleep_app;
GRANT SELECT, INSERT ON fhir_observations TO sleep_app;
```

### 4. Authentication

**Token Lifecycle:**
- Generate: When Pi device authenticates
- Validate: On every API request
- Expire: After 24 hours
- Revoke: When device is decommissioned

**Implementation:**
```bash
# Pi device gets token once per day
TOKEN=$(curl -X POST http://api.example.com/api/auth/token \
 -H "Content-Type: application/json" \
 -d '{"device_id":"pi-001"}' | jq -r .token)

# Use token for all data submissions
curl -X POST http://api.example.com/api/sensor-data \
 -H "Authorization: Bearer $TOKEN" \
 -H "Content-Type: application/json" \
 -d '{...}'
```

### 5. Monitoring & Alerts

**Monitor:**
- Failed authentication attempts
- Unusual API request patterns
- Database connection failures
- Disk space usage
- Memory usage

**Alert On:**
- Repeated authentication failures
- Service downtime
- Database connection loss
- High error rates

---

## Vulnerability Reporting

If you discover a security vulnerability:

1. **Do NOT** create a public GitHub issue
2. **Do NOT** share details publicly
3. Contact the project maintainers privately
4. Provide:
 - Detailed description
 - Steps to reproduce
 - Potential impact
 - Suggested fix (if any)

---

## Security Updates

### Dependency Updates

**Check for vulnerabilities:**
```bash
# Rust dependencies
cargo audit

# Update dependencies
cargo update
```

**Schedule:** Check monthly, update quarterly or when vulnerabilities found

### System Updates

- PostgreSQL: Follow security advisories
- Redis: Follow security advisories 
- Docker images: Use specific versions, update regularly

---

## Compliance Notes

### HIPAA Compliance Considerations

This system may handle Protected Health Information (PHI). Current implementation provides:

- Authentication and access controls (JWT)
- Input validation and error handling
- SQL injection prevention

For full HIPAA compliance, additional requirements:
- Encryption at rest (database)
- Encryption in transit (TLS/HTTPS)
- Audit logging
- Business Associate Agreements (BAAs)
- Security risk assessment
- Documented policies

**Note:** Consult with compliance experts for healthcare deployments.

### GDPR Compliance Considerations

For personal data processing, current implementation provides:

- Data validation and sanitization
- Secure data storage

Additional considerations for GDPR:
- Document data processing activities
- Implement data minimization
- Add user consent mechanisms
- Enable data export (portability)
- Enable data deletion (right to be forgotten)
- Create privacy policy

---

## References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [FHIR Security](https://www.hl7.org/fhir/security.html)
- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)

---

**Last Updated:** January 7, 2026 
**Version:** 1.0.0


