export const getStatus = () =>
  fetch("/status", { method: "GET" }).then((resp) => {
    if (!resp.ok) {
      alert(`Something wrong: ${resp.statusText} ${resp.status}`);
    } else {
      return resp.json();
    }
  });

export const enable = () =>
  fetch("/enable", {
    method: "POST",
  }).then((resp) => {
    if (!resp.ok) {
      alert(`Something wrong: ${resp.statusText} ${resp.status}`);
    }
  });

export const disable = () =>
  fetch("/disable", {
    method: "POST",
  }).then((resp) => {
    if (!resp.ok) {
      alert(`Something wrong: ${resp.statusText} ${resp.status}`);
    }
  });

export const setBrightness = (val) =>
  fetch("/brightness", {
    method: "POST",
    body: val,
  }).then((resp) => {
    if (!resp.ok) {
      alert(`Something wrong: ${resp.statusText} ${resp.status}`);
    }
  });

export const setColor = (color) =>
  fetch("/color", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      red: color.detail.r,
      green: color.detail.g,
      blue: color.detail.b,
    }),
  }).then((resp) => {
    if (!resp.ok) {
      alert(`Something wrong: ${resp.statusText} ${resp.status}`);
    }
  });
