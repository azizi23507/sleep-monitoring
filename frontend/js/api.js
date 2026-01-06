/**
 * API Handler for ML Results
 * 
 * Fetches sleep quality analysis results from backend REST API endpoints.
 * Handles:
 * - Latest sleep quality score
 * - Recent sleep records (last 7 nights)
 * - Individual sleep record by date
 */

class APIManager {
    constructor() {
        this.baseURL = 'http://localhost:3000/api';
    }

    /**
     * Fetch latest sleep quality score
     * @returns {Promise<Object>} Latest sleep quality data
     */
    async getLatestQuality() {
        try {
            const response = await fetch(`${this.baseURL}/sleep-quality/latest`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const data = await response.json();
            console.log('[API] Latest quality fetched:', data);
            return data;
        } catch (error) {
            console.error('[API] Error fetching latest quality:', error);
            throw error;
        }
    }

    /**
     * Fetch all sleep records
     * @param {number} limit - Maximum number of records to fetch (default: 7)
     * @returns {Promise<Object>} Sleep records data
     */
    async getSleepRecords(limit = 7) {
        try {
            const response = await fetch(`${this.baseURL}/sleep-records?limit=${limit}`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const data = await response.json();
            console.log('[API] Sleep records fetched:', data);
            return data;
        } catch (error) {
            console.error('[API] Error fetching sleep records:', error);
            throw error;
        }
    }

    /**
     * Fetch sleep record for specific date
     * @param {string} date - Date in YYYY-MM-DD format
     * @returns {Promise<Object>} Sleep record data
     */
    async getSleepRecordByDate(date) {
        try {
            const response = await fetch(`${this.baseURL}/sleep-records/${date}`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const data = await response.json();
            console.log('[API] Sleep record for', date, 'fetched:', data);
            return data;
        } catch (error) {
            console.error('[API] Error fetching sleep record for', date, ':', error);
            throw error;
        }
    }

    /**
     * Display latest quality in UI
     */
    async displayLatestQuality() {
        const container = document.getElementById('latest-result');
        
        try {
            const data = await this.getLatestQuality();
            
            container.innerHTML = `
                <p class="score">${data.score}</p>
                <p class="classification">${data.classification} Sleep Quality</p>
                <p class="date">Night of ${data.date}</p>
            `;
        } catch (error) {
            container.innerHTML = `
                <p class="loading">No sleep analysis data available yet.</p>
                <p class="loading" style="font-size: 0.9rem; margin-top: 10px;">
                    ML analysis will be available after the first night of data collection.
                </p>
            `;
        }
    }

    /**
     * Display recent sleep records in UI
     */
    async displayRecentRecords() {
        const container = document.getElementById('records-list');
        
        try {
            const data = await this.getSleepRecords(7);
            
            if (data.total === 0) {
                container.innerHTML = `
                    <p class="loading">No sleep records available yet.</p>
                    <p class="loading" style="font-size: 0.9rem; margin-top: 10px;">
                        Records will appear here after ML analysis runs.
                    </p>
                `;
                return;
            }
            
            // Create HTML for each record
            const recordsHTML = data.records.map(record => {
                const classificationClass = record.classification.toLowerCase();
                
                return `
                    <div class="record-item">
                        <p class="record-date">${record.sleep_date}</p>
                        <p class="record-score">${record.quality_score}</p>
                        <p class="record-classification ${classificationClass}">${record.classification}</p>
                        ${record.avg_temperature ? `<p style="font-size: 0.85rem; color: #333; margin-top: 10px;">
                            Avg Temp: ${record.avg_temperature.toFixed(1)}°C<br>
                            Avg Humidity: ${record.avg_humidity.toFixed(1)}%<br>
                            Avg Sound: ${record.avg_sound_level.toFixed(1)}dB<br>
                            Motion Events: ${record.motion_events_count}
                        </p>` : ''}
                    </div>
                `;
            }).join('');
            
            container.innerHTML = recordsHTML;
        } catch (error) {
            container.innerHTML = `
                <p class="loading">Error loading sleep records.</p>
                <p class="loading" style="font-size: 0.9rem; margin-top: 10px;">
                    ${error.message}
                </p>
            `;
        }
    }

    /**
     * Refresh all ML analysis data
     */
    async refreshAnalysis() {
        await this.displayLatestQuality();
        await this.displayRecentRecords();
    }
}

// Export singleton instance
const apiManager = new APIManager();
