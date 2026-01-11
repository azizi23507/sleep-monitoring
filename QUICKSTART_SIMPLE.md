# Quick Start Guide

**Choose your approach:**

---

## 🚀 Path 1: Local Development (2 Minutes)

**Perfect for: Demos, testing, development**

### **Steps:**

```bash
# 1. Clone
git clone https://github.com/yourusername/sleep-monitoring.git
cd sleep-monitoring

# 2. Start
docker-compose up

# 3. Open browser
http://localhost:3000
```

**Done!** ✨

---

## 🧪 Path 2: CI/CD Testing (No Setup Needed)

**Perfect for: Automated testing**

### **What It Does:**

Every `git push` automatically:
- ✅ Runs 30+ tests
- ✅ Checks code quality
- ✅ Security scanning

### **How to Use:**

```bash
# 1. Make changes
git add .
git commit -m "Your changes"

# 2. Push
git push origin main

# 3. Watch results
GitHub → Actions tab
```

**No setup, no secrets, just push!** ✨

---

## 📊 Comparison

| Feature | Path 1: Local | Path 2: CI/CD |
|---------|--------------|--------------|
| **Setup** | None | None |
| **Time** | 2 minutes | Automatic |
| **Use Case** | Run locally | Test automatically |
| **Credentials** | Hardcoded | Hardcoded (test) |

---

## ✅ Success Checklist

### **Path 1 (Local)**
- [ ] Cloned repository
- [ ] Ran `docker-compose up`
- [ ] Accessed http://localhost:3000
- [ ] Dashboard works

### **Path 2 (CI/CD)**
- [ ] Pushed code to GitHub
- [ ] Watched Actions tab
- [ ] Saw ✅ tests pass

---

**Both paths work together!** 🎉

For detailed guide: `CICD_GUIDE_NO_DOCKERHUB.md`
