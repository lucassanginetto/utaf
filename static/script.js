window.addEventListener("load", () => {
  const utatenURL =
    "https://utaten.com" +
    document.location.pathname +
    document.location.search;
  document.getElementById("goto_utaten_a").setAttribute("href", utatenURL);

  const furiganaButton = document.getElementById("furigana_button");
  if (furiganaButton) {
    const lyricsDiv = document.getElementById("lyrics_div");
    furiganaButton.addEventListener("click", () => {
      const hidden = lyricsDiv.classList.toggle("no_furigana");
      furiganaButton.textContent = hidden ? "Show furigana" : "Hide furigana";
    });
  }
});
