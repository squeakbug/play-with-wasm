import './index.css';
import init, {WorldVM, CellType, create_world, create_default_brush} from "../pkg"

// let brush = Brush().

async function main() {
//    let exports = await init();

    const brush = create_default_brush();
    console.log(brush);

    const bg_canvas = <HTMLCanvasElement>document.getElementById("bg-canvas");
    const gp_canvas = <HTMLCanvasElement>document.getElementById("gp-canvas");
    const ui_canvas = <HTMLCanvasElement>document.getElementById("ui-canvas");
    let world = <WorldVM>create_world(ui_canvas, gp_canvas, bg_canvas);

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
        "acid": CellType.Acid,
        "fire": CellType.Fire,
        "gunpowder": CellType.Gunpowder,
        "lava": CellType.Lava,
        "oil": CellType.Oil,
        "propane": CellType.Propane,
        "rock": CellType.Rock,
        "sand": CellType.Sand,
        "vapor": CellType.Vapor,
        "water": CellType.Water,
        "wood": CellType.Wood,
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
