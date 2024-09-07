<script lang="ts">
    import { onMount } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';
    import { Save, FileText, Rewind } from 'lucide-svelte';

    let board: HTMLCanvasElement
    let hover: HTMLCanvasElement
    let pieces: HTMLCanvasElement
    let ROWS: number
    let COLS: number
    let GAP: number
    let width: number
    let height: number
    let boardX: number
    let boardY: number
    let ctxBoard: CanvasRenderingContext2D
    let ctxHover: CanvasRenderingContext2D
    let ctxPieces: CanvasRenderingContext2D
    let pieceColor: string = 'black'
    let isInit: boolean = false
    let isPlay: boolean = true

    // initialize constants from backend
    async function init() {
        ROWS = await invoke('get_rows')
        COLS = await invoke('get_cols')
        ROWS -= 1
        COLS -= 1
        GAP = 40;
        width = (COLS + 2) * GAP
        height = (ROWS + 2) * GAP
    }

    onMount(async () => {
        await init()
        window.addEventListener("resize", handleResize); {
            console.log("Window resized to: " + window.innerWidth + "x" + window.innerHeight);
        }
        ctxBoard = board.getContext('2d')
        ctxHover = hover.getContext('2d')
        ctxHover.globalAlpha = 0.3
        ctxPieces = pieces.getContext('2d')

        // draw board
        ctxBoard.fillStyle = '#dcae6b'
        ctxBoard.fillRect(0, 0, width, height)
        ctxBoard.beginPath()

        // draw board lines
        let x: number = GAP
        let y: number = GAP 
        while (x < width || y < height) {
            ctxBoard.moveTo(x, GAP)
            ctxBoard.lineTo(x, GAP + GAP * ROWS)
            x += GAP
            ctxBoard.moveTo(GAP, y)
            ctxBoard.lineTo(GAP + GAP * COLS, y)
            y += GAP
        }
        ctxBoard.stroke()

        // draw star points
        ctxBoard.fillStyle = '#000000'
        let points: number[][] = [[3, 3], [9, 3], [15, 3], [3, 9], [9, 9], [15, 9], [3, 15], [9, 15], [15, 15]]
        for (let i = 0; i < points.length; i++) {
            let [x, y] = points[i]
            ctxBoard.beginPath()
            ctxBoard.arc(GAP * x + GAP, GAP * y + GAP, 5, 0, 2 * Math.PI)
            ctxBoard.fill()
        }

        // reset board
        await invoke('reset')

        // initialize states
        let numStates: number = await invoke('init_states')
        for (let i = 0; i < numStates; i++) {
            const newState = { id, name: `State ${savedStates.length + 1}` }
            savedStates = [...savedStates, newState]
            id += 1
        }
    })

    let savedStates = []
    let id = 0

    async function saveCurrentState() {
        const newState = { id, name: `State ${savedStates.length + 1}` }
        savedStates = [...savedStates, newState]
        id += 1
        await invoke('save_state')
    }

    async function loadState(stateIdx: number) {
        let pieces: number[][] = await invoke('revert_state', { stateIdx })

        // clear board
        ctxPieces.clearRect(0, 0, width, height)

        // add pieces
        for (let i = 0; i < pieces.length; i++) {
            let [x, y, color] = pieces[i]
            drawStone(ctxPieces, GAP * y + GAP, GAP * x + GAP, GAP / 2 - 2, color === 1 ? 'black' : 'white')
        }
    }

    // draw stone on board
    function drawStone(ctx: CanvasRenderingContext2D, x: number, y: number, radius: number, color: string) {
        // Create gradient
        const gradient = ctx.createRadialGradient(
                x - radius / 3, y - radius / 3, radius / 8,
                x, y, radius
                );
        gradient.addColorStop(0, color === 'black' ? '#505050' : '#FFFFFF');
        gradient.addColorStop(1, color === 'black' ? '#000000' : '#D4D4D4');

        // Draw shadow
        ctx.beginPath();
        ctx.arc(x + 2, y + 2, radius, 0, 2 * Math.PI);
        ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
        ctx.fill();

        // Draw stone
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, 2 * Math.PI);
        ctx.fillStyle = gradient;
        ctx.fill();

        // Add slight border
        ctx.strokeStyle = color === 'black' ? '#303030' : '#A0A0A0';
        ctx.lineWidth = 1;
        ctx.stroke();
    }

    // show transparent shadow of piece when hovering
    function hovering(e: MouseEvent) {
        // initialize board coordinates
        if (!isInit) {
            let rect = board.getBoundingClientRect()
            boardX = rect.left
            boardY = rect.top
            isInit = true
        }

        // set closest coordinates for piece
        let closestX: number = Math.round((e.clientX - boardX) / GAP) * GAP
        let closestY: number = Math.round((e.clientY - boardY) / GAP) * GAP

        // cap closestX and closestY
        closestX = Math.min(GAP + GAP * COLS, Math.max(GAP, closestX))
        closestY = Math.min(GAP + GAP * ROWS, Math.max(GAP, closestY))

        // draw shadow
        ctxHover.clearRect(0, 0, width, height)
        drawStone(ctxHover, closestX, closestY, GAP / 2 - 2, pieceColor)
    }

    // place piece on board when clicked
    async function placing(e: MouseEvent) {
        // set closest coordinates for piece
        let closestX: number = Math.round((e.clientX - boardX) / GAP) * GAP
        let closestY: number = Math.round((e.clientY - boardY) / GAP) * GAP

        // cap closestX and closestY
        closestX = Math.min(GAP + GAP * COLS, Math.max(GAP, closestX))
        closestY = Math.min(GAP + GAP * ROWS, Math.max(GAP, closestY))

        // check if move is legal
        let x: number = Math.floor((closestY - GAP) / GAP)
        let y: number = Math.floor((closestX - GAP) / GAP)
        let color: number = pieceColor === 'black' ? 1 : 2
        let isValid: boolean = await invoke('validate', { x, y, color })
        if (!isValid) return
        let toRemove: number[][] = await invoke('tauri_move', { x, y, color })

        // remove pieces
        for (let i = 0; i < toRemove.length; i++) {
            let [y, x] = toRemove[i]
            ctxPieces.clearRect(GAP * x + GAP / 2, GAP * y + GAP / 2, GAP, GAP)
        }

        // draw piece
        drawStone(ctxPieces, closestX, closestY, GAP / 2 - 2, pieceColor)

        // set piece color
        if (isPlay) {
            pieceColor = pieceColor === 'black' ? 'white' : 'black'
        }
    }

    // handle user left arrow to undo
    async function undo() {
        let change: number[][][] = await invoke('handle_undo')

        // remove pieces
        for (let i = 0; i < change[1].length; i++) {
            let [y, x] = change[1][i]
            ctxPieces.clearRect(GAP * x + GAP / 2, GAP * y + GAP / 2, GAP, GAP)
        }

        // add pieces
        for (let i = 0; i < change[0].length; i++) {
            let [y, x, color] = change[0][i]
            drawStone(ctxPieces, GAP * x + GAP, GAP * y + GAP, GAP / 2 - 2, color === 1 ? 'black' : 'white')
        }
    }

    // handle user right arrow to redo
    async function redo() {
        let change: number[][][] = await invoke('handle_redo')

        // remove pieces
        for (let i = 0; i < change[1].length; i++) {
            let [y, x] = change[1][i]
            ctxPieces.clearRect(GAP * x + GAP / 2, GAP * y + GAP / 2, GAP, GAP)
        }

        // add pieces
        for (let i = 0; i < change[0].length; i++) {
            let [y, x, color] = change[0][i]
            drawStone(ctxPieces, GAP * x + GAP, GAP * y + GAP, GAP / 2 - 2, color === 1 ? 'black' : 'white')
        }
    }

    // handle key presses
    async function handleKey(e: KeyboardEvent) {
        switch (e.key) {
            case 'ArrowLeft':
                await undo()
                break
            case 'ArrowRight':
                await redo()
                break
        }
    }

    // load SGF file
    async function loadSGF() {
        // open a selection dialog for image files
        const file: string | string[] = await open({
            multiple: false,
            filters: [{
                name: 'SGF',
                extensions: ['sgf']
            }]
        });

        // handle selection
        if (file === null) {
            return;
        } else if (typeof file === 'string') {
            let pieces: number[][] = await invoke('from_sgf', { file })

            // clear and reset board
            ctxPieces.clearRect(0, 0, width, height)
            await invoke('reset')

            // add pieces
            for (let i = 0; i < pieces.length; i++) {
                let [x, y, color] = pieces[i]
                drawStone(ctxPieces, GAP * y + GAP, GAP * x + GAP, GAP / 2 - 2, color === 1 ? 'black' : 'white')
            }
        }
    }

    // change to black to play
    function playBlack() {
        pieceColor = 'black'
        isPlay = true
    }

    // change to white to play
    function playWhite() {
        pieceColor = 'white'
        isPlay = true
    }

    // change to set black stones layout
    function setBlack() {
        pieceColor = 'black'
        isPlay = false
    }

    // change to set white stones layout
    function setWhite() {
        pieceColor = 'white'
        isPlay = false
    }

    // handle resizing of window
    function handleResize() {
        let rect = board.getBoundingClientRect();
        boardX = rect.left;
        boardY = rect.top;
    }
