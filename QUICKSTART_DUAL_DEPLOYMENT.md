# Quick Start - Dual Deployment System

**Choose your approach based on your needs:**

---

## 🎯 Choose Your Path

### **Path 1: Local Development (2 Minutes)**
For: Demos, testing, local development  
**No setup needed!**

### **Path 2: Production CI/CD (15 Minutes)**
For: Automated deployment, grading, production  
**Requires GitHub Secrets setup**

---

## 🚀 Path 1: Local Development

### **What You Get**
- ✅ Instant demo capability
- ✅ No configuration required
- ✅ Perfect for professor/students
- ✅ Works immediately

### **Steps**

#### **1. Clone Repository**
```bash
git clone https://github.com/yourusername/sleep-monitoring.git
cd sleep-monitoring
```

#### **2. Start Everything**
```bash
docker-compose up
```

**That's it!** ✨

#### **3. Access Dashboard**
- Open browser: `http://localhost:3000`
- Health check: `http://localhost:3000/health`

#### **4. Test API**
```bash
# Get auth token
curl -X POST http://localhost:3000/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id":"demo-pi"}'

# Copy the token, then send data
curl -X POST http://localhost:3000/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{
    "temp": 22.5,
    "hum": 45.0,
    "motion": false,
    "sound_db": 35.0,
    "deviceid": "demo-pi",
    "timestamp": "2024-12-30T10:00:00Z"
  }'
```

### **What's Running**
- PostgreSQL (port 5432) - Database
- Redis (port 6379) - Cache
- Backend (port 3000) - API + Frontend

### **Credentials Used (Hardcoded)**
```
Database: postgres / password
JWT Secret: dev-secret-key-change-in-production
Redis: No password
```

---

## 🔄 Path 2: Production CI/CD

### **What You Get**
- ✅ Automated testing on every push
- ✅ Automatic Docker image building
- ✅ Secure credential management
- ✅ Professional DevOps workflow

### **One-Time Setup (15 Minutes)**

#### **Step 1: Create Docker Hub Account**
```
1. Go to https://hub.docker.com
2. Sign up (free account)
3. Create access token:
   - Account Settings → Security
   - New Access Token
   - Name: "GitHub Actions"
   - Copy token
```

#### **Step 2: Add GitHub Secrets**
```
1. Go to your GitHub repository
2. Click Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Add these 4 secrets:
```

**Secret 1: DOCKER_USERNAME**
```
Name: DOCKER_USERNAME
Value: your_dockerhub_username
```

**Secret 2: DOCKER_PASSWORD**
```
Name: DOCKER_PASSWORD
Value: your_dockerhub_token_from_step1
```

**Secret 3: POSTGRES_PASSWORD**
```
Name: POSTGRES_PASSWORD
Value: [Generate secure password]

# Generate with:
openssl rand -base64 32
```

**Secret 4: JWT_SECRET**
```
Name: JWT_SECRET
Value: [Generate secure secret]

# Generate with:
openssl rand -base64 32
```

#### **Step 3: Push Code to Trigger CI/CD**
```bash
# Make any small change
echo "# CI/CD Test" >> README.md

# Commit and push
git add README.md
git commit -m "Test CI/CD pipeline"
git push origin main
```

#### **Step 4: Watch CI/CD Run**
```
1. Go to GitHub → Actions tab
2. See workflow running
3. Wait 5-10 minutes
4. See ✅ All checks passed
```

#### **Step 5: Verify Docker Hub**
```
1. Go to https://hub.docker.com
2. Check Repositories
3. See: yourusername/sleep-backend:latest
4. Image ready to pull!
```

---

## 📊 Comparison

| Feature | Path 1: Local | Path 2: CI/CD |
|---------|--------------|--------------|
| **Time to Start** | 2 minutes | 15 minutes setup |
| **Credentials** | Hardcoded | Encrypted secrets |
| **Testing** | Manual | Automatic |
| **Building** | Manual | Automatic |
| **Best For** | Demos | Production |
| **Security** | Low | High |
| **Setup** | None | One-time |

