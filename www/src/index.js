import {set_panic_hook} from "plop-mario-wasm";
import {render} from "./render";

(function () {
    set_panic_hook();

    const canvas = document.createElement("canvas");
    canvas.id = "plop-mario";
    canvas.width = 640;
    canvas.height = 640;
    document.body.appendChild(canvas);

    render(canvas);
})();


