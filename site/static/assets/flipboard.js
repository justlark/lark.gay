const inputTextarea = document.getElementById("flipboard-input");
const sendButton = document.getElementById("flipboard-send-button");
const warningText = document.getElementById("flipboard-warning");
const successText = document.getElementById("flipboard-success");

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

      setTimeout(() => {
        warningText.hidden = true;
      }, 5000);
    } else {
      warningText.hidden = true;
    }

    if (response.status === 200) {
      successText.hidden = false;

      setTimeout(() => {
        successText.hidden = true;
      }, 5000);
    } else {
      successText.hidden = true;
    }

    inputTextarea.value = "";
  }
});
