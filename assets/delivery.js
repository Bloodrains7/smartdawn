(() => {
  "use strict";

  const preview = document.querySelector(".delivery-preview");
  const message = document.querySelector('textarea[name="message"]');
  if (!preview || !message) return;

  const select = preview.querySelector("select");
  const panels = [...preview.querySelectorAll("[data-scope]")];
  const extras = [...preview.querySelectorAll('input[name="delivery-extra"]')];
  const chips = [...preview.querySelectorAll("[data-extra-chip]")];
  const brief = preview.querySelector(".delivery-brief");
  let previousBrief = "";

  function updatePreview() {
    panels.forEach((panel) => { panel.hidden = panel.dataset.scope !== select.value; });
    chips.forEach((chip, i) => { chip.hidden = !extras[i].checked; });
  }

  /** Replaces only the last inserted brief, preserving the visitor's own message. */
  function addBrief() {
    const panel = panels.find((item) => item.dataset.scope === select.value);
    const parts = [panel.querySelector("h3").textContent];
    parts.push(...[...panel.querySelectorAll(".system-modules li")].map((item) => item.children[1].textContent));
    parts.push(...chips.filter((chip) => !chip.hidden).map((chip) => chip.textContent));
    const nextBrief = parts.join("\n");
    const current = message.value;
    message.value = previousBrief && current.startsWith(previousBrief)
      ? nextBrief + current.slice(previousBrief.length)
      : nextBrief + (current ? "\n\n" + current : "");
    previousBrief = nextBrief;
    message.dispatchEvent(new Event("input", { bubbles: true }));
    message.focus({ preventScroll: true });
    message.scrollIntoView({ block: "center", behavior: "instant" });
  }

  select.addEventListener("change", updatePreview);
  extras.forEach((input) => input.addEventListener("change", updatePreview));
  brief.addEventListener("click", addBrief);
  updatePreview();
  preview.querySelectorAll(".delivery-controls").forEach((control) => { control.hidden = false; });
})();
