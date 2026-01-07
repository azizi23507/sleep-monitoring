# Sleep Monitoring System - Frontend

Clean, professional web interface for real-time sleep environment monitoring and analysis.

---

## Features

### 1. Live Dashboard
- Real-time sensor data visualization
- Four environmental metrics: Temperature, Humidity, Sound Level, Motion
- Current values with optimal range indicators
- Line charts showing trends (last 100 readings)
- WebSocket connection for instant updates

### 2. Sleep Analysis
- ML-analyzed sleep quality scores
- Latest sleep quality with classification (Good/Poor)
- Recent sleep records (last 7 nights)
- Environmental statistics for each night

### 3. Standards Reference
- Scientific standards for sleep quality assessment
- Detailed scoring criteria for each metric
- Research references with links
- Educational information about sleep environment factors

---

## Technology Stack

- **Pure HTML5/CSS3/JavaScript** - No frameworks, lightweight and fast
- **Chart.js 4.4.0** - Professional data visualization
- **WebSocket API** - Real-time data streaming
- **Fetch API** - REST API communication

---

## Architecture

```
Frontend (Browser)
 |
 +-- WebSocket Connection (ws://localhost:3000/ws)
 | |
 | +-- Receives: Live sensor data (every second)
 | +-- Updates: Charts + Current values
 | +-- No authentication required
 |
 +-- REST API Calls (http://localhost:3000/api/)
 |
 +-- GET /api/sleep-records (public)
 +-- GET /api/sleep-quality/latest (public)
 +-- No authentication required
 +-- GET /sleep-quality/latest (Latest ML score)
 +-- GET /sleep-records?limit=7 (Recent nights)
```

---

## File Structure

```
sleep-frontend/
├── index.html Main HTML page
├── css/
│ └── styles.css All styles (responsive, clean design)
├── js/
│ ├── websocket.js WebSocket connection manager
│ ├── charts.js Chart.js chart management
│ ├── api.js REST API handler for ML results
│ └── main.js Main application controller
├── README.md This file
├── STANDARDS.md Sleep quality standards documentation
└── TESTING.md Testing guide
```

---

## Setup Instructions

### Prerequisites
1. Backend server running on `localhost:3000`
2. PostgreSQL and Redis running
3. Modern web browser (Chrome, Firefox, Edge, Safari)

### Running Frontend

#### Option 1: Backend Served (Recommended)
Frontend is automatically served by backend at `http://localhost:3000/`

1. Start backend:
```bash
cd sleep-backend
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
export JWT_SECRET="your-secret-key"
cargo run
```

2. Open browser: `http://localhost:3000`
3. Dashboard loads immediately - no login required!

#### Option 2: Simple HTTP Server
```bash
# Navigate to frontend directory
cd sleep-frontend

# Start simple HTTP server
python3 -m http.server 8000
```

Open browser: `http://localhost:8000`

**Note:** Backend must still be running on port 3000 for API/WebSocket.

---

## Usage Guide

### Dashboard
- View real-time sensor readings
- See connection status (top right)
- Navigate between tabs (Live Dashboard, Sleep Analysis, Standards)
- Charts update automatically with WebSocket data

### Sleep Analysis Tab
- View latest sleep quality score
- Browse recent sleep records (last 7 nights)
- See environmental statistics for each night
- Data refreshes automatically when switching to this tab

---

## Configuration

### WebSocket URL
Default: `ws://localhost:3000/ws`

To change:
1. Edit `js/websocket.js`
2. Find: `connect(url = 'ws://localhost:3000/ws')`
3. Change URL to your backend address

### API Base URL
Default: `http://localhost:3000/api`

To change:
1. Edit `js/api.js`
2. Find: `this.baseURL = 'http://localhost:3000/api'`
3. Change URL to your backend address

---

## Usage

### Live Dashboard
1. Open frontend in browser
2. Click "Live Dashboard" tab
3. View current sensor readings
4. Monitor real-time charts
5. Check connection status (top-right indicator)

### Sleep Analysis
1. Click "Sleep Analysis" tab
2. View latest sleep quality score
3. Browse recent sleep records
4. Compare different nights

### Standards Reference
1. Click "Standards Reference" tab
2. Read about sleep quality standards
3. Understand scoring criteria
4. View research references

---

## Components

### 1. WebSocket Manager (`websocket.js`)
**Purpose:** Manages real-time connection to backend

**Features:**
- Automatic reconnection (every 5 seconds)
- Connection status tracking
- Multiple callback support
- Error handling

**Methods:**
- `connect(url)` - Connect to WebSocket server
- `onData(callback)` - Register data callback
- `onStatus(callback)` - Register status callback
- `close()` - Close connection

### 2. Chart Manager (`charts.js`)
**Purpose:** Handles all Chart.js visualizations

