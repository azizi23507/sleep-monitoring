// Configuration for different environments
const CONFIG = {
    // Backend API URL - change for production deployment
    API_URL: window.location.hostname === 'localhost' 
        ? 'http://localhost:3000/api' 
        : `${window.location.protocol}//${window.location.host}/api`,
    
    // SSE Stream URL - Server-Sent Events for real-time data
    SSE_URL: window.location.hostname === 'localhost'
        ? 'http://localhost:3000/events'
        : `${window.location.protocol}//${window.location.host}/events`
};