</script>


<div class="grid grid-rows-[100px_200px_auto]">
    <div class="text-center p-6 bg-slate-500 text-white">
        <div class="text-3xl">Gobase</div>
    </div>
    <div class="grid grid-cols-[10%_auto_10%] bg-slate-500 text-white">
        <div class="grid grid-rows-[200px_auto]">
            <div class="p-4">
                <svg width="50px" height="50px" role="button" fill="none" tabindex="-1" on:click={loadSGF} on:keydown={() => {}}>
                    <path 
                        d="M13 3H8.2C7.0799 3 6.51984 3 6.09202 3.21799C5.71569 3.40973 5.40973 3.71569 5.21799 4.09202C5 4.51984 5 5.0799 5 6.2V17.8C5 18.9201 5 19.4802 5.21799 19.908C5.40973 20.2843 5.71569 20.5903 6.09202 20.782C6.51984 21 7.0799 21 8.2 21H12M13 3L19 9M13 3V7.4C13 7.96005 13 8.24008 13.109 8.45399C13.2049 8.64215 13.3578 8.79513 13.546 8.89101C13.7599 9 14.0399 9 14.6 9H19M19 9V12M17 19H21M19 17V21"
                        stroke="#FFFFFF" 
                        stroke-linecap="round" 
                        stroke-linejoin="round"/>
                </svg>
            </div>
            <div class="p-4">
                <button on:click={saveCurrentState} class="bg-blue-500 hover:bg-blue-600 text-white p-2 rounded flex items-center justify-center">
                    <Save class="mr-2" size={16} />
                    Save State
                </button>
                <div class="bg-gray-700 p-2 mt-2 rounded">
                <div class="flex items-center justify-between mb-2">
                <span>Saved States</span>
                <FileText size={16} />
                </div>
                    {#each savedStates as state}
                        <button
                        on:click={() => loadState(state.id)}
                        class="w-full bg-gray-600 hover:bg-gray-500 text-white p-2 rounded mb-1 flex items-center justify-start"
                        >
                            <Rewind class="mr-2" size={16} />
                            {state.name}
                        </button>
                    {/each}
                </div>
            </div>
        </div>
        <div class="relative">
            <canvas bind:this={board} class="absolute left-1/2 transform -translate-x-1/2" {width} {height}></canvas>
            <canvas bind:this={hover} {width} {height} class="absolute left-1/2 transform -translate-x-1/2"></canvas>
            <canvas bind:this={pieces} on:mousemove={hovering} on:click={placing} {width} {height} class="absolute left-1/2 transform -translate-x-1/2"></canvas>
        </div>
        <div>
            <svg width="50px" height="40px" role="button" tabindex="-1" on:click={playBlack} on:keydown={() => {}}>
                <circle
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="path1"
                    cx="20"
                    cy="20"
                    r="10" />
            </svg>
            <svg width="50px" height="40px" role="button" tabindex="-1" on:click={playWhite} on:keydown={() => {}}>
                <circle
                    style="fill:#fffbfb;fill-opacity:1;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="path1"
                    cx="20"
                    cy="20"
                    r="10" />
            </svg>
            <svg width="50px" height="40px" role="button" tabindex="-1" on:click={setBlack} on:keydown={() => {}}>
                <circle
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="path1"
                    cx="20"
                    cy="20"
                    r="10" />
                <rect
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226"
                    id="rect1"
                    width="0.012675161"
                    height="6.5395594"
                    x="33.037266"
                    y="7.2474785" />
                <rect
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226"
                    id="rect1-5"
                    width="0.012675161"
                    height="6.5395594"
                    x="10.456588"
                    y="-36.335133"
                    transform="rotate(90)" />
            </svg>
            <svg width="50px" height="40px" role="button" tabindex="-1" on:click={setWhite} on:keydown={() => {}}>
                <circle
                style="fill:#fffbfb;fill-opacity:1;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                id="path1"
                cx="20"
                cy="20"
                r="10" />
                <rect
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226"
                    id="rect1"
                    width="0.012675161"
                    height="6.5395594"
                    x="33.037266"
                    y="7.2474785" />
                <rect
                    style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226"
                    id="rect1-5"
                    width="0.012675161"
                    height="6.5395594"
                    x="10.456588"
                    y="-36.335133"
                    transform="rotate(90)" />
            </svg>
        </div>
    </div>
</div>

<svelte:window on:keydown|preventDefault={handleKey} />

