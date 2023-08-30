
if (ismobile){
  let lhs = document.getElementById("leftside").children;
  console.log(lhs);
  document.getElementById("page").style.display = "block";
  document.getElementById("page").appendChild(lhs[1]);
  document.getElementById("page").insertBefore(lhs[0], document.getElementById("page").firstChild);
  document.getElementById("leftside").remove();

  console.log("mobile")
}

