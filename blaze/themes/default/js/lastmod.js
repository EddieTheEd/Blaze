const baseUrl = window.location.origin;
console.log(baseUrl);

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
    
    
  })
  .catch(error => {
    console.error('There was a problem fetching the text file:', error);
  });
