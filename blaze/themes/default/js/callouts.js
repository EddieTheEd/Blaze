try {
    calloutimages = document.querySelectorAll(".calloutimage");
        for (var i = 0; i < calloutimages.length; i++) {
            callouttype = calloutimages[i].id;
            try {
                calloutimg = document.createElement("img");
                calloutimg.src = window.location.origin + "/callouts/"+callouttype+".svg";
                calloutimg.classList.add('actualcalloutimage');
                calloutimg.style.width = '30px';
                calloutimg.style.height = '30px';
                calloutimages[i].appendChild(calloutimg);

            }
            catch(err) {
                console.log("ERROR: Callout image not found.")
            }
        }
}

catch(err) {
    console.log("No callouts found.")
}

function switchcalloutimagetype(mode){
  const calloutimages = document.querySelectorAll("img");
  for (let i=0; i<calloutimages.length; i++) {
    let element = calloutimages[i];
    if (element.className == 'actualcalloutimage'){
      if (mode=='light'){
    element.src = element.src.replace(".svg", "lightmode.svg");
      }
      else {
        element.src = element.src.replace("lightmode.svg", ".svg");
      }
    }
  }
}
