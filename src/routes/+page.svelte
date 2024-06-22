

<script lang="ts">
    import { onMount } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'

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
    let note = '';
    let notes = [];

    function addNote() {
        notes.push(note);
        note = '';
    }
    // initialize constants from backend
    async function init() {
        ROWS = await invoke('get_rows')
        COLS = await invoke('get_cols')
        ROWS -= 1
        COLS -= 1
        //GAP = 60 - ROWS;
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
        let toRemove: number[][] = await invoke('handle_move', { x, y, color })

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
        console.log!(e.key)
        switch (e.key) {
            case 'ArrowLeft':
                await undo()
                break
            case 'ArrowRight':
                await redo()
                break
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
    function handleResize() {
        let rect = board.getBoundingClientRect();
        boardX = rect.left;
        boardY = rect.top;
    }
</script>

<div class="grid grid-rows-[100px_auto] bg-slate-500 text-white">
    <div>
        <div>Header</div>
    </div>
    <div class="grid grid-cols-[20%_60%_20%]">
        <div style="display: flex; justify-content: space-between;">
            <svg class = "hover-effect" style = "margin-right: 10px;" width="50%" height="50" role="button" tabindex="-1" on:click={undo} on:keydown={() => {}}>
                <rect
                        style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                        id="rect1"
                        width="100%"
                        height="50"
                        rx="10"
                        ry="10"
                />
                <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">Undo</text>
            </svg>
           <svg class = "hover-effect" style = "margin-left: 20 px" width="50%" height="50" role="button" tabindex="-1" on:click={redo} on:keydown={() => {}}>
             <rect
                    style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="rect1"
                    width="100%"
                    height="50"
                    rx="10"
                    ry="10"
             />
               <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">Redo</text>
           </svg>
        </div>


        <div class="relative">
            <canvas bind:this={board} class="absolute top-1/2 left-1/2 transform -translate-x-1/2" {width} {height}></canvas>
            <canvas bind:this={hover} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
            <canvas bind:this={pieces} on:mousemove={hovering} on:click={placing} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
        </div>
        <div style="display: flex; justify-content: space-between;">


        <svg class = "hover-effect" style = "margin-right: 10px;" width="25%" height="50" role="button" tabindex="-1" on:click={playBlack} on:keydown={() => {}}>
            <rect
                    style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="rect1"
                    width="100%"
                    height="50"
                    rx="10"
                    ry="10"
            />
            <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">PlayBlack</text>
        </svg>



        <svg class = "hover-effect" style = "margin-right: 10px;" width="25%" height="50" role="button" tabindex="-1" on:click={playWhite} on:keydown={() => {}}>
            <rect
                    style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="rect1"
                    width="100%"
                    height="50"
                    rx="10"
                    ry="10"
            />
            <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">playWhite</text>
        </svg>


        <svg class = "hover-effect" style = "margin-right: 10px;" width="25%" height="50" role="button" tabindex="-1" on:click={setBlack} on:keydown={() => {}}>
            <rect
                    style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="rect1"
                    width="100%"
                    height="50"
                    rx="10"
                    ry="10"
            />
            <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">SetBlack</text>
        </svg>
            <svg class = "hover-effect" style = "margin-right: 10px;" width="25%" height="50" role="button" tabindex="-1" on:click={setWhite} on:keydown={() => {}}>
            <rect
                    style="fill:#000000;fill-opacity:30%;stroke:none;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1"
                    id="rect1"
                    width="100%"
                    height="50"
                    rx="10"
                    ry="10"
            />
            <text x="50%" y="20px" dominant-baseline="middle" text-anchor="middle" fill="#fffbfb">SetWhite</text>
        </svg>
        </div>
    </div>
</div>



<div class="note-section">
    <h2>Note Taking Section</h2>
    <textarea bind:value={note} placeholder="Write your note here..."></textarea>
    <button on:click={addNote}>Add Note</button>
    <div class="notes-display">
        <h3>Your Notes:</h3>
        {#each notes as note, i}
            <p>{i + 1}. {note}</p>
        {/each}
    </div>
</div>

<style>
    @import './styles.css';
    .note-section {
        width: 20%;
        padding: 20px;
        box-sizing: border-box;
    }
    .notes-display {
        margin-top: 20px;
    }
</style>



<svelte:window on:keydown|preventDefault={handleKey} />

