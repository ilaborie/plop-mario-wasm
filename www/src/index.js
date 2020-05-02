import {set_panic_hook, run} from "plop-mario-wasm";
import {ConsoleAppender, defaultConfig, Logger, LogLevel} from "plop-logger";

(function () {
    set_panic_hook();

    const start = () => {
        window.removeEventListener('click', start);
        run();
    };
    window.addEventListener('click', start);

})();


