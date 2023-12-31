function checkPageExistence(pagePath) {
  return fetch(pagePath)
    .then(response => {
      if (response.ok) {
        return true; // Page exists
      } else {
        return false; // Page does not exist
      }
    })
    .catch(() => {
      return false; // Fetch failed, page does not exist
    });
}

document.addEventListener("DOMContentLoaded", function() {
    const currentPageName = window.location.pathname.split("/").pop();
    const lastDashIndex = currentPageName.lastIndexOf("-");
    
    if (lastDashIndex !== -1) {
        const baseName = currentPageName.substring(0, lastDashIndex);
        const pageNumber = currentPageName.substring(lastDashIndex + 1);

        const previousPageNumber = parseInt(pageNumber) - 1;
        const nextPageNumber = parseInt(pageNumber) + 1;

        checkPageExistence(`${baseName}-${previousPageNumber}` + ".html").then(pageExists => {
          if (pageExists) {
            createLink("← Previous", `${baseName}-${previousPageNumber}`);
            checkPageExistence(`${baseName}-${nextPageNumber}` + ".html").then(pageExists => {
              if (pageExists) {
                createLink("Next →", `${baseName}-${nextPageNumber}`);
              } else {
                createLink("", `${baseName}-${nextPageNumber}`);
              }
            });
          } else {
            createLink("", `${baseName}-${previousPageNumber}`);
            checkPageExistence(`${baseName}-${nextPageNumber}` + ".html").then(pageExists => {
              if (pageExists) {
                createLink("Next →", `${baseName}-${nextPageNumber}`);
              } else {
                createLink("", `${baseName}-${nextPageNumber}`);
              }
            });
          }
        });
        
    }

    function createLink(text, link) {
      const linkElement = document.createElement("a");
      linkElement.id = "nextpreviouslink";
      linkElement.textContent = text;
      linkElement.href = link + ".html";
      linkElement.addEventListener("click", function() {
          window.location.href = linkElement.href;
      });

      const linksDiv = document.getElementById("nextpreviouslinks");
      linksDiv.appendChild(linkElement);
      
      const linkElement2 = document.createElement("a");
      linkElement2.id = "nextpreviouslink";
      linkElement2.textContent = text;
      linkElement2.href = link + ".html";
      linkElement2.addEventListener("click", function() {
          window.location.href = linkElement2.href;
      });

      const linksDiv2 = document.getElementById("nextpreviouslinks2");
      linksDiv2.appendChild(linkElement2);
    }
});
