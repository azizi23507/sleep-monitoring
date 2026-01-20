/**
 * Sleep Quality Heatmap Calendar - Fixed Alignment & Tooltip
 */

const heatmapManager = {
    data: [],
    tooltip: null,
    CELL_SIZE: 12,
    GAP: 3,

    init(sleepRecords) {
        this.data = sleepRecords;
        this.createTooltip();
        this.render();
    },

    createTooltip() {
        if (!this.tooltip) {
            this.tooltip = document.createElement('div');
            this.tooltip.className = 'heatmap-tooltip';
            // Essential styles to make sure it's visible
            Object.assign(this.tooltip.style, {
                position: 'absolute',
                display: 'none',
                backgroundColor: 'rgba(0, 0, 0, 0.8)',
                color: '#fff',
                padding: '8px',
                borderRadius: '4px',
                fontSize: '12px',
                pointerEvents: 'none',
                zIndex: '1000',
                boxShadow: '0 2px 5px rgba(0,0,0,0.2)'
            });
            document.body.appendChild(this.tooltip);
        }
    },

    render() {
        const container = document.getElementById('sleep-heatmap');
        if (!container) return;

        const today = new Date();
        const oneYearAgo = new Date(today.getFullYear() - 1, today.getMonth(), today.getDate());

        const allDays = [];
        for (let d = new Date(oneYearAgo); d <= today; d.setDate(d.getDate() + 1)) {
            const dateStr = d.toISOString().split('T')[0];
            const record = this.data.find(r => r.sleep_date === dateStr);
            allDays.push({
                date: new Date(d),
                dateStr: dateStr,
                record: record || null
            });
        }

        const grid = document.createElement('div');
        grid.style.display = 'inline-block'; // Prevents unexpected stretching

        // Logic shared by both labels and grid
        const firstDayPadding = allDays[0].date.getDay();
        const totalWeeks = Math.ceil((allDays.length + firstDayPadding) / 7);

        grid.appendChild(this.buildMonthLabels(allDays, firstDayPadding, totalWeeks));
        grid.appendChild(this.buildWeekGrid(allDays, firstDayPadding));

        container.innerHTML = '';
        container.appendChild(grid);
    },

    buildMonthLabels(days, padding, totalWeeks) {
        const row = document.createElement('div');
        row.style.display = 'flex';
        row.style.marginLeft = '35px';
        row.style.marginBottom = '5px';
        row.style.fontSize = '11px';
        row.style.color = '#666';

        let currentMonth = -1;
        let lastLabelWeek = -10; // Track when we last placed a label
        const colWidth = this.CELL_SIZE + this.GAP;

        for (let w = 0; w < totalWeeks; w++) {
            const labelWrapper = document.createElement('div');
            labelWrapper.style.width = `${colWidth}px`;

            const dayIdx = (w * 7) - padding;
            if (dayIdx >= 0 && dayIdx < days.length) {
                const day = days[dayIdx];
                const month = day.date.getMonth();

                // Fix: Only add label if it's a new month AND at least 2 weeks have passed since the last label
                if (month !== currentMonth && (w - lastLabelWeek) > 2 && w < totalWeeks - 1) {
                    labelWrapper.textContent = day.date.toLocaleDateString('en-US', { month: 'short' });
                    labelWrapper.style.overflow = 'visible';
                    labelWrapper.style.whiteSpace = 'nowrap';
                    currentMonth = month;
                    lastLabelWeek = w;
                }
            }
            row.appendChild(labelWrapper);
        }
        return row;
    },

    buildWeekGrid(days, padding) {
        const container = document.createElement('div');
        container.style.display = 'flex';

        // 1. Day Labels (Y-Axis)
        const dayLabels = document.createElement('div');
        dayLabels.style.display = 'flex';
        dayLabels.style.flexDirection = 'column';
        dayLabels.style.gap = `${this.GAP}px`;
        dayLabels.style.fontSize = '10px';
        dayLabels.style.color = '#666';
        dayLabels.style.width = '35px';

        ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].forEach((label, i) => {
            const div = document.createElement('div');
            div.textContent = (i % 2 === 0) ? label : ''; // Show Sun, Tue, Thu, Sat or similar
            div.style.height = `${this.CELL_SIZE}px`;
            dayLabels.appendChild(div);
        });
        container.appendChild(dayLabels);

        // 2. The Grid Cells
        const cellsContainer = document.createElement('div');
        cellsContainer.style.display = 'flex';
        cellsContainer.style.gap = `${this.GAP}px`;

        let week = [];
        // Apply start padding
        for (let i = 0; i < padding; i++) week.push(null);

        days.forEach(day => {
            week.push(day);
            if (week.length === 7) {
                cellsContainer.appendChild(this.createWeekColumn(week));
                week = [];
            }
        });

        // Apply end padding
        if (week.length > 0) {
            while (week.length < 7) week.push(null);
            cellsContainer.appendChild(this.createWeekColumn(week));
        }

        container.appendChild(cellsContainer);
        return container;
    },

    createWeekColumn(week) {
        const col = document.createElement('div');
        col.style.display = 'flex';
        col.style.flexDirection = 'column';
        col.style.gap = `${this.GAP}px`;

        week.forEach(day => {
            const cell = document.createElement('div');
            cell.style.width = `${this.CELL_SIZE}px`;
            cell.style.height = `${this.CELL_SIZE}px`;
            cell.style.borderRadius = '2px';

            if (!day) {
                cell.style.backgroundColor = 'transparent';
            } else {
                const record = day.record;
                cell.style.backgroundColor = '#ebedf0';

                if (record) {
                    const score = record.quality_score;
                    if (score >= 80) cell.style.backgroundColor = '#0d9373';
                    else if (score >= 60) cell.style.backgroundColor = '#6bcf7f';
                    else if (score >= 40) cell.style.backgroundColor = '#ffd93d';
                    else cell.style.backgroundColor = '#ff6b6b';

                    cell.style.cursor = 'pointer';
                    // Re-attaching events specifically
                    cell.onmouseenter = (e) => this.showTooltip(e, day, record);
                    cell.onmouseleave = () => this.hideTooltip();
                }
            }
            col.appendChild(cell);
        });
        return col;
    },

    showTooltip(event, day, record) {
        const date = day.date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
        this.tooltip.innerHTML = `
            <div style="font-weight:bold; border-bottom:1px solid #555; margin-bottom:4px; padding-bottom:2px">${date}</div>
            <div>Score: <strong>${record.quality_score}</strong></div>
            <div>Sleep: ${record.sleep_duration_hours || 'N/A'}h</div>
            <div style="font-style:italic; color:#aaa; margin-top:4px">${record.classification || ''}</div>
        `;
        this.tooltip.style.display = 'block';
        this.updateTooltipPosition(event);
    },

    updateTooltipPosition(event) {
        // Offset so it doesn't appear exactly under the cursor
        this.tooltip.style.left = (event.pageX + 15) + 'px';
        this.tooltip.style.top = (event.pageY + 15) + 'px';
    },

    hideTooltip() {
        if (this.tooltip) this.tooltip.style.display = 'none';
    }
};