import * as React from "react";
import * as ReactDOM from "react-dom";

import { Index } from "../tsx/index";
import * as wasm from "../wasm";
import { memory } from "../js/mem_hack";

function main() {
    const canvas = initCanvas();
    renderReact();

    const data = new wasm.Data(performance.now());
    console.info(data.count(), data.stride(), data.x_offset(), data.y_offset());
    window.requestAnimationFrame((ts) => drawFrame(canvas, data, ts));
}

function initCanvas(): HTMLCanvasElement {
    const canvas = document.getElementById("render") as HTMLCanvasElement;
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    window.onresize = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    };
    canvas.addEventListener('keyup', (e) => {
        console.info("Key pressed: ", e.code);
    });
    canvas.focus();
    return canvas;
}

function renderReact() {
    let element = React.createElement(Index, {}, null);
    ReactDOM.render(element, document.getElementById("root"));
}

function drawFrame(canvas: HTMLCanvasElement, data: wasm.Data, ts: number) {
    data.update(ts);

    const count = data.count();
    const stride = data.stride();
    const xoff = data.x_offset();
    const yoff = data.y_offset();

    canvas.width = canvas.width;  // clear
    const ctx = canvas.getContext("2d")!;
    ctx.fillStyle = "#FFFFFF";
    ctx.translate(canvas.width/2, canvas.height/2);
    for (let ix = 0; ix < count; ix++) {
        const offset = ix * stride;
        const view = new DataView(memory.buffer, data.points() + offset, stride);
        const x = view.getFloat32(xoff, true);
        const y = view.getFloat32(yoff, true);
        ctx.fillRect(x, y, 5, 5);
    }

    window.requestAnimationFrame((ts) => drawFrame(canvas, data, ts));
}

main();