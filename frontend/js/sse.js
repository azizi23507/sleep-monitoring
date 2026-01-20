/**
 * Server-Sent Events (SSE) Connection Handler
 * 
 * Manages real-time one-way connection from backend to frontend.
 * Receives sensor data updates via SSE and notifies callback functions.
 * 
 * Benefits over WebSocket:
 * - Simpler protocol (one-way only)
 * - Automatic reconnection built into browser
 * - Less overhead
 * - Better for server → client streaming
 * 
 * Backend sends data as JSON array of sensor readings:
 * [{temp: 22.5, hum: 45.0, motion: false, sound_db: 35.2, deviceid: "pi-001", timestamp: "..."}]
 */

class SSEManager {
    constructor() {
        this.eventSource = null;
        this.callbacks = [];
        this.statusCallback = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 10;
    }

    /**
     * Connect to SSE stream
     * @param {string} url - SSE URL (uses CONFIG.SSE_URL by default)
     */
    connect(url = CONFIG.SSE_URL) {
        try {
            // Close existing connection if any
            if (this.eventSource) {
                this.eventSource.close();
            }

            this.eventSource = new EventSource(url);
            
            this.eventSource.onopen = () => {
                console.log('[SSE] Connected to backend');
                this.updateStatus(true);
                this.reconnectAttempts = 0; // Reset counter on successful connection
            };

            this.eventSource.onmessage = (event) => {
                try {
                    // Parse incoming JSON data
                    const data = JSON.parse(event.data);
                    
                    // Notify all registered callbacks
                    this.callbacks.forEach(callback => {
                        callback(data);
                    });
                } catch (error) {
                    console.error('[SSE] Error parsing data:', error);
                }
            };

            this.eventSource.onerror = (error) => {
                console.error('[SSE] Connection error');
                this.updateStatus(false);
                
                // EventSource automatically attempts to reconnect
                // but we track attempts to show user if persistent failure
                this.reconnectAttempts++;
                
                if (this.reconnectAttempts >= this.maxReconnectAttempts) {
                    console.error('[SSE] Max reconnection attempts reached');
                    this.eventSource.close();
                }
            };

        } catch (error) {
            console.error('[SSE] Failed to connect:', error);
            this.updateStatus(false);
        }
    }

    /**
     * Register callback for data updates
     * @param {function} callback - Function to call with new data
     */
    onData(callback) {
        this.callbacks.push(callback);
    }

    /**
     * Register callback for status updates
     * @param {function} callback - Function to call with connection status (true/false)
     */
    onStatus(callback) {
        this.statusCallback = callback;
    }

    /**
     * Update connection status and notify callback
     */
    updateStatus(connected) {
        if (this.statusCallback) {
            this.statusCallback(connected);
        }
    }

    /**
     * Close SSE connection
     */
    close() {
        if (this.eventSource) {
            this.eventSource.close();
            this.eventSource = null;
        }
        this.reconnectAttempts = 0;
    }
}

// Export singleton instance
const sseManager = new SSEManager();
