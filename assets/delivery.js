(() => {
  "use strict";

  const message = document.querySelector('textarea[name="message"]');
  if (!message) return;
  let previousBrief = "";

  /** Replaces only the last inserted brief, preserving the visitor's own message. */
  function insertBrief(nextBrief) {
    const current = message.value;
    message.value = previousBrief && current.startsWith(previousBrief)
      ? nextBrief + current.slice(previousBrief.length)
      : nextBrief + (current ? "\n\n" + current : "");
    previousBrief = nextBrief;
    message.dispatchEvent(new Event("input", { bubbles: true }));
    message.focus({ preventScroll: true });
    message.scrollIntoView({ block: "center", behavior: "instant" });
  }

  document.querySelectorAll("[data-solution-brief]").forEach((link) => {
    link.addEventListener("click", (event) => {
      if (event.button !== 0 || event.ctrlKey || event.metaKey || event.shiftKey || event.altKey) return;
      event.preventDefault();
      if (location.hash !== link.hash) history.pushState(null, "", link.hash);
      insertBrief(link.dataset.solutionBrief);
    });
  });

  const preview = document.querySelector(".delivery-preview");
  if (!preview) return;
  const select = preview.querySelector("select");
  const panels = [...preview.querySelectorAll("[data-scope]")];
  const extras = [...preview.querySelectorAll('input[name="delivery-extra"]')];
  const chips = [...preview.querySelectorAll("[data-extra-chip]")];
  const brief = preview.querySelector(".delivery-brief");

  function updatePreview() {
    panels.forEach((panel) => { panel.hidden = panel.dataset.scope !== select.value; });
    chips.forEach((chip, i) => { chip.hidden = !extras[i].checked; });
  }

  function addBrief() {
    const panel = panels.find((item) => item.dataset.scope === select.value);
    const parts = [panel.querySelector("h3").textContent];
    parts.push(...[...panel.querySelectorAll(".system-modules li")].map((item) => item.children[1].textContent));
    parts.push(...chips.filter((chip) => !chip.hidden).map((chip) => chip.textContent));
    insertBrief(parts.join("\n"));
  }

  select.addEventListener("change", updatePreview);
  extras.forEach((input) => input.addEventListener("change", updatePreview));
  brief.addEventListener("click", addBrief);
  updatePreview();
  preview.querySelectorAll(".delivery-controls").forEach((control) => { control.hidden = false; });
})();
