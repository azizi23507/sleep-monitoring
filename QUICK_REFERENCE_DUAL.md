# Quick Reference - Dual Approach

---

## 🚀 Local Development (No Setup)

```bash
# Clone and run
git clone repo
docker-compose up

# Access
http://localhost:3000
```

**Credentials:** Hardcoded in docker-compose.yml  
**Setup:** None  
**Works:** Immediately ✅

---

## 🔐 CI/CD Testing

### **Option A: Without Secrets (Simple)**

```bash
# Just push
git push origin main
# Uses hardcoded fallback values
```

**Setup:** None  
**Works:** Immediately ✅

---

### **Option B: With Secrets (Professional)**

```bash
# 1. Add to GitHub → Settings → Secrets:
DATABASE_URL (optional)
JWT_SECRET (optional)
REDIS_URL (optional)

# 2. Push
git push origin main
# Uses your secrets
```

**Setup:** One-time  
**Works:** With encrypted secrets ✅

---

## 📊 Quick Comparison

| Feature | Local | CI/CD (No Secrets) | CI/CD (With Secrets) |
|---------|-------|-------------------|---------------------|
| **Setup** | None | None | One-time |
| **Credentials** | Hardcoded | Hardcoded | Encrypted |
| **Use Case** | Demo | Simple testing | Professional testing |

---

## ✅ What Works

- ✅ Local with hardcoded → Always works
- ✅ CI/CD without secrets → Always works (uses fallback)
- ✅ CI/CD with secrets → Works (uses your secrets)

**All three work!** Choose based on your needs. 🎉

---

**Full Guide:** See `DUAL_APPROACH_GUIDE.md`
