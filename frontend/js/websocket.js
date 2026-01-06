/**
 * WebSocket Connection Handler
 * 
 * Manages real-time connection to backend WebSocket server.
 * Receives sensor data updates and notifies callback functions.
 * 
 * Backend sends data as JSON array of sensor readings:
 * [{temp: 22.5, hum: 45.0, motion: false, sound_db: 35.2, deviceid: "pi-001", timestamp: "..."}]
 */

class WebSocketManager {
    constructor() {
        this.ws = null;
        this.reconnectInterval = 5000; // 5 seconds
        this.reconnectTimer = null;
        this.callbacks = [];
        this.statusCallback = null;
    }

    /**
     * Connect to WebSocket server
     * @param {string} url - WebSocket URL (default: ws://localhost:3000/ws)
     */
    connect(url = 'ws://localhost:3000/ws') {
        try {
            this.ws = new WebSocket(url);
            
            this.ws.onopen = () => {
                console.log('[WebSocket] Connected to backend');
                this.updateStatus(true);
                
                // Clear reconnect timer if exists
                if (this.reconnectTimer) {
                    clearTimeout(this.reconnectTimer);
                    this.reconnectTimer = null;
                }
            };

            this.ws.onmessage = (event) => {
                try {
                    // Parse incoming JSON data
                    const data = JSON.parse(event.data);
                    
                    // Notify all registered callbacks
                    this.callbacks.forEach(callback => {
                        callback(data);
                    });
                } catch (error) {
                    console.error('[WebSocket] Error parsing data:', error);
                }
            };

            this.ws.onclose = () => {
                console.log('[WebSocket] Connection closed');
                this.updateStatus(false);
                
                // Attempt to reconnect
                this.scheduleReconnect(url);
            };

            this.ws.onerror = (error) => {
                console.error('[WebSocket] Error:', error);
                this.updateStatus(false);
            };

        } catch (error) {
            console.error('[WebSocket] Failed to connect:', error);
            this.updateStatus(false);
            this.scheduleReconnect(url);
        }
    }

    /**
     * Schedule reconnection attempt
     */
    scheduleReconnect(url) {
        if (this.reconnectTimer) return; // Already scheduled
        
        console.log(`[WebSocket] Reconnecting in ${this.reconnectInterval / 1000} seconds...`);
        this.reconnectTimer = setTimeout(() => {
            this.reconnectTimer = null;
            this.connect(url);
        }, this.reconnectInterval);
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
     * Close WebSocket connection
     */
    close() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
        
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
    }
}

// Export singleton instance
const wsManager = new WebSocketManager();
