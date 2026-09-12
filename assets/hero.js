/* ═══════════════════════════════════════════════════════════════
   SMART DAWN — client runtime
   · WASM-powered "neural constellation" hero (hero-wasm crate)
   · Scroll reveals, animated counters, magnetic cursor glow
   · Lightweight video lightbox for project demos
   All progressive enhancement: the SSR page works without any of it.
   ═══════════════════════════════════════════════════════════════ */
(() => {
  "use strict";

  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  /* ───────────────────────── Neural constellation ───────────────────────── */
  async function bootConstellation() {
    const canvas = document.getElementById("neural");
    if (!canvas) return;
    const host = canvas.parentElement;
    const motion = window.matchMedia("(prefers-reduced-motion: reduce)");
    if (motion.matches) {
      host.dataset.engine = "static";
      return;
    }
    const ctx = canvas.getContext("2d", { alpha: true });
    if (!ctx) return;
    let wasm;
    try {
      const response = await fetch("/assets/hero.wasm");
      if (!response.ok) throw new Error("WASM unavailable");
      const bytes = await response.arrayBuffer();
      wasm = (await WebAssembly.instantiate(bytes, {})).instance.exports;
      if (typeof wasm.build_links !== "function") throw new Error("WASM version mismatch");
    } catch {
      host.dataset.engine = "static";
      return;
    }
    host.dataset.engine = "wasm";
    const toggle = host.querySelector(".motion-toggle");
    let cssW = 0, cssH = 0;
    let view, links;
    let raf = 0, last = 0, visible = true, paused = false;
    let px = -999, py = -999, active = 0;

    function resize() {
      const bounds = host.getBoundingClientRect();
      cssW = Math.max(1, bounds.width);
      cssH = Math.max(1, bounds.height);
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      canvas.width = Math.round(cssW * dpr);
      canvas.height = Math.round(cssH * dpr);
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      wasm.resize(cssW, cssH);
    }

    function draw() {
      const count = wasm.build_links(132);
      links = new Float32Array(wasm.memory.buffer, wasm.links_ptr(), count * 5);
      view = new Float32Array(wasm.memory.buffer, wasm.positions_ptr(), wasm.count() * 2);
      ctx.clearRect(0, 0, cssW, cssH);
      ctx.lineWidth = 1;
      for (let i = 0; i < count; i++) {
        const offset = i * 5;
        ctx.strokeStyle = "rgba(232,145,58," + links[offset + 4] + ")";
        ctx.beginPath();
        ctx.moveTo(links[offset], links[offset + 1]);
        ctx.lineTo(links[offset + 2], links[offset + 3]);
        ctx.stroke();
      }
      for (let i = 0; i < view.length; i += 2) {
        ctx.beginPath();
        ctx.arc(view[i], view[i + 1], 1.6, 0, Math.PI * 2);
        ctx.fillStyle = "rgba(244,168,80,0.55)";
        ctx.fill();
      }
    }

    function frame(time) {
      raf = 0;
      wasm.pointer(px, py, active);
      wasm.step(last ? Math.min((time - last) / 1000, 0.05) : 0.016);
      last = time;
      draw();
      raf = requestAnimationFrame(frame);
    }

    function syncMotion() {
      cancelAnimationFrame(raf);
      raf = 0;
      last = 0;
      const running = visible && !document.hidden && !paused && !motion.matches;
      host.dataset.motion = running ? "running" : "paused";
      if (toggle) {
        toggle.hidden = motion.matches;
        toggle.setAttribute("aria-pressed", String(paused));
      }
      if (running) raf = requestAnimationFrame(frame);
    }

    const bounds = host.getBoundingClientRect();
    wasm.seed(0x5d2a91c7);
    wasm.init(bounds.width < 640 ? 70 : bounds.width < 1100 ? 120 : 170, bounds.width, bounds.height);
    resize();
    draw();
    host.addEventListener("pointermove", (event) => {
      const bounds = canvas.getBoundingClientRect();
      px = event.clientX - bounds.left;
      py = event.clientY - bounds.top;
      active = 1;
    });
    host.addEventListener("pointerleave", () => { active = 0; });
    toggle?.addEventListener("click", () => { paused = !paused; syncMotion(); });
    document.addEventListener("visibilitychange", syncMotion);
    motion.addEventListener("change", syncMotion);
    window.addEventListener("pagehide", () => { cancelAnimationFrame(raf); });
    window.addEventListener("pageshow", syncMotion);
    new IntersectionObserver((entries) => {
      visible = entries[0].isIntersecting;
      syncMotion();
    }, { threshold: 0.02 }).observe(host);
    new ResizeObserver(() => { resize(); draw(); }).observe(host);
    syncMotion();
  }

  function bootReveals() {
    const els = document.querySelectorAll("[data-reveal]");
    if (!els.length) return;
    if (reduceMotion) {
      els.forEach((el) => el.classList.add("is-visible"));
      return;
    }
    const io = new IntersectionObserver((entries, obs) => {
      entries.forEach((en) => {
        if (en.isIntersecting) {
          en.target.classList.add("is-visible");
          obs.unobserve(en.target);
        }
      });
    }, { threshold: 0.12, rootMargin: "0px 0px -8% 0px" });
    els.forEach((el) => io.observe(el));
  }

  /* ───────────────────────── Animated counters ───────────────────────── */
  function bootCounters() {
    const els = document.querySelectorAll("[data-count]");
    if (!els.length || reduceMotion) {
      els.forEach((el) => { el.textContent = el.dataset.count; });
      return;
    }
    const fmt = (raw, v) => {
      const prefix = raw.match(/^[^\d]*/)[0];
      const suffix = raw.match(/[^\d]*$/)[0];
      return prefix + Math.round(v).toLocaleString("sk-SK") + suffix;
    };
    const run = (el) => {
      const raw = el.dataset.count;
      const target = parseFloat(raw.replace(/[^\d.]/g, "")) || 0;
      const dur = 1500;
      let start = 0;
      const tick = (t) => {
        if (!start) start = t;
        const p = Math.min((t - start) / dur, 1);
        const eased = 1 - Math.pow(1 - p, 3);
        el.textContent = fmt(raw, target * eased);
        if (p < 1) requestAnimationFrame(tick);
        else el.textContent = raw;
      };
      requestAnimationFrame(tick);
    };
    const io = new IntersectionObserver((entries, obs) => {
      entries.forEach((en) => {
        if (en.isIntersecting) { run(en.target); obs.unobserve(en.target); }
      });
    }, { threshold: 0.5 });
    els.forEach((el) => io.observe(el));
  }

  /* ───────────────────────── Video lightbox ───────────────────────── */
  function bootLightbox() {
    const triggers = document.querySelectorAll("[data-video]");
    if (!triggers.length) return;

    const overlay = document.createElement("div");
    overlay.className = "lightbox";
    overlay.setAttribute("aria-hidden", "true");
    overlay.innerHTML =
      '<div class="lightbox-inner" role="dialog" aria-modal="true" aria-label="Video">' +
      '<button class="lightbox-close" aria-label="Zavrieť">&times;</button>' +
      '<video class="lightbox-video" controls playsinline preload="metadata"></video>' +
      "</div>";
    document.body.appendChild(overlay);
    const video = overlay.querySelector("video");
    const closeBtn = overlay.querySelector(".lightbox-close");

    const open = (src, poster) => {
      video.src = src;
      if (poster) video.poster = poster;
      overlay.classList.add("is-open");
      overlay.setAttribute("aria-hidden", "false");
      document.body.style.overflow = "hidden";
      video.play().catch(() => {});
    };
    const close = () => {
      overlay.classList.remove("is-open");
      overlay.setAttribute("aria-hidden", "true");
      document.body.style.overflow = "";
      video.pause();
      video.removeAttribute("src");
      video.load();
    };

    triggers.forEach((t) => {
      t.addEventListener("click", (e) => {
        e.preventDefault();
        const src = t.getAttribute("data-video");
        if (src) open(src, t.getAttribute("data-poster"));
      });
    });
    overlay.addEventListener("click", (e) => { if (e.target === overlay) close(); });
    closeBtn.addEventListener("click", close);
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && overlay.classList.contains("is-open")) close();
    });
  }

  /* ───────────────────────── Scroll progress + nav state ───────────────────────── */
  function bootChrome() {
    const bar = document.querySelector(".scroll-progress");
    const topbar = document.querySelector(".topbar");
    const onScroll = () => {
      const h = document.documentElement;
      const max = h.scrollHeight - h.clientHeight;
      const p = max > 0 ? h.scrollTop / max : 0;
      if (bar) bar.style.transform = `scaleX(${p})`;
      if (topbar) topbar.classList.toggle("is-scrolled", h.scrollTop > 12);
    };
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
  }

  /* ───────────────────────── Headline word rotator ───────────────────────── */
  function bootRotator() {
    const el = document.querySelector(".word-rotator");
    if (!el || reduceMotion) return;
    const words = (el.dataset.words || "")
      .split(",").map((s) => s.trim()).filter(Boolean);
    if (words.length < 2) return;
    let i = 0;
    setInterval(() => {
      el.classList.add("is-swapping");
      setTimeout(() => {
        i = (i + 1) % words.length;
        el.textContent = words[i];
        el.classList.remove("is-swapping");
      }, 300);
    }, 2400);
  }

  /* ───────────────────────── 3D card tilt ───────────────────────── */
  function bootTilt() {
    if (reduceMotion || window.matchMedia("(hover: none)").matches) return;
    document.querySelectorAll(".project").forEach((card) => {
      card.addEventListener("pointermove", (e) => {
        const r = card.getBoundingClientRect();
        const px = (e.clientX - r.left) / r.width - 0.5;
        const py = (e.clientY - r.top) / r.height - 0.5;
        const max = 5;
        card.style.transform =
          `translateY(-4px) rotateX(${(-py * max).toFixed(2)}deg) rotateY(${(px * max).toFixed(2)}deg)`;
      });
      card.addEventListener("pointerleave", () => { card.style.transform = ""; });
    });
  }

  /* ───────────────────────── boot ───────────────────────── */
  function boot() {
    bootConstellation();
    bootReveals();
    bootCounters();
    bootLightbox();
    bootChrome();
    bootRotator();
    bootTilt();
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot);
  } else {
    boot();
  }
})();
