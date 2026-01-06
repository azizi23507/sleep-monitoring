/**
 * Main Application Controller
 * 
 * Coordinates all frontend functionality:
 * - Tab navigation
 * - WebSocket connection and data handling
 * - Chart updates
 * - Current value displays
 * - ML analysis data loading
 */

// Wait for DOM to be fully loaded
document.addEventListener('DOMContentLoaded', () => {
    console.log('[App] Initializing Sleep Monitoring System');

    // ========================================
    // Tab Navigation
    // ========================================
    initializeTabs();

    // ========================================
    // WebSocket Connection
    // ========================================
    initializeWebSocket();

    // ========================================
    // Charts
    // ========================================
    chartManager.init();

    // ========================================
    // ML Analysis Data
    // ========================================
    loadAnalysisData();

    console.log('[App] Initialization complete');
});

/**
 * Initialize tab navigation
 */
function initializeTabs() {
    const tabButtons = document.querySelectorAll('.tab-button');
    const tabContents = document.querySelectorAll('.tab-content');

    tabButtons.forEach(button => {
        button.addEventListener('click', () => {
            const tabName = button.getAttribute('data-tab');

            // Remove active class from all buttons and contents
            tabButtons.forEach(btn => btn.classList.remove('active'));
            tabContents.forEach(content => content.classList.remove('active'));

            // Add active class to clicked button and corresponding content
            button.classList.add('active');
            document.getElementById(`${tabName}-tab`).classList.add('active');

            console.log('[Tabs] Switched to:', tabName);

            // Refresh analysis data when switching to analysis tab
            if (tabName === 'analysis') {
                apiManager.refreshAnalysis();
            }
        });
    });

    console.log('[Tabs] Navigation initialized');
}

/**
 * Initialize WebSocket connection
 */
function initializeWebSocket() {
    const statusIndicator = document.getElementById('status-indicator');
    const statusText = document.getElementById('status-text');
    
    // Show connecting state initially
    statusIndicator.className = 'status-dot connecting';
    statusText.textContent = 'Connecting...';
    
    // Handle connection status updates
    wsManager.onStatus((connected) => {

        if (connected) {
            statusIndicator.className = 'status-dot connected';
            statusText.textContent = 'Connected';
            console.log('[App] WebSocket connected');
        } else {
            statusIndicator.className = 'status-dot disconnected';
            statusText.textContent = 'Disconnected';
            console.log('[App] WebSocket disconnected');
        }
    });

    // Handle incoming data
    wsManager.onData((data) => {
        try {
            console.log('[App] Received data:', data.length, 'readings');

            // Update current values display
            updateCurrentValues(data);

            // Update charts
            chartManager.updateCharts(data);
        } catch (error) {
            console.error('[App] Error processing data:', error);
        }
    });

    // Connect to backend
    wsManager.connect();
    console.log('[App] WebSocket connection initiated');
}

/**
 * Update current values display
 * Shows most recent reading from data array
 */
function updateCurrentValues(data) {
    if (!data || data.length === 0) return;

    // Get most recent reading (last item in array)
    const latest = data[data.length - 1];
    
    // Validate data structure
    if (!latest || typeof latest.temp === 'undefined') {
        console.warn('[App] Invalid data format received');
        return;
    }

    // Update temperature
    const tempElement = document.getElementById('current-temp');
    if (tempElement) {
        tempElement.textContent = latest.temp.toFixed(1);
        tempElement.className = 'value ' + getTemperatureClass(latest.temp);
    }

    // Update humidity
    const humidityElement = document.getElementById('current-humidity');
    if (humidityElement) {
        humidityElement.textContent = latest.hum.toFixed(1);
        humidityElement.className = 'value ' + getHumidityClass(latest.hum);
    }

    // Update sound level
    const soundElement = document.getElementById('current-sound');
    if (soundElement) {
        soundElement.textContent = latest.sound_db.toFixed(1);
        soundElement.className = 'value ' + getSoundClass(latest.sound_db);
    }

    // Update motion status
    const motionElement = document.getElementById('current-motion');
    if (motionElement) {
        motionElement.textContent = latest.motion ? 'Detected' : 'None';
        motionElement.className = 'value ' + (latest.motion ? 'motion-active' : 'motion-inactive');
    }
}

/**
 * Get CSS class for temperature value based on optimal range
 * Optimal: 18-22°C
 */
function getTemperatureClass(temp) {
    if (temp >= 18 && temp <= 22) return 'optimal';
    if (temp >= 16 && temp <= 24) return 'acceptable';
    return 'poor';
}

/**
 * Get CSS class for humidity value based on optimal range
 * Optimal: 40-60%
 */
function getHumidityClass(humidity) {
    if (humidity >= 40 && humidity <= 60) return 'optimal';
    if (humidity >= 30 && humidity <= 70) return 'acceptable';
    return 'poor';
}

/**
 * Get CSS class for sound level based on optimal range
 * Optimal: <40dB
 */
function getSoundClass(soundLevel) {
    if (soundLevel < 40) return 'optimal';
    if (soundLevel < 55) return 'acceptable';
    return 'poor';
}

/**
 * Load ML analysis data
 * Called on page load and when switching to analysis tab
 */
async function loadAnalysisData() {
    try {
        await apiManager.refreshAnalysis();
        console.log('[App] ML analysis data loaded');
    } catch (error) {
        console.error('[App] Error loading analysis data:', error);
    }
}

/**
 * Cleanup on page unload
 */
window.addEventListener('beforeunload', () => {
    wsManager.close();
    console.log('[App] Cleanup complete');
});

// Export for debugging in console
window.app = {
    wsManager,
    chartManager,
    apiManager,
    refreshAnalysis: loadAnalysisData
};

console.log('[App] Debug tools available via window.app');
