console.log(window.location.href);

function zoomed(event) {
    svg.attr("transform", event.transform);
}
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

function globalgraph(){
  
  let path = "/global.json";

  var request = new XMLHttpRequest();
  request.open('GET', path, false);  
  request.send(null);

  if (request.status === 200) {
    var graphdata = request.responseText;
    } else {
    console.error('Error fetching JSON:', request.statusText);
  }

  let data = JSON.parse(graphdata);
  const uniqueNodesMap = new Map();

  // Filter out duplicate nodes
  data.nodes.forEach(node => {
    uniqueNodesMap.set(node.id, node);
  });

  // Convert Map values back to an array of nodes
  const uniqueNodesArray = Array.from(uniqueNodesMap.values());

  // Update the graph object with unique nodes
  data.nodes = uniqueNodesArray;

  console.log(data);

  links = data.links.map(d => ({...d}));
  nodes = data.nodes.map(d => ({...d}));
  groups = [...new Set(nodes.map(node => node.group))];

  var viewportwidth = window.innerWidth || document.documentElement.clientWidth;
  var viewportheight = window.innerHeight || document.documentElement.clientHeight;
  width = 0.5*viewportwidth;
  height = 0.6*viewportheight;

  svg = d3.select("#globalgraphcontainer")
    .style("position", "relative")
    .append("svg")
    .attr("width", width)
    .attr("height", height)
    .call(d3.zoom().on("zoom", zoomed))
    .append("g");

  simulation = d3.forceSimulation(nodes)
  .force("link", d3.forceLink(links).id(d => d.id))
  .force("charge", d3.forceManyBody())
  .force("center", d3.forceCenter(width / 2, height / 2));

  link = svg.selectAll("line")
  .data(links)
  .enter()
  .append("line")
  .attr("stroke", "#999")
  .attr("stroke-opacity", 0.6)
  .attr("stroke-width", d => Math.sqrt(d.value));

  node = svg.selectAll("a")
  .data(nodes)
  .enter()
  .append("a") 
  .attr("xlink:href", d => d.link) 
  .append("circle") 
  .attr("r", 5)
  .attr("fill", "#1f2e6b");

  labels = svg.selectAll("text")
    .data(nodes)
    .enter()
    .append("text")
    .text(d => d.id)
    .attr("text-anchor", "middle")
    .attr("dy", "-0.25em")
    .attr("fill", "#FBFAF5")
    .attr("font-size", "10px")
    .attr("pointer-events", "none"); 

  tick = () => {
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

  zoom = d3.zoom()
    .scaleExtent([0.5, 2])
    .on("zoom", zoomed);

  d3.select("#graph").call(zoom);
}

function creategraph(textcolour){
  try {
    let graphdiv = document.getElementById("graph");
    let child = graphdiv.lastElementChild;
    graphdiv.removeChild(child);
  } catch (error) {
    console.log(error); 
  }
  
  let path = window.location.pathname.replace(".html","")+".json"/*"/global.json"*/;

  if(path == "/.json"){
    path = "/index.json";
  }

  var request = new XMLHttpRequest();
  request.open('GET', path, false);  
  request.send(null);

  if (request.status === 200) {
    var graphdata = request.responseText;
    } else {
    console.error('Error fetching JSON:', request.statusText);
  }

  /*var graphTitle = document.getElementById("graphTitle");

  graphTitle.innerHTML = window.filename;
  graphTitle.style.color = "#FBFAF5";
  graphTitle.style.textAlign = "center";
  graphTitle.style.margin = "0";
  graphTitle.style.padding = "10px";*/

  let data = JSON.parse(graphdata);
  console.log(data);
  links = data.links.map(d => ({...d}));
  nodes = data.nodes.map(d => ({...d}));
  groups = [...new Set(nodes.map(node => node.group))];

  width = 356;
  height = 356;

  svg = d3.select("#graph")
    .style("position", "relative")
    .append("svg")
    .attr("width", width)
    .attr("height", height)
    .call(d3.zoom().on("zoom", zoomed))
    .append("g");

   // .attr("transform","scale(3)");

  simulation = d3.forceSimulation(nodes)
  .force("link", d3.forceLink(links).id(d => d.id))
  .force("charge", d3.forceManyBody())
  .force("center", d3.forceCenter(width / 2, height / 2));

  link = svg.selectAll("line")
  .data(links)
  .enter()
  .append("line")
  .attr("stroke", "#999")
  .attr("stroke-opacity", 0.6)
  .attr("stroke-width", d => Math.sqrt(d.value));

  node = svg.selectAll("a")
  .data(nodes)
  .enter()
  .append("a") 
  .attr("xlink:href", function(d) { //here's a cheap parlor trick
    if (d.linktype === 'var(--blnode)' || d.linktype === 'var(--root)') {
      return d.link;
    } else {
      return window.location.href.replace(/\/[^/]+$/, '') + d.link;
    }
  })
  .append("circle") 
  .attr("r", 5)
  .attr("fill", d => d.linktype);

  labels = svg.selectAll("text")
    .data(nodes)
    .enter()
    .append("text")
    .text(d => d.id)
    .attr("text-anchor", "middle")
    .attr("dy", "-0.25em")
    .attr("fill", textcolour)
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

  tick = () => {
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

  zoom = d3.zoom()
    .scaleExtent([0.5, 2])
    .on("zoom", zoomed);

  d3.select("#graph").call(zoom);
  console.log("successfully switched to " + textcolour);
}

globalloaded = false;

function openglobalgraph(){
  if (!globalloaded){
    globalgraph();
    globalloaded = true;
  }
  else {

    let viewportwidth = window.innerWidth || document.documentElement.clientWidth;
    let viewportheight = window.innerHeight || document.documentElement.clientHeight;
    updatedwidth = 0.5*viewportwidth;
    updatedheight = 0.6*viewportheight;

    svg.attr("width", updatedwidth)
       .attr("height", updatedheight)

  }
  document.getElementById("globalgraphbackground").style.display = "inline-block";
}

document.addEventListener('keydown', function(event) {
    if (event.key === 'Escape' || event.keyCode === 27) {
      document.getElementById("globalgraphbackground").style.display = "none";
    }
});


