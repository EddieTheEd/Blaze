let mediatype = navigator.userAgent;
let regexphonetype = /android|iphone|kindle|ipad/i;

let ismobile = regexphonetype.test(mediatype);

console.log(ismobile);
