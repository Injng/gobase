<script lang="ts">
    import { onMount } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';

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

        // reset board
        await invoke('reset')
    })

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
        ctxHover.beginPath()
        ctxHover.arc(closestX, closestY, GAP / 2, 0, 2 * Math.PI)
        ctxHover.fillStyle = pieceColor
        ctxHover.fill()
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
        ctxPieces.beginPath()
        ctxPieces.arc(closestX, closestY, GAP / 2, 0, 2 * Math.PI)
        ctxPieces.fillStyle = pieceColor
        ctxPieces.fill()

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
            ctxPieces.beginPath()
            ctxPieces.arc(GAP * x + GAP, GAP * y + GAP, GAP / 2, 0, 2 * Math.PI)
            ctxPieces.fillStyle = color === 1 ? 'black' : 'white'
            ctxPieces.fill()
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
            ctxPieces.beginPath()
            ctxPieces.arc(GAP * x + GAP, GAP * y + GAP, GAP / 2, 0, 2 * Math.PI)
            ctxPieces.fillStyle = color === 1 ? 'black' : 'white'
            ctxPieces.fill()
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
        // Open a selection dialog for image files
        const file: string | string[] = await open({
            multiple: false,
            filters: [{
                name: 'SGF',
                extensions: ['sgf']
            }]
        });

        // Handle selection
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
                ctxPieces.beginPath()
                ctxPieces.arc(GAP * x + GAP, GAP * y + GAP, GAP / 2, 0, 2 * Math.PI)
                ctxPieces.fillStyle = color === 1 ? 'black' : 'white'
                ctxPieces.fill()
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


<div class="grid grid-rows-[100px_auto] bg-slate-500 text-white">
    <div class="text-center p-6">
        <div class="text-3xl">Gobase</div>
    </div>
    <div class="grid grid-cols-[3%_auto_3%]">
        <div class="p-4">
            <svg width="50px" height="50px" role="button" fill="none" tabindex="-1" on:click={loadSGF} on:keydown={() => {}}>
                <path 
                    d="M13 3H8.2C7.0799 3 6.51984 3 6.09202 3.21799C5.71569 3.40973 5.40973 3.71569 5.21799 4.09202C5 4.51984 5 5.0799 5 6.2V17.8C5 18.9201 5 19.4802 5.21799 19.908C5.40973 20.2843 5.71569 20.5903 6.09202 20.782C6.51984 21 7.0799 21 8.2 21H12M13 3L19 9M13 3V7.4C13 7.96005 13 8.24008 13.109 8.45399C13.2049 8.64215 13.3578 8.79513 13.546 8.89101C13.7599 9 14.0399 9 14.6 9H19M19 9V12M17 19H21M19 17V21"
                    stroke="#FFFFFF" 
                    stroke-linecap="round" 
                    stroke-linejoin="round"/>
            </svg>
        </div>
        <div class="relative">
            <canvas bind:this={board} class="absolute top-1/2 left-1/2 transform -translate-x-1/2" {width} {height}></canvas>
            <canvas bind:this={hover} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
            <canvas bind:this={pieces} on:mousemove={hovering} on:click={placing} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
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

