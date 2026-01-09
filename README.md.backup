# Sleep Monitoring System

A non-invasive sleep quality monitoring system using Raspberry Pi sensors, Rust backend, and real-time web dashboard.

## Overview

This system monitors sleep quality through environmental sensors (temperature, humidity, sound, motion) and provides:
- **Real-time monitoring** via WebSocket streaming
- **FHIR-compliant** data storage for healthcare interoperability
- **ML-ready infrastructure** for sleep quality analysis
- **RESTful API** for data access and integration

## Architecture

### Three-Branch Data Flow

**Branch 1: Real-time Streaming**
```
Raspberry Pi → Backend → Redis Cache → WebSocket → Frontend Dashboard
```
- Zero-latency live monitoring
- Last 100 readings cached
- 1-second update interval

**Branch 2A: FHIR Compliance**
```
Raspberry Pi → Backend → PostgreSQL → FHIR API
```
- Healthcare standard compliance
- LOINC code mapping
- Searchable observations

**Branch 2B: ML Analysis**
```
PostgreSQL → ML Service (External) → Sleep Quality Scores
```
- Backend infrastructure ready (database tables, API endpoints)
- ML service integration pending external delivery

## Quick Start

### Option 1: Docker (Recommended)

```bash
# Start all services
docker-compose up -d

# Backend will be available at http://localhost:3000
# Frontend dashboard at http://localhost:3000/
```

### Option 2: Local Development

**Prerequisites:**
- Rust (nightly)
- PostgreSQL 15+
- Redis 7+
- Node.js (for frontend development)

**Start services:**
```bash
# Terminal 1: Start PostgreSQL and Redis
# (Installation varies by OS - see backend/README.md)

# Terminal 2: Start backend
cd backend
cp .env.example .env
# Edit .env with your database credentials
cargo run

# Backend runs on http://localhost:3000
# Frontend served at http://localhost:3000/
```

## Project Structure

```
sleep-monitoring-project/
├── backend/ # Rust backend server
│ ├── src/ # Source code
│ ├── migrations/ # Database migrations
│ ├── tests/ # Unit & integration tests
│ └── README.md # Detailed backend documentation
├── frontend/ # Web dashboard
│ ├── index.html # Main dashboard
│ ├── css/ # Styles
│ └── js/ # Client-side logic
├── docker-compose.yml # Docker orchestration
├── Dockerfile # Backend container
└── README.md # This file
```

## API Endpoints

### Authentication
- `POST /api/auth/token` - Generate JWT token for Pi devices

### Data Ingestion
- `POST /api/sensor-data` - Submit sensor readings (requires JWT)

### Real-time Streaming
- `WS /ws` - WebSocket for live data stream

### FHIR API
- `GET /api/fhir/Observation` - Search FHIR observations
- `GET /api/fhir/Observation/:id` - Get single observation

### Sleep Quality (ML Results)
- `GET /api/sleep-records` - Get all sleep records
- `GET /api/sleep-records/:date` - Get record for specific date
- `GET /api/sleep-quality/latest` - Get latest analysis

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

See `backend/.env.example` for all options:

```bash
DATABASE_URL=postgres://user:pass@localhost/sleep_monitor
JWT_SECRET=your-secure-secret-key-here
REDIS_URL=redis://127.0.0.1:6379
RUST_LOG=info
```

### Docker Environment

Set in `docker-compose.yml` or create `.env` file:

```bash
JWT_SECRET=your-production-secret
POSTGRES_PASSWORD=secure-password
```

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

- Real-time WebSocket streaming
- JWT authentication for Pi devices
- PostgreSQL data persistence
- Redis caching (last 100 readings)
- FHIR R4 compliance
- Input validation & error handling
- Docker deployment
- Comprehensive testing (30+ tests)
- Structured logging
- Health check endpoints
- REST API with authentication
- ML-ready infrastructure (database tables and API endpoints)

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

**Last Updated:** January 7, 2026 
**Version:** 1.0.0


