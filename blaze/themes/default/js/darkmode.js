

var image = document.getElementById("darklightimg");
mode = 'dark';

const currentTheme = localStorage.getItem('theme') ? localStorage.getItem('theme') : 'dark';

if (currentTheme) {
    document.documentElement.setAttribute('data-theme', currentTheme);

    if (currentTheme === 'light') {
        image.src = image.src.replace("darkmode.svg", "") + "lightmode.svg";
        creategraph("#131330"); 
        mode = 'light';
    }
    else {
      creategraph("#FBFAF5"); 
      mode = 'dark';
    }
}

image.onclick = function(e) {
  if (mode=='dark'){
    console.log('change to light');
    image.src = image.src.replace("darkmode.svg", "").replace("lightmode.svg", "") + "lightmode.svg";
    mode = 'light';
    document.documentElement.setAttribute('data-theme', 'light'); 
    localStorage.setItem('theme', 'light');
    creategraph("#131330"); 
    switchcalloutimagetype(mode)
    
  }
  else {
    console.log('change to light');
    image.src = image.src.replace("lightmode.svg", "").replace("darkmode.svg", "") + "darkmode.svg";
    mode = 'dark';
    document.documentElement.setAttribute('data-theme', 'dark');
    localStorage.setItem('theme', 'dark');
    creategraph("#FBFAF5"); 
    switchcalloutimagetype(mode)
  }
}
