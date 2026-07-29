const inputTextarea = document.getElementById("flipboard-input");
const sendButton = document.getElementById("flipboard-send-button");

sendButton.addEventListener("click", () => {
  const message = inputTextarea.value.trim();
  const params = new URLSearchParams(location.search);

  if (message) {
    fetch(`/flipboard/?token=${params.get("token")}`, {
      method: "POST",
      body: message,
    });

    inputTextarea.value = "";
  }
});
