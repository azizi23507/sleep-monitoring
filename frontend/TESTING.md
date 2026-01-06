# Frontend Testing Guide

Comprehensive guide for testing the Sleep Monitoring System frontend.

---

## Prerequisites

Before testing:
1. Backend server running on `localhost:3000`
2. PostgreSQL database with sensor data
3. Modern web browser with developer tools
4. HTTP server for serving frontend files

---

## Setup for Testing

### 1. Start Backend
```bash
cd ~/sleep-backend-updated
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
RUST_LOG=info cargo run
```

Wait for: "Server listening on 0.0.0.0:3000"

### 2. Start Frontend Server
```bash
cd ~/sleep-frontend
python3 -m http.server 8000
```

### 3. Open Browser
Navigate to: `http://localhost:8000`

---

## Test Cases

### Test 1: Page Load and UI

**What to check:**
- Page loads without errors
- Header displays "Sleep Monitoring System"
- Three tabs visible: Live Dashboard, Sleep Analysis, Standards Reference
- Connection status shows "Disconnected" initially
- Footer displays project information

**Expected behavior:**
- Clean, professional layout
- No console errors
- Responsive design on different screen sizes

**How to verify:**
```
1. Open browser
2. Navigate to http://localhost:8000
3. Check for any error messages
4. Open browser console (F12) - should have no red errors
5. Resize browser window - layout should adapt
```

---

### Test 2: WebSocket Connection

**What to check:**
- Connection status changes to "Connected"
- Green dot appears next to status
- Console shows "[WebSocket] Connected to backend"

**Expected behavior:**
- Automatic connection within 2 seconds
- Auto-reconnect if connection drops

**How to verify:**
```
1. Watch connection status indicator (top-right)
2. Should change from "Disconnected" (red) to "Connected" (green)
3. Open console (F12)
4. Look for: "[WebSocket] Connected to backend"
```

**If connection fails:**
```
1. Check backend is running: curl http://localhost:3000/health
2. Check WebSocket URL in js/websocket.js
3. Check browser console for error messages
```

---

### Test 3: Real-Time Data Reception

**What to check:**
- Current values update every second
- Values are not "--" (placeholder)
- Charts start populating with data

**Expected behavior:**
- Temperature shows in °C
- Humidity shows in %
- Sound level shows in dB
- Motion shows "Detected" or "None"

**How to verify:**
```
1. Watch "Current Readings" section
2. Values should update every ~1 second
3. Open console
4. Look for: "[App] Received data: X readings"
5. Numbers should change as new data arrives
```

---

### Test 4: Charts Visualization

**What to check:**
- Four charts display: Temperature, Humidity, Sound, Motion
- Charts populate with data points
- X-axis shows timestamps
- Y-axis shows appropriate ranges

**Expected behavior:**
- Line charts with smooth curves
- Data points added in real-time
- Maximum 100 points per chart
- No missing or broken charts

**How to verify:**
```
1. Scroll to "Real-Time Trends" section
2. All four charts should be visible
3. Watch data points being added
4. Hover over points to see values
5. Check console for: "[Charts] Initialized successfully"
```

**If charts don't appear:**
```
1. Check Chart.js loaded: Open console, type "Chart"
2. Should see Chart object, not undefined
3. Check for JavaScript errors in console
4. Try: window.app.chartManager.init()
```

---

### Test 5: Tab Navigation

**What to check:**
- Clicking tabs switches views
- Active tab highlighted
- Content changes appropriately

**Expected behavior:**
- Dashboard tab shows live data
- Analysis tab shows ML results
- Standards tab shows documentation

**How to verify:**
```
1. Click "Sleep Analysis" tab
2. Content should change to analysis view
3. Click "Standards Reference" tab
4. Should show standards documentation
5. Click "Live Dashboard" tab
6. Should return to live charts
7. Check console for: "[Tabs] Switched to: X"
```

---

### Test 6: Current Values Display

**What to check:**
- Values update in real-time
- Optimal ranges displayed below each value
- Values formatted correctly (1 decimal place)

