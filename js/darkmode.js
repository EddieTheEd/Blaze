var image = document.getElementById("darklightimg");
mode = 'dark';

const currentTheme = localStorage.getItem('theme') ? localStorage.getItem('theme') : null;

if (currentTheme) {
    document.documentElement.setAttribute('data-theme', currentTheme);

    if (currentTheme === 'light') {
        image.src = image.src.replace("darkmode.svg", "") + "lightmode.svg";
        mode = 'light';
    }
    else {
      mode = 'dark';
    }
}

image.onclick = function(e) {
  if (mode=='dark'){
    image.src = image.src.replace("darkmode.svg", "") + "lightmode.svg";
    mode = 'light';
    document.documentElement.setAttribute('data-theme', 'light'); 
  }
  else {
    image.src = image.src.replace("lightmode.svg", "") + "darkmode.svg";
    mode = 'dark';
    document.documentElement.setAttribute('data-theme', 'dark');
    localStorage.setItem('theme', 'dark');
  }
}