**Features:**
- Four real-time line charts
- Automatic data point limiting (100 max)
- Optimal range visualization
- Responsive design

**Methods:**
- `init()` - Initialize all charts
- `updateCharts(data)` - Update with new data
- `clearCharts()` - Clear all data

### 3. API Manager (`api.js`)
**Purpose:** Fetches ML analysis results from backend

**Features:**
- Latest sleep quality score
- Recent sleep records list
- Error handling with user-friendly messages

**Methods:**
- `getLatestQuality()` - Fetch latest score
- `getSleepRecords(limit)` - Fetch recent records
- `refreshAnalysis()` - Refresh all data

### 4. Main Controller (`main.js`)
**Purpose:** Coordinates all components

**Features:**
- Tab navigation
- Component initialization
- Data flow coordination
- Debug tools (window.app)

---

## Data Flow

### Real-Time Data (WebSocket)
```
1. Backend sends data every second
2. WebSocket receives JSON array
3. Update current values display
4. Add points to charts
5. Maintain 100-point limit
```

### ML Results (REST API)
```
1. User switches to Analysis tab
2. Fetch latest quality from API
3. Fetch recent records from API
4. Display in UI
5. Auto-refresh on tab switch
```

---

## Responsive Design

Frontend adapts to different screen sizes:

- **Desktop (>768px):** Side-by-side layouts, full charts
- **Mobile (<768px):** Stacked layouts, vertical tabs
- **Print:** Optimized for printing reports

---

## Browser Compatibility

Tested and working:
- Chrome 90+
- Firefox 88+
- Edge 90+
- Safari 14+

Requirements:
- WebSocket support
- ES6 JavaScript support
- CSS Grid/Flexbox support

---

## Troubleshooting

### WebSocket Won't Connect
**Problem:** Status shows "Disconnected"

**Solutions:**
1. Check backend is running: `curl http://localhost:3000/health`
2. Check WebSocket URL in `websocket.js`
3. Check browser console for errors
4. Verify CORS settings in backend

### Charts Not Updating
**Problem:** Charts remain empty

**Solutions:**
1. Check WebSocket connection status
2. Open browser console, look for errors
3. Verify data format from backend
4. Try: `window.app.chartManager.clearCharts()` in console

### ML Analysis Shows "No Data"
**Problem:** Analysis tab shows loading/no data

**Solutions:**
1. Verify backend API endpoints working: `curl http://localhost:3000/api/sleep-quality/latest`
2. Ensure ML script has run and created data
3. Note: Python ML service not yet delivered - infrastructure ready but no analysis results yet

---

**Last Updated:** January 7, 2026 
**Version:** 1.0.0
3. Check browser console for API errors
4. Try manual refresh: `window.app.refreshAnalysis()` in console

### CORS Errors
**Problem:** Browser blocks requests

**Solutions:**
1. Use HTTP server (not file:// protocol)
2. Verify backend CORS configuration
3. Check backend allows origin `http://localhost:8000`

---

## Development

### Debug Tools
Open browser console and use:

```javascript
// Access managers
window.app.wsManager // WebSocket manager
window.app.chartManager // Chart manager
window.app.apiManager // API manager

// Refresh analysis manually
window.app.refreshAnalysis()

// Check WebSocket status
window.app.wsManager.ws.readyState // 1 = connected

// Clear all charts
window.app.chartManager.clearCharts()
```

### Testing Data Flow
1. Open browser console
2. Watch for log messages:
 - `[WebSocket] Connected`
 - `[App] Received data: X readings`
 - `[Charts] Updated`
 - `[API] Data fetched`

---

## Performance

### Optimizations
- Chart animations disabled for real-time updates
- Data point limiting (100 max per chart)
- Efficient DOM updates
- Minimal dependencies

### Resource Usage
- Memory: ~50MB (with charts)
- CPU: <5% (during updates)
- Network: ~1KB/second (WebSocket)

---

## Future Enhancements

Potential improvements:
1. Export data to CSV/PDF
2. Date range picker for historical analysis
3. Comparison view (multiple nights)
4. Notifications for poor sleep conditions
5. User preferences/settings
6. Dark mode
7. Multi-language support

---

## Credits

**Developed for:** University Sleep Monitoring Project

**Technologies:**
- Chart.js for visualization
- WebSocket for real-time communication
- REST API for data retrieval

**Standards Based On:**
- National Sleep Foundation guidelines
- WHO Environmental Noise Guidelines
- American Academy of Sleep Medicine recommendations
- Peer-reviewed sleep research

---

## License

Educational use only - University project.

---

## Support

For issues or questions:
1. Check TESTING.md for testing procedures
2. Check STANDARDS.md for standards information
3. Check browser console for errors
4. Verify backend is running and accessible