---

## 🎓 For Submission

### **To Professor**

**Include Both:**
1. ✅ GitHub repository URL
2. ✅ Quick demo instructions (Path 1)

**Professor can:**
- Clone and run in 2 minutes (Path 1)
- Review CI/CD in Actions tab (Path 2)
- Grade both approaches

---

## 🔄 Daily Workflow (After Setup)

### **Local Development**
```bash
# Start working
docker-compose up -d

# Make changes
# ... edit code ...

# Test changes
docker-compose restart backend

# Stop when done
docker-compose down
```

### **With CI/CD**
```bash
# Make changes
# ... edit code ...

# Test locally first
cargo test

# Push to trigger CI/CD
git add .
git commit -m "Added feature X"
git push

# CI/CD automatically:
# ✅ Runs tests
# ✅ Builds image
# ✅ Pushes to Docker Hub
```

---

## 📝 Commands Cheat Sheet

### **Docker Compose**
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f backend

# Restart backend only
docker-compose restart backend

# Stop all services
docker-compose down

# Remove all data
docker-compose down -v

# Rebuild after code changes
docker-compose up --build
```

### **Testing**
```bash
# Run backend tests
cd backend && cargo test

# Test specific function
cargo test test_sensor_validation

# Check health endpoint
curl http://localhost:3000/health
```

### **Git & CI/CD**
```bash
# Check CI/CD status
git push
# Then: GitHub → Actions tab

# View workflow runs
# GitHub → Actions → Click any run

# Re-run failed workflow
# Actions → Failed run → Re-run jobs
```

---

## 🆘 Common Issues

### **Path 1 (Local) Issues**

**Problem: Port 3000 already in use**
```bash
# Find and kill process
lsof -i :3000        # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Or use different port
# Edit docker-compose.yml: "3001:3000"
```

**Problem: Backend exits immediately**
```bash
# Check logs
docker-compose logs backend

# Restart all
docker-compose down
docker-compose up
```

---

### **Path 2 (CI/CD) Issues**

**Problem: Workflow fails**
```
1. GitHub → Actions → Click failed workflow
2. Expand failed step
3. Read error message
4. Common fixes:
   - Check secrets are added correctly
   - Verify Docker Hub credentials
   - Run tests locally first
```

**Problem: Docker Hub push unauthorized**
```
1. Verify DOCKER_USERNAME is correct
2. Verify DOCKER_PASSWORD is access token (not password)
3. Check token hasn't expired
4. Create new token if needed
```

---

## 📚 Next Steps

### **After Quick Start**

1. **Read Full Guide**
   - See: `COMPLETE_CICD_DEPLOYMENT_GUIDE.md`
   - Comprehensive 26KB guide

2. **Review Implementation**
   - See: `IMPLEMENTATION_STATUS.md`
   - What's been implemented

3. **Check Integration Guides**
   - `PI_CONNECTIVITY_GUIDE.md` - For Pi team
   - `ML_CONNECTIVITY_GUIDE.md` - For ML team

4. **Review Security**
   - See: `SECURITY.md`
   - Best practices

---

## ✅ Success Checklist

### **Path 1 (Local)**
- [ ] Cloned repository
- [ ] Ran `docker-compose up`
- [ ] Accessed http://localhost:3000
- [ ] Tested health endpoint
- [ ] Sent test sensor data

### **Path 2 (CI/CD)**
- [ ] Created Docker Hub account
- [ ] Generated access token
- [ ] Added all 4 GitHub Secrets
- [ ] Pushed code
- [ ] Watched workflow run
- [ ] Verified Docker Hub image

---

## 🎉 You're Ready!

**Path 1:** Demo-ready in 2 minutes  
**Path 2:** Production-ready with CI/CD

**Both paths work together - choose based on your needs!**

For detailed instructions: `COMPLETE_CICD_DEPLOYMENT_GUIDE.md`

**Good luck!** 🚀
