/**
 * Chart Management
 * 
 * Handles Chart.js initialization and updates for real-time sensor data visualization.
 * Creates line charts for temperature, humidity, sound, and motion over time.
 * 
 * Charts display last 100 data points received from WebSocket.
 */

class ChartManager {
    constructor() {
        this.charts = {};
        this.maxDataPoints = 100; // Show last 100 readings
    }

    /**
     * Initialize all charts
     */
    init() {
        // Temperature Chart
        this.charts.temperature = this.createChart('temp-chart', {
            label: 'Temperature (°C)',
            borderColor: 'rgb(239, 68, 68)',
            backgroundColor: 'rgba(239, 68, 68, 0.1)',
            min: 0,
            max: 40,
            optimal: { min: 18, max: 22 }
        });

        // Humidity Chart
        this.charts.humidity = this.createChart('humidity-chart', {
            label: 'Humidity (%)',
            borderColor: 'rgb(59, 130, 246)',
            backgroundColor: 'rgba(59, 130, 246, 0.1)',
            min: 0,
            max: 100,
            optimal: { min: 40, max: 60 }
        });

        // Sound Chart
        this.charts.sound = this.createChart('sound-chart', {
            label: 'Sound Level (dB)',
            borderColor: 'rgb(16, 185, 129)',
            backgroundColor: 'rgba(16, 185, 129, 0.1)',
            min: 0,
            max: 120,
            optimal: { max: 40 }
        });

        // Motion Chart (shows motion events as spikes)
        this.charts.motion = this.createChart('motion-chart', {
            label: 'Motion Detected',
            borderColor: 'rgb(245, 158, 11)',
            backgroundColor: 'rgba(245, 158, 11, 0.1)',
            min: -0.1,
            max: 1.1,
            stepped: true // Show as steps for boolean data
        });

        console.log('[Charts] Initialized successfully');
    }

    /**
     * Create a Chart.js line chart
     */
    createChart(canvasId, config) {
        const canvas = document.getElementById(canvasId);
        if (!canvas) {
            console.error(`[Charts] Canvas ${canvasId} not found`);
            return null;
        }

        const ctx = canvas.getContext('2d');
        
        return new Chart(ctx, {
            type: 'line',
            data: {
                labels: [],
                datasets: [{
                    label: config.label,
                    data: [],
                    borderColor: config.borderColor,
                    backgroundColor: config.backgroundColor,
                    borderWidth: 2,
                    tension: 0.4,
                    pointRadius: 0,
                    pointHoverRadius: 5,
                    stepped: config.stepped || false
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                animation: {
                    duration: 0 // Disable animation for real-time updates
                },
                scales: {
                    x: {
                        display: true,
                        title: {
                            display: true,
                            text: 'Time'
                        },
                        ticks: {
                            maxTicksLimit: 10,
                            autoSkip: true
                        }
                    },
                    y: {
                        display: true,
                        title: {
                            display: true,
                            text: config.label
                        },
                        min: config.min,
                        max: config.max
                    }
                },
                plugins: {
                    legend: {
                        display: true,
                        position: 'top'
                    },
                    tooltip: {
                        enabled: true,
                        mode: 'nearest',
                        intersect: true,
                        callbacks: {
                            label: function(context) {
                                return context.dataset.label + ': ' + context.parsed.y;
                            }
                        }
                    }
                },
                interaction: {
                    mode: 'nearest',
                    intersect: true
                }
            }
        });
    }

    /**
     * Update charts with new data
     * @param {Array} data - Array of sensor readings from WebSocket
     */
    updateCharts(data) {
        if (!data || data.length === 0) return;

        // Process each reading
        data.forEach(reading => {
            // Format timestamp as 24-hour time (HH:MM:SS)
            const timestamp = new Date(reading.timestamp).toLocaleTimeString('en-GB', { 
                hour12: false,
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit'
            });
            
            // Update temperature chart
            this.addDataPoint(this.charts.temperature, timestamp, reading.temp);
            
            // Update humidity chart
            this.addDataPoint(this.charts.humidity, timestamp, reading.hum);
            
            // Update sound chart
            this.addDataPoint(this.charts.sound, timestamp, reading.sound_db);
            
            // Update motion chart (convert boolean to number: true=1, false=0)
            this.addDataPoint(this.charts.motion, timestamp, reading.motion ? 1 : 0);
        });
    }

    /**
     * Add data point to chart and maintain max data points limit
     */
    addDataPoint(chart, label, value) {
        if (!chart) return;

        chart.data.labels.push(label);
        chart.data.datasets[0].data.push(value);

        // Remove oldest data point if exceeding limit
        if (chart.data.labels.length > this.maxDataPoints) {
            chart.data.labels.shift();
            chart.data.datasets[0].data.shift();
        }

        chart.update();
    }

    /**
     * Clear all charts
     */
    clearCharts() {
        Object.values(this.charts).forEach(chart => {
            if (chart) {
                chart.data.labels = [];
                chart.data.datasets[0].data = [];
                chart.update();
            }
        });
        console.log('[Charts] Cleared all data');
    }
}

// Export singleton instance
const chartManager = new ChartManager();
