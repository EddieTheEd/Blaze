let headingsnodelist = document.querySelectorAll("h1, h2, h3, h4, h5, h6");
let allheadings = Array.from(headingsnodelist);
let headings = new Array();
let toc = new Array();

allheadings.forEach(heading => {
    if (heading.id == "") {
        headings.push({content: heading, type: heading.nodeName});
    }
});

headings.forEach(heading => {
  let content = heading.content;
  let type = heading.type
  const section = document.createElement("section");
  section.id = content.textContent.replaceAll(" ", "-").toLowerCase();
  content.parentNode.insertBefore(section, content);
  section.appendChild(content);
  content.id = content.textContent.replaceAll(" ", "-").toLowerCase();
  toc.push({text: content.textContent.replaceAll(" ", "-").toLowerCase(), link: "#" + content.textContent.replaceAll(" ", "-").toLowerCase(), type: type.replace("H", "")})
});

console.log(toc);

toc.forEach(tocelement => {
  let headertext = tocelement.text;
  let headerlink = tocelement.link;
  let elem = document.createElement("a");
  elem.innerText = headertext + tocelement.type;
  elem.href = headerlink;
  //document.body.appendChild(elem);
});
