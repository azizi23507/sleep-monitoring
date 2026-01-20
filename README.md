# Sleep Monitoring System

A non-invasive sleep quality monitoring system using Raspberry Pi sensors, Rust backend, and real-time web dashboard with ML-powered analysis.

## Overview

This system monitors sleep quality through environmental sensors (temperature, humidity, sound, motion) and provides:
- **Real-time monitoring** via Server-Sent Events (SSE) streaming
- **FHIR-compliant** data storage for healthcare interoperability (LOINC 93832-4 for sleep duration)
- **ML-powered sleep analysis** with Random Forest model
- **Visual sleep calendar** with heatmap showing 90-day quality patterns
- **RESTful API** for data access and integration
- **Secure configuration** via environment variables (no hardcoded credentials)

## Architecture

### Three-Branch Data Flow

**Branch 1: Real-time Streaming**
```
Raspberry Pi → Backend → Redis Cache → SSE → Frontend Dashboard
```
- Zero-latency live monitoring via Server-Sent Events
- One-way server-to-client streaming (lighter than WebSocket)
- Automatic browser reconnection
- Last 100 readings cached
- Sub-second update delivery

**Branch 2A: FHIR Compliance**
```
Raspberry Pi → Backend → PostgreSQL → ML Analysis → FHIR API
```
- Healthcare standard compliance with LOINC 93832-4 (Sleep Duration)
- One FHIR observation per sleep analysis (daily)
- Interoperable with EHR systems
- Searchable observations

**Branch 2B: ML Analysis**
```
PostgreSQL → ML Service (Python) → Sleep Quality Scores → FHIR
```
- **Fully operational** Random Forest model (787 KB)
- Daily automated analysis at 8:00 AM (sleep window: 20:00-08:00)
- Sleep duration calculation from motion patterns
- Quality scoring based on PSQI methodology
- Creates FHIR observations after analysis

## Quick Start

### Prerequisites
- Copy `.env.example` to `.env` and configure your credentials
- **Important:** Never commit `.env` to version control (already in .gitignore)

### Option 1: Docker (Recommended)

```bash
# Clone repository
git clone <repository-url>
cd sleep-monitoring-project

# Configure environment variables
cp .env.example .env
# Edit .env with your credentials

# Start all services
docker-compose up -d

# Backend will be available at http://localhost:3000
# Frontend dashboard at http://localhost:3000/
```

### Option 2: Local Development (Automated Setup)

```bash
# 1. Configure environment
cp .env.example .env
# Edit .env with your database credentials

# 2. Run setup script (creates database, starts services)
./setup.sh

# 3. Start backend
cd backend
cargo run

# Backend runs on http://localhost:3000
# Frontend served at http://localhost:3000/
```

### Option 3: Manual Setup

**Prerequisites:**
- Rust (stable)
- PostgreSQL 15+
- Redis 7+
- Python 3.11+ (for ML)

**Start services:**
```bash
# Start PostgreSQL
sudo service postgresql start

# Create database
sudo -u postgres psql -c "CREATE DATABASE sleep_monitor;"

# Start Redis
redis-server --daemonize yes

# Configure and run backend
cp .env.example .env
# Edit .env with your credentials
cd backend
cargo run  # Automatically loads .env and runs migrations

# Backend runs on http://localhost:3000
# Frontend served at http://localhost:3000/
```
# Frontend served at http://localhost:3000/
```

### Option 3: Hardware Setup (Raspberry Pi)

See `hardware/README.md` for complete setup instructions.

**Quick steps:**
1. Upload `hardware/temps_reel.ino` to Arduino
2. Connect sensors (DHT11, PIR, Sound)
3. Update `BACKEND_URL` in `hardware/real_time.py` with your computer's IP
4. Run: `python3 real_time.py`

## Project Structure

```
sleep-monitoring-project/
├── backend/             # Rust backend server
│   ├── src/            # Source code
│   ├── migrations/     # Database migrations
│   ├── tests/          # Unit & integration tests
│   └── README.md       # Detailed backend documentation
├── frontend/           # Web dashboard
│   ├── index.html      # Main dashboard
│   ├── css/            # Styles
│   └── js/             # Client-side logic
├── hardware/           # Raspberry Pi sensor code
│   ├── temps_reel.ino  # Arduino sensor reader
│   ├── real_time.py    # Pi → Backend bridge
│   └── README.md       # Hardware setup guide
├── ml/                 # ML service (fully operational)
│   ├── sleep_score_ml.py       # ML analysis script (319 lines)
│   ├── random_forest_sleep_score.pkl  # Trained model (787 KB)
│   ├── requirements.txt        # Python dependencies
│   └── README.md              # ML documentation
├── docker-compose.yml  # Docker orchestration
├── Dockerfile          # Backend container
└── README.md           # This file
```

## API Endpoints

### Authentication
- `POST /api/auth/token` - Generate JWT token for Pi devices

### Data Ingestion
- `POST /api/sensor-data` - Submit sensor readings (requires JWT)

### Real-time Streaming
- `GET /events` - Server-Sent Events stream for live data

### FHIR API
- `GET /api/fhir/Observation` - Search FHIR observations (Sleep Duration - LOINC 93832-4)
- `GET /api/fhir/Observation/:id` - Get single observation

### Sleep Quality (ML Results)
- `GET /api/sleep-records` - Get all sleep records (supports `?limit=90` for heatmap)
- `GET /api/sleep-records/:date` - Get record for specific date (format: YYYY-MM-DD)
- `GET /api/sleep-quality/latest` - Get latest ML analysis with score and classification

### Health
- `GET /health` - Server health check

## Security

**Current Implementation:**
- JWT authentication for data ingestion
- Input validation (range checks)
- SQL injection prevention (parameterized queries)
- Structured logging with tracing

**Production Recommendations:**
- Change default JWT_SECRET to secure random value
- Restrict CORS to specific frontend domain(s)
- Add rate limiting per device
- Enable HTTPS/WSS only
- Implement audit logging

## Database

### PostgreSQL Tables
- `sensor_readings` - Raw sensor data
- `fhir_observations` - FHIR-formatted observations
- `sleep_records` - ML analysis results
- `ml_processing_log` - ML execution tracking

### Redis Cache
- Key: `sensor:latest`
- Type: List (FIFO)
- Size: 100 readings max
- TTL: 2 hours
- **Note:** Resets on container restart (no persistence configured)

## Testing

```bash
# Run backend tests
cd backend
cargo test

