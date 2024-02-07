import init, {WorldVM, Cell, create_world} from "j4ffallingsand";

const cell_type_color_map : any= {
    0:  "#ffffff",
    1:  "#555555",
    2:  "#555555",
    3:  "#555555",
    4:  "#555555",
    5:  "#555555",
    6:  "#555555",
    7:  "#555555",
    8:  "#555555",
    9:  "#555555",
    10: "#555555",
    11: "#555555",
}

function render_cell_on_canvas() {
    
}

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
        world.render_cell_on_canvas(canvas)
        drawGrid()
        animationId = requestAnimationFrame(renderLoop);
    };

    const isPaused = () => {
        return animationId === null;
    };

    const play = () => {
        playPauseButton.textContent = "⏸";
        renderLoop();
    };

    const pause = () => {
        playPauseButton.textContent = "▶";
        if (animationId == null) return ;
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    const playPauseButton = <HTMLButtonElement>document.getElementById("play-pause");
    playPauseButton.addEventListener("click", event => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
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
