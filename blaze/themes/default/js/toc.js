let headingsnodelist = document.querySelectorAll("h1, h2, h3, h4, h5, h6");
let allheadings = Array.from(headingsnodelist);
let headings = new Array();
let toc = new Array();
let toccontent = document.getElementById("toccontent");
let orderedtoc = new Array();

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
  toc.push({text: content.textContent, link: "#" + content.textContent.replaceAll(" ", "-").toLowerCase(), type: type.replace("H", "")})
});

function makelink(index, array, aparent) {
  let headertext = array[index].text;
  let headerlink = array[index].link;
  let elem = document.createElement("a");
  let listelement = document.createElement("li");
  elem.innerText = headertext;
  elem.href = headerlink;
  listelement.appendChild(elem);
  aparent.appendChild(listelement);
}

// this part needs fixing- it only works for up to h2 and h3. ideally we want it to work up to 
// h2-h6. this is probably best done by comparing the difference in type, i.e. h2-h3 differ by 
// 1, h2-h4 differ by 2, etc., using the value (pos, neg) to determine whether the level of 
// "nesting". say, if it was h2, h3and h4, the final list should look like this:
// [h2elem, [h3elem, [h4 element]]]. after this, the rest of the code *should* work.

for (let index = 0; index < toc.length; index++) {

  if (toc[index].type === "2") {
    orderedtoc.push(toc[index]);
  } else if (toc[index].type === "3") {
    if (Array.isArray(orderedtoc[orderedtoc.length - 1])) {
        let array = orderedtoc[orderedtoc.length - 1];
        array.push(toc[index]);
        orderedtoc[orderedtoc.length - 1] = array;
    } else {
      orderedtoc.push([toc[index]]);
    }
  }
}

// tricky stuff ends here

for (let index = 0; index < orderedtoc.length; index++) {
 
  if (Array.isArray(orderedtoc[index])){
    let hthreelist = document.createElement("ol");
    for(let j=0; j<orderedtoc[index].length; j++){
      makelink(j, orderedtoc[index], hthreelist);
      console.log("add element " + j + "in subarray");
    }
    toccontent.appendChild(hthreelist);
   
  }
  else {
    makelink(index, orderedtoc, toccontent);
    console.log("add element " + index + "in mainarray");
  }
  
  
}