**Expected behavior:**
- Temperature: XX.X °C (Optimal: 18-22°C)
- Humidity: XX.X % (Optimal: 40-60%)
- Sound: XX.X dB (Quiet: <40dB)
- Motion: Detected/None (Fewer = Better)

**How to verify:**
```
1. Watch current values section
2. Compare with backend data
3. Send test data via curl:

curl -X POST http://localhost:3000/api/sensor-data \
  -H "Content-Type: application/json" \
  -d '{"temp":21.5,"hum":50.0,"motion":false,"sound_db":35.0,"deviceid":"test","timestamp":"2024-12-29T18:00:00Z"}'

4. Frontend should update with these exact values
```

---

### Test 7: ML Analysis Display

**What to check:**
- Latest sleep quality score displays
- Recent sleep records list shows
- Data formatted correctly

**Expected behavior:**
- If data exists: Shows score, classification, date
- If no data: Shows "No data available" message

**How to verify:**
```
1. Click "Sleep Analysis" tab
2. Check "Latest Sleep Quality" section
3. Check "Recent Sleep Records" section
4. Open console
5. Look for: "[API] Latest quality fetched" or "[API] Error"
```

**To test with data:**
```
1. Insert test record:

sudo -u postgres psql -d sleep_monitor -c "INSERT INTO sleep_records (id, device_id, sleep_date, analysis_start, analysis_end, quality_score, classification, avg_temperature, avg_humidity, avg_sound_level, motion_events_count) VALUES (uuid_generate_v4(), 'pi-001', '2024-12-29', '2024-12-29 22:00:00+00', '2024-12-30 06:00:00+00', 80, 'Good', 20.5, 48.0, 32.0, 3);"

2. Refresh analysis tab
3. Should display score: 80, classification: Good
```

---

### Test 8: Standards Reference

**What to check:**
- All standards sections display
- Reference links work
- Information is readable and formatted

**Expected behavior:**
- Temperature standard: 18-22°C
- Humidity standard: 40-60%
- Sound standard: <40dB
- Motion standard: Minimal movement
- External links open correctly

**How to verify:**
```
1. Click "Standards Reference" tab
2. Scroll through all sections
3. Click reference links
4. Should open in new tab
5. Check formatting is clear and professional
```

---

### Test 9: Error Handling

**What to check:**
- Graceful handling of connection errors
- User-friendly error messages
- Auto-reconnect functionality

**Test scenarios:**

**Scenario A: Backend stops**
```
1. Stop backend (Ctrl+C in backend terminal)
2. Frontend should show "Disconnected"
3. Console should show "[WebSocket] Connection closed"
4. Should attempt reconnection every 5 seconds
5. Restart backend
6. Should automatically reconnect
```

**Scenario B: No ML data**
```
1. Clear sleep_records table:
   sudo -u postgres psql -d sleep_monitor -c "DELETE FROM sleep_records;"
2. Click "Sleep Analysis" tab
3. Should show "No data available" message
4. Should NOT show error messages to user
```

---

### Test 10: Performance

**What to check:**
- Smooth animations and updates
- No lag or freezing
- Efficient memory usage

**How to verify:**
```
1. Open browser DevTools (F12)
2. Go to Performance tab
3. Record for 30 seconds
4. Check:
   - Frame rate should be 60 FPS
   - Memory should be stable
   - No memory leaks
5. Check Network tab:
   - WebSocket connection active
   - Data packets arriving regularly
```

---

### Test 11: Responsive Design

**What to check:**
- Layout adapts to different screen sizes
- All content remains accessible
- No horizontal scrolling

**How to verify:**
```
1. Open DevTools (F12)
2. Click device toolbar (phone icon)
3. Test different devices:
   - iPhone 12 (390x844)
   - iPad (768x1024)
   - Desktop (1920x1080)
4. Check:
   - Tabs work on mobile
   - Charts visible on all sizes
   - Text readable
   - No content overflow
```

---

### Test 12: Browser Compatibility

**What to check:**
- Works in multiple browsers
- No browser-specific issues

**Browsers to test:**
- Chrome/Chromium
- Firefox
- Edge
- Safari (if available)

