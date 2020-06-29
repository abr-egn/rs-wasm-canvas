import * as React from "react";
import * as ReactDOM from "react-dom";

import {Index} from "../tsx/index";
import * as wasm from "../wasm";

function main() {
    initCanvas();
    renderReact();
    wasm.greet();
}

function initCanvas() {
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
}

function renderReact() {
    let element = React.createElement(Index, {}, null);
    ReactDOM.render(element, document.getElementById("root"));
}

main();