# Test API endpoints
./test_endpoints.sh

# Test FHIR compliance
./test_branch_2a.sh
```

## Monitoring

**Available Metrics:**
- Server uptime
- Database connection status
- Redis connection status
- WebSocket active connections
- Request counts and errors (via logs)

**Health Check:**
```bash
curl http://localhost:3000/health
```

## Configuration

### Environment Variables

**Required variables** (copy from `.env.example` and customize):

```bash
# Database
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your_secure_password
POSTGRES_DB=sleep_monitor
DATABASE_URL=postgres://postgres:your_password@localhost:5432/sleep_monitor

# Redis
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-secure-jwt-secret-minimum-32-characters

# Database Connection (for ML script)
DB_HOST=localhost
DB_PORT=5432

# Logging
RUST_LOG=info
```

**Important:**
- Backend automatically loads `.env` via `dotenvy` crate
- Docker Compose reads `.env` automatically
- Never commit `.env` to version control
- Use GitHub Secrets for CI/CD deployments

## Documentation

- `backend/README.md` - Comprehensive backend documentation
- `backend/QUICK_REFERENCE.md` - API quick reference
- `backend/TESTING_GUIDE.md` - Testing documentation
- `frontend/README.md` - Frontend documentation
- `DOCKER_GUIDE.md` - Docker deployment guide
- `FINAL_REQUIREMENTS_DOCUMENTATION.md` - Complete requirements spec

## Troubleshooting

**Backend won't start:**
- Check PostgreSQL is running: `psql -h localhost -U postgres`
- Check Redis is running: `redis-cli ping`
- Verify DATABASE_URL in .env

**Docker issues:**
- Check containers: `docker-compose ps`
- View logs: `docker-compose logs backend`
- Restart services: `docker-compose restart`

**Migration warning:**
- Warning about modified migration is benign if database tables exist
- To fix: Drop database and restart, or ignore if system works

**WebSocket not connecting:**
- Ensure backend is running
- Check browser console for errors
- Verify URL: `ws://localhost:3000/ws`

## System Features

### Real-Time Monitoring
- Server-Sent Events (SSE) streaming for one-way data flow
- Automatic browser reconnection
- Sub-second data delivery
- Last 100 readings cached in Redis

### Data Analysis
- ML-powered sleep quality scoring (Random Forest model)
- Sleep duration calculation (20:00-08:00 window)
- PSQI-based quality classification
- Automated daily analysis at 8 AM

### Visualization
- **Sleep Quality Heatmap Calendar** - 90-day visual overview
- Real-time sensor charts
- Current value displays
- Historical data tables
- Color-coded quality indicators

### Healthcare Integration
- FHIR R4 compliance
- Official LOINC code 93832-4 (Sleep Duration)
- Interoperable with EHR systems
- Searchable observations API

### Security & Configuration
- JWT authentication for device data ingestion
- Environment-based configuration (.env)
- No hardcoded credentials
- Input validation & sanitization
- PostgreSQL data persistence
- Structured logging with tracing

### Development Features
- Docker deployment ready
- Comprehensive testing (30+ tests)
- Health check endpoints
- dotenv auto-loading for local development
- CI/CD pipeline with GitHub Actions

## License

Educational/University Project

## Contributing

This is a university project. For questions or issues, refer to the documentation or contact the project team.

## Related Resources

- [Rust Axum Framework](https://github.com/tokio-rs/axum)
- [FHIR R4 Specification](https://www.hl7.org/fhir/)
- [LOINC Codes](https://loinc.org/)
- [Redis Documentation](https://redis.io/docs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

---

**Last Updated:** January 16, 2026  
**Version:** 2.0.0 - SSE Implementation, FHIR Sleep Duration, Environment Variables, Heatmap Calendar