**How to verify:**
```
1. Open in each browser
2. Run basic tests:
   - Page loads
   - WebSocket connects
   - Charts display
   - Tabs work
3. Check console for errors
```

---

## Debug Tools

### Browser Console Commands

```javascript
// Check WebSocket status
window.app.wsManager.ws.readyState
// 0 = CONNECTING, 1 = OPEN, 2 = CLOSING, 3 = CLOSED

// Refresh ML analysis data
window.app.refreshAnalysis()

// Clear all charts
window.app.chartManager.clearCharts()

// Check Chart.js version
Chart.version

// Send test data manually (for testing)
window.app.wsManager.callbacks[0]([
  {temp: 22.5, hum: 45, motion: false, sound_db: 35, deviceid: 'test', timestamp: new Date().toISOString()}
])
```

### Common Issues and Solutions

**Issue: WebSocket won't connect**
```
Solution 1: Check backend is running
curl http://localhost:3000/health

Solution 2: Check URL in websocket.js
Should be: ws://localhost:3000/ws

Solution 3: Check CORS settings in backend
```

**Issue: Charts not updating**
```
Solution 1: Check console for errors
F12 -> Console tab

Solution 2: Verify Chart.js loaded
Type in console: Chart
Should not be undefined

Solution 3: Reinitialize charts
window.app.chartManager.init()
```

**Issue: ML analysis shows no data**
```
Solution 1: Check API endpoint
curl http://localhost:3000/api/sleep-quality/latest

Solution 2: Verify data exists
sudo -u postgres psql -d sleep_monitor -c "SELECT * FROM sleep_records;"

Solution 3: Check API URL in api.js
Should be: http://localhost:3000/api
```

---

## Test Data Generation

### Insert Test Sleep Record
```sql
INSERT INTO sleep_records (
    id, device_id, sleep_date, 
    analysis_start, analysis_end,
    quality_score, classification,
    avg_temperature, avg_humidity, 
    avg_sound_level, motion_events_count
) VALUES (
    uuid_generate_v4(), 'pi-001', CURRENT_DATE - 1,
    (CURRENT_DATE - 1) || ' 22:00:00+00',
    CURRENT_DATE || ' 06:00:00+00',
    75, 'Good',
    21.0, 48.0, 35.0, 5
);
```

### Generate Multiple Records (Last 7 Days)
```sql
INSERT INTO sleep_records 
SELECT 
    uuid_generate_v4(),
    'pi-001',
    CURRENT_DATE - i,
    (CURRENT_DATE - i) || ' 22:00:00+00',
    (CURRENT_DATE - i + 1) || ' 06:00:00+00',
    60 + (RANDOM() * 30)::int,
    CASE WHEN RANDOM() > 0.5 THEN 'Good' ELSE 'Poor' END,
    18 + (RANDOM() * 6),
    35 + (RANDOM() * 30),
    25 + (RANDOM() * 30),
    (RANDOM() * 20)::int
FROM generate_series(0, 6) i;
```

---

## Acceptance Criteria

Frontend passes testing if:

- [ ] Page loads without errors
- [ ] WebSocket connects successfully
- [ ] Real-time data displays correctly
- [ ] All four charts populate with data
- [ ] Tab navigation works smoothly
- [ ] Current values update every second
- [ ] ML analysis displays (if data exists)
- [ ] Standards reference is readable
- [ ] Connection status indicator works
- [ ] Reconnection happens automatically
- [ ] Responsive design works on mobile
- [ ] No JavaScript console errors
- [ ] Performance is smooth (60 FPS)
- [ ] Works in major browsers

---

## Reporting Issues

When reporting issues, include:
1. Browser and version
2. Operating system
3. Steps to reproduce
4. Expected vs actual behavior
5. Console errors (if any)
6. Screenshots (if helpful)

---

## Next Steps After Testing

If all tests pass:
1. Frontend ready for use
2. Connect real Raspberry Pi
3. Run ML script (when implemented)
4. Monitor for 24 hours
5. Verify end-to-end flow

---

**Testing complete! Frontend ready for production use.**
