let mediatype = navigator.userAgent;
let regexphonetype = /android|iphone|kindle|ipad/i;

let ismobile = regexphonetype.test(mediatype);
if (ismobile){
  let lhs = document.getElementById("leftside").children;
  document.getElementById("page").style.display = "block";
  document.getElementById("page").appendChild(lhs[1]);
  document.getElementById("page").insertBefore(lhs[0], document.getElementById("page").firstChild);
  document.getElementById("leftside").remove();

  let sidenotesyes = document.getElementById("sidenotetitle");
  if (sidenotesyes != null){
    document.getElementById("page").style.width = "70%";
    document.getElementById("graph").style.width = "80%";
    document.getElementById("sidenotetitle").style.fontSize = "1.2em";
  }
  document.getElementById("darklightimg").style.right = "5%";
  document.getElementById("darklightimg").style.top = "10%";
  document.getElementById("darklightimg").style.position = "relative";

  document.getElementById("pageh1").style.height = "fit-content";
  document.getElementById("pageh1").style.paddingBottom = "20px";
  document.getElementById("description").style.marginTop = "auto";


}

