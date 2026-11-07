// The check's hands: real events on the real controls of the dev shell, in a real browser. It reads what the page shows, and posts what it saw.
"use strict";
(async () => {
  const report = (body) => fetch("/report", {method: "POST", body: JSON.stringify(body)});
  try {
    const shell = await window.stitchShell.ready;
    shell.quiet = true;
    const results = [];
    const check = (name, held, detail) => results.push({name, held: Boolean(held), detail: detail === undefined ? "" : String(detail)});
    const overlay = document.getElementById("overlay");
    const box = () => overlay.getBoundingClientRect();
    const pointer = (type, x, y) => overlay.dispatchEvent(new PointerEvent(type, {clientX: box().left + x, clientY: box().top + y, bubbles: true, pointerId: 1}));
    const click = (id) => document.getElementById(id).click();
    const type = (token, value) => {
      const field = document.getElementById("input-" + token);
      field.value = value;
      field.dispatchEvent(new Event("change", {bubbles: true}));
    };
    const shown = (token) => Number(document.getElementById("input-" + token).value);
    const cuts = () => shell.state().lines.filter((line) => line.kind === "cut").map((line) => line.points);
    const units = () => shell.state().pixel_units;
    const lines = () => JSON.stringify(shell.state().lines);
    // How dark the picture is around a pixel: a read of what the renderer actually drew.
    const inked = (x, y) => {
      const context = shell.snapshot().getContext("2d", {willReadFrequently: true});
      const data = context.getImageData(Math.round(x) - 2, Math.round(y) - 2, 5, 5).data;
      let darkest = 255;
      for (let at = 0; at < data.length; at += 4) darkest = Math.min(darkest, (data[at] + data[at + 1] + data[at + 2]) / 3);
      return darkest;
    };
    const longest = (points) => {
      let best = 0, length = -1;
      for (let at = 0; at + 3 < points.length; at += 2) {
        const here = Math.hypot(points[at + 2] - points[at], points[at + 3] - points[at + 1]);
        if (here > length) { length = here; best = at; }
      }
      return [(points[best] + points[best + 2]) / 2 / units(), (points[best + 1] + points[best + 3]) / 2 / units()];
    };

    check("all five pieces are drawn", cuts().length === 5, cuts().length);
    const [fx, fy] = longest(cuts()[2]);
    check("the renderer drew ink where the front's fold is", inked(fx, fy) < 128, inked(fx, fy));
    check("the renderer drew no ink where nothing is", inked(box().width - 3, box().height - 3) > 200, inked(box().width - 3, box().height - 3));

    const start = lines(), startRevision = shell.state().revision;
    type("hip_girth", "1000");
    check("a measurement field changes the value through the bus", shown("hip_girth") === 1000 && shell.state().revision !== startRevision, shell.state().status);
    check("the drawing follows the measurement", lines() !== start);
    const edited = lines();
    click("undo");
    check("Undo returns the drawing exactly", lines() === start && shown("hip_girth") === 980);
    click("redo");
    check("Redo returns the edit exactly", lines() === edited && shown("hip_girth") === 1000);
    type("sa_side", "-10");
    check("a value the draft cannot draw is refused, and says why", shell.state().failed && /negative allowance/.test(document.getElementById("status").textContent), document.getElementById("status").textContent);
    check("the refused value is taken back", shown("sa_side") === 10);

    const before = cuts()[0].slice(0, 2).map((value) => value / units());
    const [cx, cy] = [400, 300];
    const wheel = (deltaY) => overlay.dispatchEvent(new WheelEvent("wheel", {clientX: box().left + cx, clientY: box().top + cy, deltaY, bubbles: true, cancelable: true}));
    const zoomStart = lines();
    wheel(-100);
    const after = cuts()[0].slice(0, 2).map((value) => value / units());
    check("the wheel zooms the drawing", lines() !== zoomStart && Math.hypot(after[0] - before[0], after[1] - before[1]) > 1);
    wheel(100);
    check("the wheel back returns exactly", lines() === zoomStart);

    const dragStart = cuts()[0].slice(0, 2).map((value) => value / units());
    pointer("pointerdown", 500, 400);
    pointer("pointermove", 560, 430);
    pointer("pointermove", 620, 470);
    pointer("pointerup", 620, 470);
    const dragEnd = cuts()[0].slice(0, 2).map((value) => value / units());
    check("a drag moves the drawing by exactly the drag", Math.abs(dragEnd[0] - dragStart[0] - 120) < 0.01 && Math.abs(dragEnd[1] - dragStart[1] - 70) < 0.01, [dragEnd[0] - dragStart[0], dragEnd[1] - dragStart[1]]);
    click("fit");

    const [px, py] = longest(cuts()[2]);
    pointer("pointerdown", Math.round(px), Math.round(py));
    pointer("pointerup", Math.round(px), Math.round(py));
    const selected = shell.state().selected;
    check("a click picks the front's fold, which measures true", selected && selected.piece === "skirt_front" && selected.length_um === 660000, JSON.stringify(selected && {piece: selected.piece, edge: selected.edge, length_um: selected.length_um}));
    const context = shell.snapshot().getContext("2d", {willReadFrequently: true});
    const pixel = context.getImageData(Math.round(px), Math.round(py), 1, 1).data;
    check("the picked edge is highlighted on screen", pixel[0] > 200 && pixel[2] < 100, Array.from(pixel));
    check("the status line says what was picked", /skirt_front/.test(document.getElementById("selected").textContent), document.getElementById("selected").textContent);

    click("save");
    check("Save commits", /saved/.test(document.getElementById("status").textContent), document.getElementById("status").textContent);
    type("waist_to_hem", "700");
    click("reopen");
    check("Reopen returns what was saved", shown("waist_to_hem") === 620 && shown("hip_girth") === 1000, document.getElementById("status").textContent);

    const files = {};
    for (const [id, name] of [["export-canonical", "pattern.json"], ["export-dxf", "pattern.dxf"], ["export-a3", "pattern-a3.pdf"], ["export-a4", "pattern-a4.pdf"]]) {
      click(id);
      const file = shell.lastFile();
      check("the " + id + " button writes a file", file && file.path.endsWith(name), document.getElementById("status").textContent);
      if (file) files[file.path] = file.hex;
    }
    // A0 fits only at the session's own measurements, so here it may refuse; what it must not do is shrink.
    click("export-a0");
    check("the A0 button either writes or says it does not fit", shell.state().failed ? /never rescaled/.test(shell.state().status) : /exported/.test(shell.state().status), shell.state().status);
    if (!shell.state().failed) files[shell.lastFile().path] = shell.lastFile().hex;

    await report({results, files, recording: shell.recording(), renderer: shell.renderer, picture: shell.snapshot().toDataURL("image/png")});
  } catch (error) {
    await report({failure: String(error && error.stack || error)});
  }
})();
