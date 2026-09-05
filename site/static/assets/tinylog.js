const inputTextarea = document.getElementById("tinylog-input");
const sendButton = document.getElementById("tinylog-send-button");
const authWarningText = document.getElementById("tinylog-permission-warning");
const successText = document.getElementById("tinylog-success");

const TOAST_TTL_MILLIS = 5000;

sendButton.addEventListener("click", async () => {
  const message = inputTextarea.value.trim();
  const params = new URLSearchParams(location.search);

  if (message) {
    let response = await fetch("https://tinylog-bot.lark.gay/", {
      method: "POST",
      headers: {
        Authorization: `Bearer ${params.get("token")}`,
      },
      body: JSON.stringify({
        message: message,
      }),
    });

    if (response.status === 401 || response.status === 403) {
      authWarningText.hidden = false;

      setTimeout(() => {
        authWarningText.hidden = true;
      }, TOAST_TTL_MILLIS);
    } else {
      authWarningText.hidden = true;
    }

    if (response.status === 200) {
      successText.hidden = false;

      setTimeout(() => {
        successText.hidden = true;
      }, TOAST_TTL_MILLIS);
    } else {
      successText.hidden = true;
    }

    inputTextarea.value = "";
  }
});
