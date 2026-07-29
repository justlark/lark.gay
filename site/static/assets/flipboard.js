const inputTextarea = document.getElementById("flipboard-input");
const sendButton = document.getElementById("flipboard-send-button");
const authWarningText = document.getElementById("flipboard-permission-warning");
const lengthWarningText = document.getElementById("flipboard-length-warning");
const successText = document.getElementById("flipboard-success");

const TOAST_TTL_MILLIS = 5000;
const MAX_LEN_GRAPHEMES = 45;

sendButton.addEventListener("click", async () => {
  const message = inputTextarea.value.trim();
  const params = new URLSearchParams(location.search);

  if (message) {
    const messageLen = [...new Intl.Segmenter().segment(message)].length;

    if (messageLen > MAX_LEN_GRAPHEMES) {
      lengthWarningText.hidden = false;

      setTimeout(() => {
        lengthWarningText.hidden = true;
      }, TOAST_TTL_MILLIS);

      return;
    }

    let response = await fetch(`/flipboard/?token=${params.get("token")}`, {
      method: "POST",
      body: message,
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
