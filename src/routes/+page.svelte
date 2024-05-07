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
        console.log!(boardX)
        console.log!(boardY)

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
        pieceColor = pieceColor === 'black' ? 'white' : 'black'
    }
</script>

<div class="grid grid-rows-2">
    <div>
        <div>Header</div>
    </div>
    <div class="grid grid-cols-[2%_auto_2%]">
        <div>Left</div>
        <div class="relative">
            <canvas bind:this={board} class="absolute top-1/2 left-1/2 transform -translate-x-1/2" {width} {height}></canvas>
            <canvas bind:this={hover} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
            <canvas bind:this={pieces} on:mousemove={hovering} on:click={placing} {width} {height} class="absolute top-1/2 left-1/2 transform -translate-x-1/2"></canvas>
        </div>
        <div>Right</div>
    </div>
</div>

