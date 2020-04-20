import {attach_canvas} from "plop-mario-wasm";

(function () {
    const canvas = document.createElement("canvas");
    canvas.id = "plop-mario";
    canvas.width = 640;
    canvas.height = 480;
    document.body.appendChild(canvas);

    attach_canvas(canvas.id);

    // const ctx = canvas.getContext("2d");
    // ctx.fillStyle = "rgba(0,100,0,.5)"
    // ctx.fillRect(0, 0, 640, 480);
})();


