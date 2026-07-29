const inputTextarea = document.getElementById("flipboard-input");
const sendButton = document.getElementById("flipboard-send-button");
const warningText = document.getElementById("flipboard-warning");

sendButton.addEventListener("click", async () => {
  const message = inputTextarea.value.trim();
  const params = new URLSearchParams(location.search);

  if (message) {
    let response = await fetch(`/flipboard/?token=${params.get("token")}`, {
      method: "POST",
      body: message,
    });

    if (response.status === 403) {
      warningText.hidden = false;
    } else {
      warningText.hidden = true;
    }

    inputTextarea.value = "";
  }
});
