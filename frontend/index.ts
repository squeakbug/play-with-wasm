import './index.css';
import init, {WorldVM, create_world, Tool, create_default_brush} from "../pkg"

// let brush = Brush().

async function main() {
    let exports = await init();

    const bg_canvas = <HTMLCanvasElement>document.getElementById("bg-canvas");
    const gp_canvas = <HTMLCanvasElement>document.getElementById("gp-canvas");
    const ui_canvas = <HTMLCanvasElement>document.getElementById("ui-canvas");
    let world = <WorldVM>create_world(ui_canvas, gp_canvas, bg_canvas);

    bg_canvas.width = 1080;
    bg_canvas.height = 1080;

    let animationId: number | null = null;

    const renderLoop = () => {
        world.poll()
        world.render()
        animationId = requestAnimationFrame(renderLoop);
    };

    const isPaused = () => {
        return animationId === null;
    };

    const play = () => {
        playPauseButton.textContent = "⏸";
        renderLoop();
        world.resume_simultaion();
    };

    const pause = () => {
        playPauseButton.textContent = "▶";
        if (animationId == null) return ;
        cancelAnimationFrame(animationId);
        animationId = null;
        world.pause_simulation();
    };

    const playPauseButton = <HTMLButtonElement>document.getElementById("play-pause");
    playPauseButton.addEventListener("click", event => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    });

    const clearButton = <HTMLButtonElement>document.getElementById("reset");
    clearButton.addEventListener("click", event => {
        world.reset_world()
    });

    var slider = <HTMLInputElement>document.getElementById("brush-size-slider");
    slider.oninput = function() {
        // world.set_brush_size(+slider.value);
    } 

    const id_btn_to_cell_type = {
        "delete": Tool.Delete,
        "acid": Tool.Acid,
        "fire": Tool.Fire,
        "gunpowder": Tool.Gunpowder,
        "lava": Tool.Lava,
        "oil": Tool.Oil,
        "propane": Tool.Propane,
        "rock": Tool.Rock,
        "sand": Tool.Sand,
        "vapor": Tool.Vapor,
        "water": Tool.Water,
        "wood": Tool.Wood,
    }

    Object.entries(id_btn_to_cell_type).forEach(entry => {
        const btn = <HTMLButtonElement>document.getElementById(entry[0]);
        btn.addEventListener("click", event => {
            // brush.value = ...
            // world.set_brush(brush)
        });
    });    

    gp_canvas.addEventListener("click", event => {
        world.tap_on_grid(event.clientY, event.clientX);
    });

    ui_canvas.addEventListener("click", event => {
        world.tap_on_grid(event.clientY, event.clientX);
    });
}

main()
