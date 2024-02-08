import init, {WorldVM, Cell, CellType, create_world} from "j4ffallingsand";

async function main() {
    let exports = await init()

    let height = 100
    let width = 100
    let world = <WorldVM>create_world()

    const CELL_SIZE = 5;
    const canvas = <HTMLCanvasElement>document.getElementById("space");
    const ctx = <CanvasRenderingContext2D>canvas.getContext('2d');
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    let animationId: number | null = null;

    const renderLoop = () => {
        world.tick()
        drawGrid()
        world.render_cell_on_canvas(canvas)
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

    var slider = <HTMLInputElement>document.getElementById("brush-size-slider");
    slider.oninput = function() {
        world.set_brush_size(slider.value);
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
            world.set_cell_type(entry[1])
        });
    });    

    canvas.addEventListener("click", event => {
        const boundingRect = canvas.getBoundingClientRect();
        
        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;
        
        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;
        
        const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
        
        world.tap_on_grid(row, col);
        
        drawGrid();
        world.render_cell_on_canvas(canvas);
    });

    const drawGrid = () => {
        ctx.beginPath();
        ctx.strokeStyle = "#eeeeee";

        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
        }

        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
        }

        ctx.stroke();
    };
}

main()
