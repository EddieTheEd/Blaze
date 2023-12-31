
const currentTheme = localStorage.getItem('theme') ? localStorage.getItem('theme') : 'dark';

if (currentTheme) {
    document.documentElement.setAttribute('data-theme', currentTheme);
    if (currentTheme === 'light') {
      mode = 'light';
    }
    else {
      mode = 'dark';
    }
}


