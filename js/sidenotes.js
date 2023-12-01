try {
    document.getElementById("sidenotetitle").innerHTML = "Sidenotes";
    document.getElementById("rightside").appendChild(document.querySelector(".footnotes"));

    let rightside = document.getElementById("rightside");
    let section = document.querySelector(".footnotes");
    section.lastElementChild.classList.add("footnoteslist");
    let sidenotes = document.querySelector(".footnotes").lastElementChild.children;
    let footnotenums = document.querySelectorAll('[aria-describedby="footnote-label"]');

    while (section.firstChild) {
        rightside.insertBefore(section.firstChild, section);
    }

    rightside.removeChild(section);
    rightside.removeChild(document.querySelector("#footnote-label"));

    let footnoteslist = document.querySelector(".footnoteslist")

    while (footnoteslist.firstChild) {
        rightside.insertBefore(footnoteslist.firstChild, footnoteslist);
    }

    rightside.removeChild(footnoteslist);

    let realfootnotes = document.querySelectorAll("[id*=user-content-fn-]")
    for (i = 0; i<realfootnotes.length; i++) {
        realfootnotes[i].classList.add("temp");
    }

    let realfootnotenums = document.querySelectorAll("[id*=user-content-fnref]")

    for (i = 0; i<realfootnotes.length; i++) {

        let footnotecontent = document.createElement("span");
        footnotecontent.innerHTML = (i+1) + ". " + realfootnotes[i].firstElementChild.innerHTML;
        footnotecontent.classList.add("sidenote");
        footnotecontent.id = realfootnotes[i].id;


        realfootnotenums[i].parentElement.insertBefore(footnotecontent, realfootnotenums[i]);
        realfootnotenums[i].parentElement.insertBefore(realfootnotenums[i], footnotecontent);
        
        realfootnotenums[i].classList.add("sidenoteelement");
        
    }

    let goingtodiefootnotes = document.querySelectorAll(".temp");
    for (i = 0; i<goingtodiefootnotes.length; i++) { 
        goingtodiefootnotes[i].remove();
    }
    let sidenotesvalue = true;
  }
  catch(err) {
    console.log("no footnotes");
    sidenotetitle = document.getElementById("sidenotetitle");
    sidenotetitle.remove();
    page = document.getElementById("page");
    page.style.gridTemplateColumns = "0.18fr 1.5fr"

    body = document.body;
    body.style.marginRight = "0px !important";
    body.style.maxWidth = "90%";
    let sidenotesvalue = false;
  }
