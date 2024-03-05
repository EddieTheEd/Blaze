const baseUrl = window.location.origin;

fetch(baseUrl + "/lastmod.txt")
  .then(response => {
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    return response.text();
  })
  .then(text => {
    const lines = text.split('\n');
    const result = {};

    for (let i = 0; i < lines.length; i += 2) {
      const key = lines[i];
      const value = lines[i + 1];
      result[key] = value;
    }

    path = window.location.href.replace(baseUrl, "")
    const lastHtmlIndex = path.lastIndexOf(".html");
    if (lastHtmlIndex !== -1) {
      path = path.slice(0, lastHtmlIndex) + ".md";
    }

    if (path === "/") {
      path = "/index.md"
    }

    if (result.hasOwnProperty(path)) {
      try {
        div = document.createElement("div");
        lastmod = document.createElement("i");
        div.append(lastmod);
        document.getElementById("description").insertAdjacentElement('afterend', div);
        div.setAttribute("id", "description")
        lastmod.innerHTML = "Last modified: " + result[path]
      } catch (error) {
        console.log(error)
      }
    } else {
        console.log("Not in lastmod.");
    }

    const dataArray = Object.entries(result);
    dataArray.sort((a, b) => new Date(b[1]) - new Date(a[1]));
    const mostRecent = dataArray.slice(0, 5); // number 5 should be set from blazeconfig.toml, will fix later.
    const mostRecentPages = mostRecent.map(item => item[0]);
    recent = document.getElementById("recent");
    recentdummy = document.getElementById("recentdummytext");
    recentdummy.remove();
    for (const page in mostRecentPages){
      let element = document.createElement("a");
      element.innerHTML = mostRecentPages[page].replace(".md","").substring(mostRecentPages[page].replace(".md","").lastIndexOf('/') + 1).replace("%20", " "); // temp fix, shouldn't be necessary when using page title
      element.href = baseUrl + mostRecentPages[page].replace(".md",".html");
      recent.appendChild(element);
      recent.appendChild(document.createElement("br"));
    }
  })

  .catch(error => {
    console.error('There was a problem fetching the text file:', error);
  });
