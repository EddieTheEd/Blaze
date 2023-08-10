let path = window.location.pathname.replace(".html",".json");

if(path == "/"){
  path = "/index.json";
}

console.log(path)

var request = new XMLHttpRequest();
request.open('GET', path, false);  
request.send(null);

if (request.status === 200) {
  var graphdata = JSON.parse(request.responseText);
  } else {
  console.error('Error fetching JSON:', request.statusText);
}



  /*var graphTitle = document.getElementById("graphTitle");

  graphTitle.innerHTML = window.filename;
  graphTitle.style.color = "#FBFAF5";
  graphTitle.style.textAlign = "center";
  graphTitle.style.margin = "0";
  graphTitle.style.padding = "10px";*/

  let data = graphdata;


  const links = data.links.map(d => ({...d}));
  const nodes = data.nodes.map(d => ({...d}));
  const groups = [...new Set(nodes.map(node => node.group))];

  const width = 356;
  const height = 356;

  const svg = d3.select("#graph")
    .style("position", "relative")
    .append("svg")
    .attr("width", width)
    .attr("height", height)
    .call(d3.zoom().on("zoom", zoomed))
    .append("g");

   // .attr("transform","scale(3)");

  const simulation = d3.forceSimulation(nodes)
  .force("link", d3.forceLink(links).id(d => d.id))
  .force("charge", d3.forceManyBody())
  .force("center", d3.forceCenter(width / 2, height / 2));

  const link = svg.selectAll("line")
  .data(links)
  .enter()
  .append("line")
  .attr("stroke", "#999")
  .attr("stroke-opacity", 0.6)
  .attr("stroke-width", d => Math.sqrt(d.value));



  const node = svg.selectAll("a")
  .data(nodes)
  .enter()
  .append("a") 
  .attr("xlink:href", d => d.link) 
  .append("circle") 
  .attr("r", 5)
  .attr("fill", d => d.linktype);

  
  const labels = svg.selectAll("text")
  .data(nodes)
  .enter()
  .append("text")
  .text(d => d.id)
  .attr("text-anchor", "middle")
  .attr("dy", "-0.25em")
  .attr("fill", "#FBFAF5")
  .attr("font-size", "10px")
  .attr("pointer-events", "none");

  /*const legend = d3.select("#legend");

  const legendtitle = document.getElementById("legendtitle");
  legendtitle.innerHTML = "Legend";
  legendtitle.style.color = "#FBFAF5";
  legendtitle.style.margin = "0";
  legendtitle.style.paddingBottom = "10px";
  legendtitle.style.textAlign = "left";

  const legendItems = legend.selectAll("#legend-item")
    .data(groups)
    .enter()
    .append("div")
    .attr("id", "legend-item");
  
  legendItems.append("span")
    .style("background-color", d => nodes.find(node => node.group === d).color)
    .attr("id", "legend-color");
  
  legendItems.append("span")
    .text(d => d)
    .attr("id", "legend-label");
*/

  const tick = () => {
  link
      .attr("x1", d => d.source.x)
      .attr("y1", d => d.source.y)
      .attr("x2", d => d.target.x)
      .attr("y2", d => d.target.y);

  node
      .attr("cx", d => d.x)
      .attr("cy", d => d.y);

  labels
      .attr("x", d => d.x)
      .attr("y", d => d.y);
  };

  simulation.on("tick", tick);

  node.call(d3.drag()
  .on("start", dragstarted)
  .on("drag", dragged)
  .on("end", dragended));

  function dragstarted(event) {
  if (!event.active) simulation.alphaTarget(0.3).restart();
  event.subject.fx = event.subject.x;
  event.subject.fy = event.subject.y;
  }

  function dragged(event) {
  event.subject.fx = event.x;
  event.subject.fy = event.y;
  }

  function dragended(event) {
  if (!event.active) simulation.alphaTarget(0);
  event.subject.fx = null;
  event.subject.fy = null;
  }

  function dragged(event) {
  event.subject.fx = event.x;
  event.subject.fy = event.y;
  }

  function dragended(event) {
  if (!event.active) simulation.alphaTarget(0);
  event.subject.fx = null;
  event.subject.fy = null;
  }

  const zoom = d3.zoom()
      .scaleExtent([0.5, 2])
      .on("zoom", zoomed);

  function zoomed(event) {
      svg.attr("transform", event.transform);
  }

  d3.select("#graph").call(zoom);

