window.addEventListener("load", () => {
  const utatenURL =
    "https://utaten.com" +
    document.location.pathname +
    document.location.search;
  document.getElementById("goto_utaten_a").setAttribute("href", utatenURL);
});
