export function renderGauges(containerId, data) {
  const container = document.getElementById(containerId);
  if (!container) return;
  if (data.length === 0) {
    container.innerHTML = "<p>En attente de données...</p>";
    return;
  }

  const latest = data[data.length - 1];
  container.innerHTML = `
    <div class="gauges">
      ${gaugeCard("Température", latest.temp.toFixed(1), "°C")}
      ${gaugeCard("Humidité", latest.hum.toFixed(1), "%")}
      ${gaugeCard("Son", latest.sound_db.toFixed(1), "dB")}
    </div>
  `;
}

function gaugeCard(label, value, unit) {
  return `
    <div class="gauge-card">
      <div class="gauge-label">${label}</div>
      <div class="gauge-value">${value}<span class="gauge-unit"> ${unit}</span></div>
    </div>
  `;
}
