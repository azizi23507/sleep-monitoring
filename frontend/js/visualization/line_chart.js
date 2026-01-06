export function renderSoundChart(svgId, data) {
  renderLine(svgId, data, d => d.sound_db, [0, 120]);
}

export function renderTempChart(svgId, data) {
  renderLine(svgId, data, d => d.temp, [10, 35]); // à ajuster
}

function renderLine(svgId, data, accessor, yDomain) {
  const svg = d3.select(`#${svgId}`);
  svg.selectAll("*").remove();
  if (!data || data.length < 2) return;

  const margin = { top: 10, right: 20, bottom: 30, left: 40 };
  const width = +svg.attr("width") - margin.left - margin.right;
  const height = +svg.attr("height") - margin.top - margin.bottom;

  const g = svg.append("g")
    .attr("transform", `translate(${margin.left},${margin.top})`);

  const x = d3.scaleTime()
    .domain(d3.extent(data, d => new Date(d.timestamp || Date.now())))
    .range([0, width]);

  const y = d3.scaleLinear()
    .domain(yDomain)
    .range([height, 0]);

  const line = d3.line()
    .x(d => x(new Date(d.timestamp || Date.now())))
    .y(d => y(accessor(d)));

  g.append("path")
    .datum(data.slice(-200))
    .attr("d", line)
    .attr("stroke", "steelblue")
    .attr("stroke-width", 1.8)
    .attr("fill", "none");

  g.append("g")
    .attr("transform", `translate(0,${height})`)
    .call(d3.axisBottom(x).ticks(4));

  g.append("g")
    .call(d3.axisLeft(y).ticks(4));
}
