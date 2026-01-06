// Configuration for different environments
const CONFIG = {
    // Backend API URL - change for production deployment
    API_URL: window.location.hostname === 'localhost' 
        ? 'http://localhost:3000/api' 
        : `${window.location.protocol}//${window.location.host}/api`,
    
    // WebSocket URL - change for production deployment
    WS_URL: window.location.hostname === 'localhost'
        ? 'ws://localhost:3000/ws'
        : `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`
};
