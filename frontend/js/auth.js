/**
 * Authentication Manager
 * 
 * Handles JWT token management for API authentication:
 * - Token generation from backend
 * - Token storage in localStorage
 * - Token refresh when expired
 * - Automatic logout on token expiration
 */

class AuthManager {
    constructor() {
        this.token = null;
        this.deviceId = null;
        this.expiresAt = null;
        this.loadFromStorage();
    }

    /**
     * Load saved token from localStorage
     */
    loadFromStorage() {
        try {
            const savedToken = localStorage.getItem('auth_token');
            const savedDeviceId = localStorage.getItem('device_id');
            const savedExpiry = localStorage.getItem('token_expiry');

            if (savedToken && savedDeviceId && savedExpiry) {
                const expiryTime = parseInt(savedExpiry);
                
                // Check if token is still valid
                if (Date.now() < expiryTime) {
                    this.token = savedToken;
                    this.deviceId = savedDeviceId;
                    this.expiresAt = expiryTime;
                    console.log('[Auth] Loaded saved token');
                    return true;
                } else {
                    console.log('[Auth] Saved token expired');
                    this.clearStorage();
                }
            }
        } catch (error) {
            console.error('[Auth] Error loading from storage:', error);
        }
        return false;
    }

    /**
     * Save token to localStorage
     */
    saveToStorage() {
        try {
            localStorage.setItem('auth_token', this.token);
            localStorage.setItem('device_id', this.deviceId);
            localStorage.setItem('token_expiry', this.expiresAt.toString());
            console.log('[Auth] Token saved to storage');
        } catch (error) {
            console.error('[Auth] Error saving to storage:', error);
        }
    }

    /**
     * Clear token from localStorage
     */
    clearStorage() {
        localStorage.removeItem('auth_token');
        localStorage.removeItem('device_id');
        localStorage.removeItem('token_expiry');
        this.token = null;
        this.deviceId = null;
        this.expiresAt = null;
    }

    /**
     * Login - get token from backend
     * @param {string} deviceId - Device identifier
     * @returns {Promise<boolean>} Success status
     */
    async login(deviceId) {
        try {
            console.log(`[Auth] Logging in with device: ${deviceId}`);
            
            const response = await fetch('http://localhost:3000/api/auth/token', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ device_id: deviceId })
            });

            if (!response.ok) {
                const error = await response.text();
                throw new Error(`Authentication failed: ${error}`);
            }

            const data = await response.json();
            
            // Save token and device ID
            this.token = data.token;
            this.deviceId = deviceId;
            // Set expiry time (expires_in is in seconds)
            this.expiresAt = Date.now() + (data.expires_in * 1000);
            
            // Save to localStorage
            this.saveToStorage();
            
            console.log('[Auth] Login successful');
            return true;

        } catch (error) {
            console.error('[Auth] Login failed:', error);
            throw error;
        }
    }

    /**
     * Logout - clear token
     */
    logout() {
        console.log('[Auth] Logging out');
        this.clearStorage();
    }

    /**
     * Check if currently authenticated
     * @returns {boolean} Authentication status
     */
    isAuthenticated() {
        if (!this.token || !this.expiresAt) {
            return false;
        }
        
        // Check if token expired
        if (Date.now() >= this.expiresAt) {
            console.log('[Auth] Token expired');
            this.clearStorage();
            return false;
        }
        
        return true;
    }

    /**
     * Get current token
     * @returns {string|null} JWT token
     */
    getToken() {
        if (!this.isAuthenticated()) {
            return null;
        }
        return this.token;
    }

    /**
     * Get device ID
     * @returns {string|null} Device ID
     */
    getDeviceId() {
        return this.deviceId;
    }

    /**
     * Get authorization header for API calls
     * @returns {object} Headers object with Authorization
     */
    getAuthHeaders() {
        const token = this.getToken();
        if (!token) {
            throw new Error('Not authenticated');
        }
        
        return {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        };
    }

    /**
     * Get time until token expires (in minutes)
     * @returns {number} Minutes until expiration
     */
    getTimeUntilExpiry() {
        if (!this.expiresAt) {
            return 0;
        }
        const remaining = this.expiresAt - Date.now();
        return Math.floor(remaining / (1000 * 60));
    }
}

// Export singleton instance
const authManager = new AuthManager();
